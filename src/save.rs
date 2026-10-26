use crate::binary::*;
use anyhow::{bail, ensure, Context, Result};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub next: u32,
    pub child: u32,
    pub offset: usize,
    pub size: usize,
}
#[derive(Clone, Debug)]
pub struct Fact {
    pub hash: u32,
    pub value: u32,
    pub offset: usize,
    pub name: String,
}
#[derive(Clone, Debug)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub inventory: u64,
    pub offset: usize,
    pub quantity: Option<usize>,
    pub flags: usize,
}
#[derive(Clone)]
pub struct Save {
    pub original: Vec<u8>,
    pub data: Vec<u8>,
    pub nodes: Vec<Node>,
    pub table: usize,
    pub data_start: usize,
    pub version: u32,
    pub game: u32,
    pub source: Option<PathBuf>,
    pub metadata: Option<serde_json::Value>,
}
pub fn tdb(s: &str) -> u64 {
    crc32fast::hash(s.as_bytes()) as u64 | ((s.len() as u64) << 32)
}
pub fn fact_hash(s: &str) -> u32 {
    s.bytes()
        .fold(2166136261, |h, b| (h ^ b as u32).wrapping_mul(16777619))
}
impl Save {
    pub fn load(path: &Path) -> Result<Self> {
        let path = if path.is_dir() {
            path.join("sav.dat")
        } else {
            path.to_owned()
        };
        let mut s = Self::parse(fs::read(&path).context("Cannot read sav.dat")?)?;
        s.metadata = fs::read(path.with_file_name("metadata.9.json"))
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok());
        s.source = Some(path);
        Ok(s)
    }
    pub fn parse(original: Vec<u8>) -> Result<Self> {
        ensure!(
            original.len() >= 40 && &original[..4] == b"VASC",
            "Not a Cyberpunk save"
        );
        let version = u32_at(&original, 4)?;
        let game = u32_at(&original, 8)?;
        ensure!(version==277 && game==2500,"Unsupported save/game version {version}/{game}; this build writes only the verified 2.5 layout (277/2500)");
        let mut h = Reader::new(&original, 12);
        h.string()?;
        h.bytes(12)?;
        let table = h.pos;
        ensure!(h.bytes(4)? == b"FZLC", "Missing compression table");
        let count = h.u32()? as usize;
        ensure!(count > 0 && count <= 4096, "Invalid chunk count");
        let mut chunks = Vec::new();
        for _ in 0..count {
            chunks.push((h.u32()? as usize, h.u32()? as usize, h.u32()? as usize));
        }
        let data_start = chunks[0].0;
        ensure!(
            data_start >= h.pos && (data_start - table - 8).is_multiple_of(12),
            "Invalid chunk table"
        );
        let mut data = original
            .get(..data_start)
            .context("Truncated header")?
            .to_vec();
        let mut previous_end = data_start;
        for (o, c, d) in chunks {
            ensure!(
                o == previous_end && d <= 524288 && c <= 1048576,
                "Invalid chunk bounds"
            );
            let src = original
                .get(o..o.checked_add(c).context("Chunk overflow")?)
                .context("Truncated chunk")?;
            if src.starts_with(b"4ZLX") {
                ensure!(
                    src.len() >= 8 && u32_at(src, 4)? as usize == d,
                    "Chunk size mismatch"
                );
                data.extend(lz4_flex::block::decompress(&src[8..], d)?);
            } else {
                ensure!(c == d, "Invalid raw chunk");
                data.extend(src);
            }
            ensure!(data.len() < 512 * 1024 * 1024, "Save exceeds size limit");
            previous_end = o + c;
        }
        let footer = u32_at(&original, original.len() - 8)? as usize;
        ensure!(
            &original[original.len() - 4..] == b"ENOD",
            "Missing DONE footer"
        );
        ensure!(footer >= previous_end, "Overlapping footer");
        let mut r = Reader::new(&original, footer);
        ensure!(r.bytes(4)? == b"EDON", "Missing node table");
        let n = r.vlq()?;
        ensure!((0..1000000).contains(&n), "Invalid node count");
        let mut nodes = Vec::new();
        for _ in 0..n {
            let name = r.string()?;
            let next = r.u32()?;
            let child = r.u32()?;
            let offset = r.u32()? as usize;
            let size = r.u32()? as usize;
            ensure!(
                size >= 4
                    && offset >= data_start
                    && offset.checked_add(size).is_some_and(|e| e <= data.len()),
                "Node outside data: {name}"
            );
            nodes.push(Node {
                name,
                next,
                child,
                offset,
                size,
            });
        }
        ensure!(r.pos == original.len() - 8, "Unknown node footer data");
        for (i, n) in nodes.iter().enumerate() {
            ensure!(
                u32_at(&data, n.offset)? == i as u32,
                "Nonsequential node ID at {i}"
            );
            for link in [n.next, n.child] {
                ensure!(
                    link == u32::MAX || (link as usize) > i && (link as usize) < nodes.len(),
                    "Invalid node link at {i}"
                );
            }
        }
        Ok(Self {
            original,
            data,
            nodes,
            table,
            data_start,
            version,
            game,
            source: None,
            metadata: None,
        })
    }
    pub fn node(&self, name: &str) -> Result<&Node> {
        self.nodes
            .iter()
            .find(|n| n.name == name)
            .with_context(|| format!("Missing node {name}"))
    }
    pub fn replace_node(&mut self, name: &str, payload: Vec<u8>) -> Result<()> {
        let n = self.node(name)?.clone();
        ensure!(n.child == u32::MAX, "Cannot replace a node with children");
        let delta = payload.len() as isize - (n.size - 4) as isize;
        self.data.splice(n.offset + 4..n.offset + n.size, payload);
        for x in &mut self.nodes {
            if x.offset == n.offset {
                x.size = (x.size as isize + delta) as usize;
            } else if x.offset >= n.offset + n.size {
                x.offset = (x.offset as isize + delta) as usize;
            } else if x.offset < n.offset && x.offset + x.size >= n.offset + n.size {
                x.size = (x.size as isize + delta) as usize;
            }
        }
        Ok(())
    }
    pub fn facts(&self) -> Result<Vec<Fact>> {
        let names: HashMap<String, String> =
            serde_json::from_str(include_str!("../assets/facts.json"))?;
        let mut out = Vec::new();
        for node in self.nodes.iter().filter(|n| n.name == "FactsTable") {
            let mut r = Reader::new(&self.data, node.offset + 4);
            let count = r.vlq()?;
            ensure!(
                count >= 0 && count as usize <= node.size / 8,
                "Invalid facts table"
            );
            let mut hashes = Vec::new();
            for _ in 0..count {
                hashes.push(r.u32()?);
            }
            for hash in hashes {
                let offset = r.pos;
                let value = r.u32()?;
                out.push(Fact {
                    hash,
                    value,
                    offset,
                    name: names
                        .get(&hash.to_string())
                        .cloned()
                        .unwrap_or_else(|| format!("#{hash:08X}")),
                });
            }
        }
        Ok(out)
    }
    pub fn set_fact(&mut self, hash: u32, value: u32) -> Result<()> {
        let facts = self.facts()?;
        let matches: Vec<_> = facts.iter().filter(|f| f.hash == hash).collect();
        ensure!(matches.len() <= 1, "Ambiguous fact in multiple tables");
        if let Some(f) = matches.first() {
            return put32(&mut self.data, f.offset, value);
        }
        let n = self
            .nodes
            .iter()
            .find(|n| n.name == "FactsTable")
            .context("No facts table")?
            .clone();
        let mut r = Reader::new(&self.data, n.offset + 4);
        let count = r.vlq()? as usize;
        let mut hashes = Vec::new();
        for _ in 0..count {
            hashes.push(r.u32()?);
        }
        let mut pairs = Vec::new();
        for h in hashes {
            pairs.push((h, r.u32()?));
        }
        ensure!(r.pos == n.offset + n.size, "Unknown facts trailer");
        pairs.push((hash, value));
        pairs.sort_by_key(|p| p.0);
        let mut payload = vlq(pairs.len() as i32);
        for (h, _) in &pairs {
            payload.extend(h.to_le_bytes());
        }
        for (_, v) in &pairs {
            payload.extend(v.to_le_bytes());
        }
        // Replace this exact table; multiple tables may share the same name.
        let idx = self
            .nodes
            .iter()
            .position(|x| x.offset == n.offset)
            .unwrap();
        let old = self.nodes[idx].name.clone();
        self.nodes[idx].name = "__target_fact_table".into();
        let result = self.replace_node("__target_fact_table", payload);
        self.nodes[idx].name = old;
        result
    }
    pub fn items(&self) -> Result<Vec<Item>> {
        static NAMES: std::sync::OnceLock<HashMap<u64, String>> = std::sync::OnceLock::new();
        let known = NAMES.get_or_init(|| {
            serde_json::from_str::<Vec<String>>(include_str!("../assets/items.json"))
                .expect("Bundled item catalog is valid JSON")
                .into_iter()
                .map(|s| (tdb(&s), s))
                .collect()
        });
        let inventory = self.node("inventory")?;
        let mut r = Reader::new(&self.data, inventory.offset + 4);
        let count = r.u32()?;
        ensure!(count < 10000, "Invalid inventory count");
        let mut out = Vec::new();
        for _ in 0..count {
            let inv = r.u64()?;
            let cnt = r.u32()?;
            ensure!(cnt < 100000, "Invalid item count");
            for _ in 0..cnt {
                let header = r.bytes(16)?.to_vec();
                let off = r.pos;
                let node = self
                    .nodes
                    .iter()
                    .find(|n| n.offset == off && n.name == "itemData")
                    .context("Inventory item node missing")?;
                r.bytes(4)?;
                let item = r.bytes(16)?;
                ensure!(item == header, "Inventory header mismatch");
                let id = u64::from_le_bytes(item[..8].try_into()?);
                let seed = u32_at(item, 8)?;
                let structure = item[12];
                ensure!(structure <= 3, "Unknown item structure");
                let flags = r.pos;
                r.bytes(5)?;
                let quantity = if structure & 2 != 0 || (structure == 0 && seed == 2) {
                    let p = r.pos;
                    r.u32()?;
                    Some(p)
                } else {
                    None
                };
                out.push(Item {
                    id,
                    name: if id == tdb("Items.money") {
                        "Items.money / Eurodollars".into()
                    } else {
                        known
                            .get(&id)
                            .cloned()
                            .unwrap_or_else(|| format!("TDBID {id:010X}"))
                    },
                    inventory: inv,
                    offset: off,
                    quantity,
                    flags,
                });
                r.pos = node.offset + node.size;
            }
        }
        ensure!(
            r.pos <= inventory.offset + inventory.size,
            "Inventory exceeds node"
        );
        Ok(out)
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let original = Self::parse(self.original.clone())?;
        if self.data == original.data && self.nodes == original.nodes {
            return Ok(self.original.clone());
        }
        let mut out = self.data[..self.data_start].to_vec();
        let max = (self.data_start - self.table - 8) / 12;
        let chunks: Vec<_> = self.data[self.data_start..].chunks(262144).collect();
        ensure!(chunks.len() <= max, "Compression table full");
        out[self.table + 4..self.data_start].fill(0);
        put32(&mut out, self.table + 4, chunks.len() as u32)?;
        for (i, chunk) in chunks.into_iter().enumerate() {
            let compressed = lz4_flex::block::compress(chunk);
            let o = out.len();
            out.extend(b"4ZLX");
            out.extend((chunk.len() as u32).to_le_bytes());
            out.extend(&compressed);
            let p = self.table + 8 + i * 12;
            put32(&mut out, p, o as u32)?;
            put32(&mut out, p + 4, (compressed.len() + 8) as u32)?;
            put32(&mut out, p + 8, chunk.len() as u32)?;
        }
        let footer = out.len();
        out.extend(b"EDON");
        out.extend(vlq(self.nodes.len() as i32));
        for n in &self.nodes {
            out.extend(string(&n.name));
            out.extend(n.next.to_le_bytes());
            out.extend(n.child.to_le_bytes());
            out.extend((n.offset as u32).to_le_bytes());
            out.extend((n.size as u32).to_le_bytes());
        }
        out.extend((footer as u32).to_le_bytes());
        out.extend(b"ENOD");
        let check = Self::parse(out.clone())?;
        ensure!(
            check.data[self.data_start..] == self.data[self.data_start..]
                && check.nodes == self.nodes,
            "Verification failed"
        );
        Ok(out)
    }
    /// Export a complete copy. The source folder is never modified, and an existing destination is refused.
    pub fn export(&self, destination: &Path) -> Result<()> {
        let name = destination
            .file_name()
            .context("No folder name")?
            .to_string_lossy();
        self.export_named(destination, &name)
    }

    /// Replace only the loaded save, after creating a complete, uniquely named backup.
    pub fn replace_original(&self) -> Result<PathBuf> {
        let source = self.source.as_ref().context("No source save selected")?;
        ensure!(
            source.file_name().is_some_and(|n| n == "sav.dat"),
            "Replacement requires a sav.dat source"
        );
        ensure!(
            fs::read(source)? == self.original,
            "Source save changed on disk; reopen it before replacing"
        );
        let folder = source.parent().context("No source folder")?;
        let name = folder
            .file_name()
            .context("No source folder name")?
            .to_string_lossy();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_nanos();
        let stage = folder.join(format!(".replacement-{stamp}"));
        self.export_named(&stage, &name)?;
        let backup = folder.join("old").join(format!("backup-{stamp}"));
        let result = (|| -> Result<()> {
            fs::create_dir_all(&backup).context("Cannot create old backup folder")?;
            for entry in fs::read_dir(folder)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    fs::copy(entry.path(), backup.join(entry.file_name()))?;
                }
            }
            ensure!(
                fs::read(backup.join("sav.dat"))? == self.original
                    && fs::read(source)? == self.original,
                "Source changed while backing up; replacement cancelled"
            );
            let mut files = fs::read_dir(&stage)?.collect::<std::io::Result<Vec<_>>>()?;
            // Publish the save data last. Restore all published files on any error.
            files.sort_by_key(|e| e.file_name() == "sav.dat");
            let mut published = Vec::new();
            for entry in files {
                let destination = folder.join(entry.file_name());
                let existed = destination.exists();
                if let Err(error) = fs::rename(entry.path(), &destination) {
                    let mut rollback_errors = Vec::new();
                    for (path, existed) in &published {
                        let restored = if *existed {
                            fs::copy(backup.join(Path::new(path).file_name().unwrap()), path)
                                .map(|_| ())
                        } else {
                            fs::remove_file(path)
                        };
                        if let Err(e) = restored {
                            rollback_errors.push(e.to_string());
                        }
                    }
                    bail!(
                        "Replacement failed: {error}. Backup: {}. Restore errors: {:?}",
                        backup.display(),
                        rollback_errors
                    );
                }
                published.push((destination, existed));
            }
            Ok(())
        })();
        let _ = fs::remove_dir_all(&stage);
        result?;
        Ok(backup)
    }

    fn export_named(&self, destination: &Path, save_name: &str) -> Result<()> {
        ensure!(
            !destination.exists(),
            "Destination already exists; choose a new folder name"
        );
        let bytes = self.encode()?;
        let parent = destination.parent().context("No parent folder")?;
        let stem = destination
            .file_name()
            .context("No folder name")?
            .to_string_lossy();
        let stage = parent.join(format!(".{stem}.partial-{}", std::process::id()));
        fs::create_dir(&stage).context("Cannot create export staging folder")?;
        let result = (|| -> Result<()> {
            if let Some(src) = &self.source {
                for entry in fs::read_dir(src.parent().context("No source folder")?)? {
                    let e = entry?;
                    let filename = e.file_name().to_string_lossy().into_owned();
                    if e.file_type()?.is_file()
                        && (filename == "screenshot.png"
                            || filename.starts_with("metadata.") && filename.ends_with(".json"))
                    {
                        fs::copy(e.path(), stage.join(e.file_name()))?;
                    }
                }
            }
            fs::write(stage.join("sav.dat"), &bytes)?;
            if let Some(mut meta) = self.metadata.clone() {
                meta["Data"]["metadata"]["fileSize"] = bytes.len().into();
                meta["Data"]["metadata"]["name"] = save_name.into();
                fs::write(
                    stage.join("metadata.9.json"),
                    serde_json::to_vec_pretty(&meta)?,
                )?;
            }
            let reread = Self::load(&stage)?;
            ensure!(
                reread.data[self.data_start..] == self.data[self.data_start..],
                "Export verification failed"
            );
            if destination.exists() {
                bail!("Destination appeared during export");
            }
            fs::rename(&stage, destination)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_dir_all(&stage);
        }
        result
    }
}
pub fn fnv64(s: &str) -> u64 {
    s.bytes().fold(14695981039346656037u64, |h, b| {
        (h ^ b as u64).wrapping_mul(1099511628211)
    })
}
#[derive(Clone, Debug)]
pub struct Appearance {
    pub group: String,
    pub label: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug)]
pub struct Vehicle {
    pub id: u64,
    pub health: f32,
    pub raw: Vec<u8>,
}
impl Save {
    pub fn appearances(&self) -> Result<Vec<Appearance>> {
        let n = self.node("CharacetrCustomization_Appearances")?;
        let mut r = Reader::new(&self.data, n.offset + 4);
        ensure!(r.u8()? == 1, "No appearance preset");
        r.bytes(10)?;
        let mut out = Vec::new();
        for section in ["Head", "Arms", "Body"] {
            let groups = r.u32()?;
            ensure!(groups < 1000, "Invalid appearance groups");
            for _ in 0..groups {
                let name = r.string()?;
                let count = r.u32()?;
                ensure!(count < 1000, "Invalid appearance count");
                for _ in 0..count {
                    r.u64()?;
                    let start = r.pos;
                    let value = r.string()?;
                    let end = r.pos;
                    let label = r.string()?;
                    r.bytes(8)?;
                    out.push(Appearance {
                        group: format!("{section} / {name}"),
                        label,
                        value,
                        start,
                        end,
                    });
                }
                let count = r.u32()?;
                ensure!(count < 1000, "Invalid morph count");
                for _ in 0..count {
                    let label = r.string()?;
                    let start = r.pos;
                    let value = r.string()?;
                    let end = r.pos;
                    r.bytes(8)?;
                    out.push(Appearance {
                        group: format!("{section} / {name} / morph"),
                        label,
                        value,
                        start,
                        end,
                    });
                }
            }
        }
        ensure!(r.pos <= n.offset + n.size, "Appearance outside node");
        Ok(out)
    }
    pub fn set_appearance(&mut self, index: usize, value: &str) -> Result<()> {
        ensure!(
            !value.is_empty() && value.len() < 256 && !value.contains('\0'),
            "Invalid appearance definition"
        );
        let row = self
            .appearances()?
            .get(index)
            .context("Appearance entry absent")?
            .clone();
        let n = self.node("CharacetrCustomization_Appearances")?;
        let start = n.offset + 4;
        let mut payload = self.data[start..n.offset + n.size].to_vec();
        payload.splice(row.start - start..row.end - start, string(value));
        self.replace_node("CharacetrCustomization_Appearances", payload)?;
        self.appearances()?;
        Ok(())
    }
    fn garage(&self) -> Result<(usize, usize, usize)> {
        self.persistent_entry("vehicleGarageComponentPS")
    }
    pub(crate) fn persistent_entry(&self, class_name: &str) -> Result<(usize, usize, usize)> {
        let n = self.node("PersistencySystem2")?;
        let mut r = Reader::new(&self.data, n.offset + 4);
        let ids = r.u32()? as usize;
        ensure!(ids < 1000000, "Invalid persistency IDs");
        r.bytes(ids * 4 + 4)?;
        let count = r.u32()?;
        ensure!(count < 1000000, "Invalid persistency count");
        let mut found = Vec::new();
        for _ in 0..count {
            let id = r.u64()?;
            if id == 0 {
                continue;
            }
            let hash = r.u64()?;
            let size_pos = r.pos;
            let size = r.u32()? as usize;
            let start = r.pos;
            r.bytes(size)?;
            if hash == fnv64(class_name) {
                found.push((size_pos, start, r.pos));
            }
        }
        ensure!(r.pos == n.offset + n.size, "Unknown persistency trailer");
        ensure!(found.len() == 1, "Expected a unique {class_name} entry");
        Ok(found[0])
    }
    fn garage_array(&self) -> Result<(usize, usize, Vec<Vehicle>)> {
        let (_, start, end) = self.garage()?;
        let mut needle = fnv64("unlockedVehicleArray").to_le_bytes().to_vec();
        needle.extend(fnv64("array:vehicleUnlockedVehicle").to_le_bytes());
        let matches: Vec<_> = self.data[start..end]
            .windows(16)
            .enumerate()
            .filter_map(|(i, x)| (x == needle).then_some(start + i + 16))
            .collect();
        ensure!(matches.len() == 1, "Garage array layout unsupported");
        let begin = matches[0];
        let mut r = Reader::new(&self.data, begin);
        let count = r.u32()?;
        ensure!(count < 10000, "Invalid garage count");
        let mut out = Vec::new();
        for _ in 0..count {
            let a = r.pos;
            let mut id = 0;
            let mut health = 100.0;
            loop {
                let hash = r.u64()?;
                if hash == 0 {
                    break;
                }
                let kind = r.u64()?;
                if hash == fnv64("vehicleID") && kind == fnv64("vehicleGarageVehicleID") {
                    loop {
                        let field = r.u64()?;
                        if field == 0 {
                            break;
                        }
                        let t = r.u64()?;
                        if field == fnv64("recordID") && t == fnv64("TweakDBID") {
                            id = r.u64()?;
                        } else if field == fnv64("name") && t == fnv64("CName") {
                            r.u64()?;
                        } else {
                            anyhow::bail!("Unknown garage vehicle field");
                        }
                    }
                } else if hash == fnv64("health") && kind == fnv64("Float") {
                    health = f32::from_bits(r.u32()?);
                } else {
                    anyhow::bail!("Unknown unlocked vehicle field");
                }
            }
            ensure!(id != 0 && r.pos <= end, "Invalid vehicle");
            out.push(Vehicle {
                id,
                health,
                raw: self.data[a..r.pos].to_vec(),
            });
        }
        Ok((begin, r.pos, out))
    }
    pub fn vehicles(&self) -> Result<Vec<Vehicle>> {
        Ok(self.garage_array()?.2)
    }
    pub fn set_vehicle(&mut self, id: u64, unlocked: bool) -> Result<()> {
        let (a, b, mut rows) = self.garage_array()?;
        if unlocked {
            if rows.iter().any(|v| v.id == id) {
                return Ok(());
            }
            let mut raw = Vec::new();
            for h in [
                fnv64("vehicleID"),
                fnv64("vehicleGarageVehicleID"),
                fnv64("recordID"),
                fnv64("TweakDBID"),
                id,
                0,
                0,
            ] {
                raw.extend(h.to_le_bytes());
            }
            rows.push(Vehicle {
                id,
                health: 100.0,
                raw,
            });
        } else {
            rows.retain(|v| v.id != id);
        }
        let mut encoded = (rows.len() as u32).to_le_bytes().to_vec();
        for row in rows {
            encoded.extend(row.raw);
        }
        let delta = encoded.len() as isize - (b - a) as isize;
        let (size_pos, gstart, gend) = self.garage()?;
        let n = self.node("PersistencySystem2")?;
        let base = n.offset + 4;
        let mut payload = self.data[base..n.offset + n.size].to_vec();
        payload.splice(a - base..b - base, encoded);
        put32(
            &mut payload,
            size_pos - base,
            ((gend - gstart) as isize + delta) as u32,
        )?;
        let mut candidate = self.clone();
        candidate.replace_node("PersistencySystem2", payload)?;
        candidate.vehicles()?;
        *self = candidate;
        Ok(())
    }
}
impl Save {
    /// Adds only simple stackable records. Unique equipment requires a separate, versioned schema.
    pub fn add_stack(&mut self, inventory: u64, record: &str, quantity: u32) -> Result<()> {
        ensure!(quantity > 0 && quantity <= 999999999, "Invalid quantity");
        ensure!(
            STACK_RECORDS.contains(&record),
            "Record is not in the verified stack catalog"
        );
        let id = tdb(record);
        if let Some(item) = self
            .items()?
            .into_iter()
            .find(|i| i.inventory == inventory && i.id == id)
        {
            let p = item.quantity.context("Existing item is not stackable")?;
            let old = u32_at(&self.data, p)?;
            return put32(
                &mut self.data,
                p,
                old.checked_add(quantity).context("Quantity overflow")?,
            );
        }
        let n = self.node("inventory")?.clone();
        let mut r = Reader::new(&self.data, n.offset + 4);
        let cnt = r.u32()?;
        let mut location = None;
        for _ in 0..cnt {
            let inv = r.u64()?;
            let count_pos = r.pos;
            let items = r.u32()?;
            for _ in 0..items {
                r.bytes(16)?;
                let node = self
                    .nodes
                    .iter()
                    .find(|n| n.offset == r.pos)
                    .context("Invalid inventory node")?;
                r.pos = node.offset + node.size;
            }
            if inv == inventory {
                location = Some((count_pos, items, r.pos));
            }
        }
        let (count_pos, count, insert) = location.context("Inventory not found")?;
        let mut candidate = self.clone();
        let idx = candidate
            .nodes
            .iter()
            .position(|n| n.offset >= insert)
            .context("No node after inventory")?;
        let new_id = idx as u32;
        let mut header = id.to_le_bytes().to_vec();
        header.extend(2u32.to_le_bytes());
        header.push(2);
        header.extend([0u8; 3]);
        let mut payload = header.clone();
        payload.extend(new_id.to_le_bytes());
        payload.extend(&header);
        payload.push(0);
        payload.extend(0u32.to_le_bytes());
        payload.extend(quantity.to_le_bytes());
        let delta = payload.len();
        candidate.data.splice(insert..insert, payload);
        for node in &mut candidate.nodes {
            if node.offset >= insert {
                node.offset += delta;
            } else if node.offset <= n.offset && node.offset + node.size >= insert {
                node.size += delta;
            }
            if node.next != u32::MAX && node.next >= new_id {
                node.next += 1;
            }
            if node.child != u32::MAX && node.child >= new_id {
                node.child += 1;
            }
            let old = u32_at(&candidate.data, node.offset)?;
            ensure!(old < 1000000, "Invalid node ID");
            if old >= new_id {
                put32(&mut candidate.data, node.offset, old + 1)?;
            }
        }
        // All itemData nodes are siblings even across different subinventories.
        if let Some(prev) = candidate.nodes[..idx].iter_mut().rev().find(|x| {
            x.name == "itemData" && x.offset >= n.offset && x.offset < n.offset + n.size + delta
        }) {
            prev.next = new_id;
        } else {
            candidate
                .nodes
                .iter_mut()
                .find(|x| x.name == "inventory")
                .context("Inventory parent missing")?
                .child = new_id;
        }
        candidate.nodes.insert(
            idx,
            Node {
                name: "itemData".into(),
                next: if idx + 1 < self.nodes.len() && self.nodes[idx].name == "itemData" {
                    new_id + 1
                } else {
                    u32::MAX
                },
                child: u32::MAX,
                offset: insert + 16,
                size: 29,
            },
        );
        put32(&mut candidate.data, count_pos, count + 1)?;
        candidate.items()?;
        *self = candidate;
        Ok(())
    }
}
pub const STACK_RECORDS: &[&str] = &[
    "Items.money",
    "Items.CommonMaterial1",
    "Items.UncommonMaterial1",
    "Items.RareMaterial1",
    "Items.EpicMaterial1",
    "Items.LegendaryMaterial1",
    "Items.HandgunAmmo",
    "Items.RifleAmmo",
    "Items.ShotgunAmmo",
    "Items.SniperRifleAmmo",
];
