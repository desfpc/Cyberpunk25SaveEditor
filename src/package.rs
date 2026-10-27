//! RED package reader: only known fields are interpreted; all other field bytes are retained.
use crate::{binary::*, save::Save};
use anyhow::{ensure, Context, Result};
#[derive(Clone, Debug)]
pub struct Field {
    pub name: u16,
    pub kind: u16,
    pub data: Vec<u8>,
}
#[derive(Clone, Debug)]
pub struct Class {
    pub fields: Vec<Field>,
}
#[derive(Clone)]
pub struct Package {
    pub bytes: Vec<u8>,
    pub names: Vec<String>,
    pub base: usize,
    pub desc: usize,
    pub chunks: Vec<(String, usize, usize)>,
}
impl Class {
    pub fn field<'a>(&'a self, p: &Package, name: &str) -> Option<&'a Field> {
        self.fields
            .iter()
            .find(|f| p.names[f.name as usize] == name)
    }
    pub fn set(&mut self, p: &Package, name: &str, kind: &str, data: Vec<u8>) -> Result<()> {
        if let Some(f) = self
            .fields
            .iter_mut()
            .find(|f| p.names[f.name as usize] == name)
        {
            ensure!(p.names[f.kind as usize] == kind, "Field type mismatch");
            f.data = data;
        } else {
            self.fields.push(Field {
                name: p.name_id(name)?,
                kind: p.name_id(kind)?,
                data,
            });
        }
        Ok(())
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut out = (self.fields.len() as u16).to_le_bytes().to_vec();
        let mut offset = 2 + self.fields.len() * 8;
        for f in &self.fields {
            out.extend(f.name.to_le_bytes());
            out.extend(f.kind.to_le_bytes());
            out.extend((offset as u32).to_le_bytes());
            offset += f.data.len();
        }
        for f in &self.fields {
            out.extend(&f.data);
        }
        out
    }
    pub fn integer(&self, p: &Package, name: &str) -> Result<u32> {
        self.field(p, name)
            .map(|f| u32_at(&f.data, 0))
            .unwrap_or(Ok(0))
    }
    pub fn enumeration(&self, p: &Package, name: &str, default: &str) -> Result<String> {
        if let Some(f) = self.field(p, name) {
            let id = Reader::new(&f.data, 0).u16()?;
            Ok(p.names.get(id as usize).context("Invalid enum")?.clone())
        } else {
            Ok(default.into())
        }
    }
}
impl Package {
    pub fn from_save(s: &Save, name: &str) -> Result<Self> {
        let n = s.node(name)?;
        let len = u32_at(&s.data, n.offset + 4)? as usize;
        ensure!(len + 8 == n.size, "Package length mismatch");
        Self::parse(
            s.data[n.offset + 8..n.offset + n.size].to_vec(),
            name == "ScriptableSystemsContainer",
        )
    }
    pub fn parse(bytes: Vec<u8>, scriptable: bool) -> Result<Self> {
        let mut r = Reader::new(&bytes, 0);
        ensure!(r.u8()? == 4, "Unsupported RED package version");
        r.u8()?;
        let sections = r.u16()?;
        ensure!(sections == 6 || sections == 7, "Invalid RED sections");
        r.u32()?;
        if sections == 7 {
            ensure!(r.u32()? == 0, "Unsupported reference pool");
            r.u32()?;
        }
        let nd = r.u32()? as usize;
        let ns = r.u32()? as usize;
        let desc = r.u32()? as usize;
        let cd = r.u32()? as usize;
        if scriptable {
            let count = r.u32()? as usize;
            ensure!(count < 100000, "Invalid root count");
            r.bytes(count * 8)?;
        }
        let base = r.pos;
        ensure!(
            nd <= ns && ns <= desc && desc <= cd && base + cd <= bytes.len(),
            "Invalid package pools"
        );
        let mut names = Vec::new();
        for i in (nd..ns).step_by(4) {
            let off = (u32_at(&bytes, base + i)? & 0xffffff) as usize;
            let data = bytes.get(base + off..).context("Invalid name offset")?;
            let end = data
                .iter()
                .position(|b| *b == 0)
                .context("Unterminated name")?;
            names.push(std::str::from_utf8(&data[..end])?.to_owned());
        }
        let mut chunks = Vec::new();
        for i in (desc..cd).step_by(8) {
            let id = u32_at(&bytes, base + i)? as usize;
            let off = base + u32_at(&bytes, base + i + 4)? as usize;
            ensure!(
                off >= base + cd && off <= bytes.len(),
                "Invalid chunk offset"
            );
            chunks.push((names.get(id).context("Invalid chunk type")?.clone(), off, 0));
        }
        for i in 0..chunks.len() {
            let end = chunks.get(i + 1).map(|c| c.1).unwrap_or(bytes.len());
            ensure!(end >= chunks[i].1, "Unordered chunks");
            chunks[i].2 = end;
        }
        Ok(Self {
            bytes,
            names,
            base,
            desc,
            chunks,
        })
    }
    pub fn name_id(&self, s: &str) -> Result<u16> {
        self.names
            .iter()
            .position(|n| n == s)
            .map(|n| n as u16)
            .with_context(|| format!("Name {s} absent from package"))
    }
    /// Append names without renumbering existing IDs or rewriting unknown chunks.
    pub(crate) fn ensure_names(&mut self, names: &[String]) -> Result<()> {
        let mut missing = Vec::new();
        for name in names {
            if !self.names.contains(name) && !missing.contains(name) {
                ensure!(
                    !name.contains('\0') && name.len() < 255,
                    "Invalid package name"
                );
                missing.push(name.clone());
            }
        }
        if missing.is_empty() {
            return Ok(());
        }
        ensure!(
            self.names.len() + missing.len() <= u16::MAX as usize,
            "Package name pool is full"
        );
        let header = if u16::from_le_bytes(self.bytes[2..4].try_into()?) == 7 {
            16
        } else {
            8
        };
        let nd = u32_at(&self.bytes, header)? as usize;
        let ns = u32_at(&self.bytes, header + 4)? as usize;
        let cd = u32_at(&self.bytes, header + 12)? as usize;
        let descriptors = missing.len() * 4;
        let extra: usize = missing.iter().map(|s| s.len() + 1).sum();
        let delta = descriptors + extra;
        ensure!(cd + delta < 0x1000000, "Package name pool is too large");
        let mut bytes = self.bytes[..self.base + ns].to_vec();
        let mut position = self.desc + descriptors;
        for name in &missing {
            bytes.extend(((position as u32) | (((name.len() + 1) as u32) << 24)).to_le_bytes());
            position += name.len() + 1;
        }
        bytes.extend(&self.bytes[self.base + ns..self.base + self.desc]);
        for name in &missing {
            bytes.extend(name.as_bytes());
            bytes.push(0);
        }
        bytes.extend(&self.bytes[self.base + self.desc..]);
        for i in (nd..ns).step_by(4) {
            let value = u32_at(&self.bytes, self.base + i)?;
            put32(
                &mut bytes,
                self.base + i,
                (value & 0xff000000) | ((value & 0xffffff) + descriptors as u32),
            )?;
        }
        put32(&mut bytes, header + 4, (ns + descriptors) as u32)?;
        put32(&mut bytes, header + 8, (self.desc + delta) as u32)?;
        put32(&mut bytes, header + 12, (cd + delta) as u32)?;
        for (i, (_, start, _)) in self.chunks.iter().enumerate() {
            put32(
                &mut bytes,
                self.base + self.desc + delta + i * 8 + 4,
                (start - self.base + delta) as u32,
            )?;
        }
        *self = Self::parse(bytes, true)?;
        Ok(())
    }
    pub fn class(&self, bytes: &[u8]) -> Result<Class> {
        let mut r = Reader::new(bytes, 0);
        let count = r.u16()? as usize;
        ensure!(
            count <= 4096 && 2 + count * 8 <= bytes.len(),
            "Invalid class field count"
        );
        let mut headers = Vec::new();
        for _ in 0..count {
            let n = r.u16()?;
            let t = r.u16()?;
            let off = r.u32()? as usize;
            ensure!(
                (n as usize) < self.names.len()
                    && (t as usize) < self.names.len()
                    && off >= 2 + count * 8
                    && off <= bytes.len(),
                "Invalid class field"
            );
            headers.push((n, t, off));
        }
        let mut fields = Vec::new();
        for (n, t, o) in &headers {
            let end = headers
                .iter()
                .filter_map(|(_, _, x)| (*x > *o).then_some(*x))
                .min()
                .unwrap_or(bytes.len());
            fields.push(Field {
                name: *n,
                kind: *t,
                data: bytes[*o..end].to_vec(),
            });
        }
        Ok(Class { fields })
    }
    pub fn class_len(&self, bytes: &[u8], depth: usize) -> Result<usize> {
        ensure!(depth < 12, "Class nesting too deep");
        let c = self.class(bytes)?;
        let mut end = 2 + c.fields.len() * 8;
        let mut r = Reader::new(bytes, 2);
        for f in &c.fields {
            r.bytes(4)?;
            let off = r.u32()? as usize;
            let size = self.value_len(&self.names[f.kind as usize], &bytes[off..], depth + 1)?;
            end = end.max(off + size);
        }
        ensure!(end <= bytes.len(), "Class exceeds buffer");
        Ok(end)
    }
    pub(crate) fn value_len(&self, t: &str, b: &[u8], depth: usize) -> Result<usize> {
        if let Some(t) = t.strip_prefix("array:") {
            let count = u32_at(b, 0)? as usize;
            ensure!(count < 100000, "Array too large");
            let mut end = 4;
            for _ in 0..count {
                end += self.value_len(t, b.get(end..).context("Array truncated")?, depth + 1)?;
            }
            return Ok(end);
        }
        let size = match t {
            "Int32" | "Uint32" | "Float" => 4,
            "Uint16" | "Int16" | "CName" => 2,
            "Uint64" | "Int64" | "Double" | "TweakDBID" => 8,
            "Bool" | "Uint8" | "Int8" => 1,
            _ if t.starts_with("gamedata") || t == "gameStatIDType" => 2,
            _ if t.starts_with("handle:") => 4,
            _ => return self.class_len(b, depth),
        };
        ensure!(b.len() >= size, "Truncated scalar");
        Ok(size)
    }
    pub fn array(&self, b: &[u8]) -> Result<Vec<Class>> {
        let count = u32_at(b, 0)? as usize;
        ensure!(count < 100000, "Array too large");
        let mut off = 4;
        let mut out = Vec::new();
        for _ in 0..count {
            let len = self.class_len(b.get(off..).context("Truncated array")?, 0)?;
            out.push(self.class(&b[off..off + len])?);
            off += len;
        }
        ensure!(off == b.len(), "Unknown array trailer");
        Ok(out)
    }
    pub fn array_bytes(c: &[Class]) -> Vec<u8> {
        let mut b = (c.len() as u32).to_le_bytes().to_vec();
        for x in c {
            b.extend(x.encode());
        }
        b
    }
    pub fn chunk(&self, name: &str) -> Result<(usize, Class)> {
        let i = self
            .chunks
            .iter()
            .position(|c| c.0 == name)
            .with_context(|| format!("Missing package class {name}"))?;
        let (_, a, b) = self.chunks[i];
        Ok((i, self.class(&self.bytes[a..b])?))
    }
    pub fn player_chunk(&self) -> Result<(usize, Class)> {
        let mut found = Vec::new();
        for (i, (name, start, end)) in self.chunks.iter().enumerate() {
            if name != "PlayerDevelopmentData" {
                continue;
            }
            let c = self.class(&self.bytes[*start..*end])?;
            if let Some(owner) = c.field(self, "ownerID") {
                let owner = self.class(&owner.data)?;
                if owner
                    .field(self, "hash")
                    .and_then(|f| Reader::new(&f.data, 0).u64().ok())
                    == Some(1)
                {
                    found.push((i, c));
                }
            }
        }
        ensure!(
            found.len() == 1,
            "Expected unique PlayerDevelopmentData for entity 1"
        );
        Ok(found.remove(0))
    }
    pub fn replace_chunk(&mut self, i: usize, c: Class) -> Result<()> {
        let (_, start, end) = self.chunks[i];
        let encoded = c.encode();
        let delta = encoded.len() as isize - (end - start) as isize;
        self.bytes.splice(start..end, encoded);
        for (j, (_, a, b)) in self.chunks.iter_mut().enumerate() {
            if j == i {
                *b = (*b as isize + delta) as usize;
            } else if j > i {
                *a = (*a as isize + delta) as usize;
                *b = (*b as isize + delta) as usize;
            }
            put32(
                &mut self.bytes,
                self.base + self.desc + j * 8 + 4,
                (*a - self.base) as u32,
            )?;
        }
        Ok(())
    }
    pub fn write(self, s: &mut Save, name: &str) -> Result<()> {
        let mut b = (self.bytes.len() as u32).to_le_bytes().to_vec();
        b.extend(self.bytes);
        s.replace_node(name, b)
    }
}
#[derive(Clone, Debug)]
pub struct PlayerStat {
    pub group: String,
    pub key: String,
    pub value: u32,
    pub max: u32,
}
impl Save {
    pub fn player_stats(&self) -> Result<Vec<PlayerStat>> {
        let p = Package::from_save(self, "ScriptableSystemsContainer")?;
        let (_, c) = p.player_chunk()?;
        let mut out = Vec::new();
        for (group, field, key, val, default) in [
            (
                "Attributes",
                "attributes",
                "attributeName",
                "value",
                "Strength",
            ),
            (
                "Progression",
                "proficiencies",
                "type",
                "currentLevel",
                "Assault",
            ),
            ("Points", "devPoints", "type", "unspent", "Attribute"),
        ] {
            let f = c.field(&p, field).context("Missing player data field")?;
            for row in p.array(&f.data)? {
                let k = row.enumeration(&p, key, default)?;
                if group == "Attributes"
                    && ![
                        "Strength",
                        "Reflexes",
                        "TechnicalAbility",
                        "Intelligence",
                        "Cool",
                    ]
                    .contains(&k.as_str())
                {
                    continue;
                }
                if group == "Progression"
                    && ![
                        "Level",
                        "StreetCred",
                        "CoolSkill",
                        "IntelligenceSkill",
                        "StrengthSkill",
                        "ReflexesSkill",
                        "TechnicalAbilitySkill",
                    ]
                    .contains(&k.as_str())
                {
                    continue;
                }
                out.push(PlayerStat {
                    group: group.into(),
                    key: k,
                    value: row.integer(&p, val)?,
                    max: if group == "Attributes" {
                        20
                    } else if group == "Points" {
                        999
                    } else if group == "Progression"
                        && row.enumeration(&p, key, default)? == "StreetCred"
                    {
                        50
                    } else {
                        60
                    },
                });
            }
        }
        Ok(out)
    }
    pub fn set_player_stat(&mut self, group: &str, key: &str, value: u32) -> Result<()> {
        let mut candidate = self.clone();
        candidate.set_player_stat_inner(group, key, value)?;
        *self = candidate;
        Ok(())
    }
    fn set_player_stat_inner(&mut self, group: &str, key: &str, value: u32) -> Result<()> {
        let stat = self
            .player_stats()?
            .into_iter()
            .find(|s| s.group == group && s.key == key)
            .context("Unknown player stat")?;
        ensure!(
            value <= stat.max
                && (group == "Points" || value >= if group == "Attributes" { 3 } else { 1 }),
            "Value out of range"
        );
        let (array, kind, name, field, default) = match group {
            "Attributes" => (
                "attributes",
                "array:SAttribute",
                "attributeName",
                "value",
                "Strength",
            ),
            "Progression" => (
                "proficiencies",
                "array:SProficiency",
                "type",
                "currentLevel",
                "Assault",
            ),
            "Points" => (
                "devPoints",
                "array:SDevelopmentPoints",
                "type",
                "unspent",
                "Attribute",
            ),
            _ => anyhow::bail!("Unknown stat group"),
        };
        let mut p = Package::from_save(self, "ScriptableSystemsContainer")?;
        // RED omits default-valued properties. A fresh save can have neither
        // the unspent field nor its name in the pool; add both when first edited.
        p.ensure_names(&[field.into(), "Int32".into()])?;
        let (idx, mut c) = p.player_chunk()?;
        let mut rows = p.array(&c.field(&p, array).context("Missing array")?.data)?;
        let row = rows
            .iter_mut()
            .find(|r| r.enumeration(&p, name, default).ok().as_deref() == Some(key))
            .context("Stat absent")?;
        row.set(&p, field, "Int32", value.to_le_bytes().to_vec())?;
        if group == "Progression" && row.field(&p, "currentExp").is_some() {
            row.set(&p, "currentExp", "Int32", 0u32.to_le_bytes().to_vec())?;
        }
        c.set(&p, array, kind, Package::array_bytes(&rows))?;
        p.replace_chunk(idx, c)?;
        p.write(self, "ScriptableSystemsContainer")?;
        if group != "Points" {
            self.patch_forced_stat(key, value)?;
        }
        if let Some(meta) = &mut self.metadata {
            let meta_key = match key {
                "Strength" => "strength",
                "Reflexes" => "reflexes",
                "Intelligence" => "intelligence",
                "TechnicalAbility" => "technicalAbility",
                "Cool" => "cool",
                "Level" => "level",
                "StreetCred" => "streetCred",
                _ => "",
            };
            if !meta_key.is_empty() {
                meta["Data"]["metadata"][meta_key] = value.into();
            }
        }
        Ok(())
    }
    fn patch_forced_stat(&mut self, key: &str, value: u32) -> Result<()> {
        let p = Package::from_save(self, "StatsSystem")?;
        let (_, c) = p.chunk("gameStatsStateMapStructure")?;
        let rows = p.array(
            &c.field(&p, "valuesOptimized")
                .context("2.5 optimized stats missing")?
                .data,
        )?;
        let player = rows
            .iter()
            .find(|r| {
                r.field(&p, "entityHash")
                    .and_then(|f| Reader::new(&f.data, 0).u64().ok())
                    == Some(1)
            })
            .context("Player stat map absent")?;
        let indices = &player
            .field(&p, "bufferSliceIndices")
            .context("Player stat slices missing")?
            .data;
        let slices = p.array(
            &c.field(&p, "sharedBufferSlices")
                .context("Stats slices missing")?
                .data,
        )?;
        let buffer = &c
            .field(&p, "sharedDataBuffer")
            .context("Stats buffer missing")?
            .data;
        let buffer_len = u32_at(buffer, 0)? as usize;
        ensure!(buffer.len() == buffer_len + 4, "Unsupported stats buffer");
        let hash = key.bytes().fold(14695981039346656037u64, |h, b| {
            (h ^ b as u64).wrapping_mul(1099511628211)
        });
        let mut found = Vec::new();
        let mut r = Reader::new(indices, 0);
        let count = r.u32()?;
        for _ in 0..count {
            let idx = r.u16()? as usize;
            let slice = slices.get(idx).context("Invalid slice index")?;
            let combined = slice.integer(&p, "combined")?;
            if combined >> 30 != 1 {
                continue;
            }
            // A shared slice cannot be patched in place without changing other entities.
            let references = rows
                .iter()
                .filter(|row| {
                    row.field(&p, "bufferSliceIndices").is_some_and(|f| {
                        let mut ir = Reader::new(&f.data, 0);
                        ir.u32()
                            .ok()
                            .is_some_and(|n| (0..n).any(|_| ir.u16().ok() == Some(idx as u16)))
                    })
                })
                .count();
            ensure!(
                references == 1,
                "Player forced modifiers are shared; in-place edit refused"
            );
            let start = slice.integer(&p, "offset")? as usize + 4;
            let end = start + (combined & 0x3fffffff) as usize;
            let mut r = Reader::new(buffer, start);
            while r.pos < end {
                let cls = r.string()?;
                let stat = r.u64()?;
                let modifier = r.u32()?;
                match cls.as_str() {
                    "gameConstantStatModifier" => {
                        if stat == hash && modifier == 0 {
                            found.push(r.pos);
                        }
                        r.bytes(4)?;
                    }
                    "gameCombinedStatModifier" => {
                        r.bytes(20)?;
                    }
                    "gameCurveStatModifier" => {
                        r.string()?;
                        r.string()?;
                        r.bytes(12)?;
                    }
                    _ => anyhow::bail!("Unknown stat modifier {cls}"),
                };
            }
            ensure!(r.pos == end, "Modifier slice mismatch");
        }
        ensure!(
            found.len() == 1,
            "Expected one unique forced {key} modifier, got {}; refusing ambiguous edit",
            found.len()
        );
        let (_, class_start, _) = p.chunks[0];
        let mut r = Reader::new(&p.bytes, class_start + 2);
        let count = (class_start + 2 + 8 * c.fields.len() - r.pos) / 8;
        let mut buf_offset = None;
        for _ in 0..count {
            let n = r.u16()?;
            r.u16()?;
            let off = r.u32()? as usize;
            if p.names[n as usize] == "sharedDataBuffer" {
                buf_offset = Some(class_start + off);
            }
        }
        let n = self.node("StatsSystem")?;
        let absolute = n.offset + 8 + buf_offset.context("Missing buffer offset")? + found[0];
        put32(&mut self.data, absolute, (value as f32).to_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_points_save(remove_name: bool) -> Save {
        let mut s = Save::load(std::path::Path::new("saveExample")).unwrap();
        let mut p = Package::from_save(&s, "ScriptableSystemsContainer").unwrap();
        let (index, mut player) = p.player_chunk().unwrap();
        let mut rows = p
            .array(&player.field(&p, "devPoints").unwrap().data)
            .unwrap();
        for row in &mut rows {
            row.fields
                .retain(|f| !matches!(p.names[f.name as usize].as_str(), "spent" | "unspent"));
        }
        player
            .set(
                &p,
                "devPoints",
                "array:SDevelopmentPoints",
                Package::array_bytes(&rows),
            )
            .unwrap();
        p.replace_chunk(index, player).unwrap();
        if remove_name {
            // Remove the name from the pool as well, preserving every existing ID.
            let id = p.name_id("unspent").unwrap() as usize;
            let header = if u16::from_le_bytes(p.bytes[2..4].try_into().unwrap()) == 7 {
                16
            } else {
                8
            };
            let nd = u32_at(&p.bytes, header).unwrap() as usize;
            let offset =
                p.base + (u32_at(&p.bytes, p.base + nd + id * 4).unwrap() & 0xffffff) as usize;
            p.bytes[offset..offset + 7].copy_from_slice(b"unused_");
        }
        p.write(&mut s, "ScriptableSystemsContainer").unwrap();
        Save::parse(s.encode().unwrap()).unwrap()
    }

    #[test]
    fn points_can_be_added_before_any_are_earned_or_spent() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        for remove_name in [false, true] {
            let mut s = fresh_points_save(remove_name);
            let before = Package::from_save(&s, "ScriptableSystemsContainer").unwrap();
            assert_eq!(before.name_id("unspent").is_err(), remove_name);
            let (_, player) = before.player_chunk().unwrap();
            let before_rows = before
                .array(&player.field(&before, "devPoints").unwrap().data)
                .unwrap();
            assert!(before_rows
                .iter()
                .all(|r| r.field(&before, "spent").is_none()
                    && r.field(&before, "unspent").is_none()));
            for (key, value) in [("Primary", 80), ("Attribute", 25), ("Primary", 0)] {
                if value != 0 {
                    assert_eq!(
                        s.player_stats()
                            .unwrap()
                            .iter()
                            .find(|r| r.group == "Points" && r.key == key)
                            .unwrap()
                            .value,
                        0
                    );
                }
                s.set_player_stat("Points", key, value).unwrap();
                s = Save::parse(s.encode().unwrap()).unwrap();
                assert_eq!(
                    s.player_stats()
                        .unwrap()
                        .iter()
                        .find(|r| r.group == "Points" && r.key == key)
                        .unwrap()
                        .value,
                    value
                );
                let after = Package::from_save(&s, "ScriptableSystemsContainer").unwrap();
                let (player_index, player) = after.player_chunk().unwrap();
                let rows = after
                    .array(&player.field(&after, "devPoints").unwrap().data)
                    .unwrap();
                assert_eq!(rows.len(), before_rows.len());
                for (original, mut updated) in before_rows.iter().zip(rows) {
                    updated
                        .fields
                        .retain(|f| after.names[f.name as usize] != "unspent");
                    assert_eq!(original.encode(), updated.encode());
                }
                for (i, (name, start, end)) in before.chunks.iter().enumerate() {
                    if i != player_index {
                        let (_, a, b) = after.chunks[i];
                        assert_eq!(
                            &before.bytes[*start..*end],
                            &after.bytes[a..b],
                            "Changed unrelated chunk {name}"
                        );
                    }
                }
            }
        }
    }
}
