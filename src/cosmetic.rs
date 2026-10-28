//! Portable snapshots of cyberware cosmetic selections. Equipment and stats are excluded.
use crate::{
    binary::{u32_at, Reader},
    package::{Class, Field, Package},
    save::Save,
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
const NODE: &str = "ScriptableSystemsContainer";
const SYSTEM: &str = "CyberwareCustomizationSystem";

/// Reset to the game's empty/default cosmetic selection. Equipment is separate.
pub(crate) fn reset(s: &mut Save) -> Result<()> {
    let mut p = Package::from_save(s, NODE)?;
    if !p.chunks.iter().any(|(name, _, _)| name == SYSTEM) {
        return Ok(());
    }
    let (index, _) = p.chunk(SYSTEM)?;
    p.replace_chunk(index, Class { fields: vec![] })?;
    p.write(s, NODE)
}
#[cfg(test)]
const DATA: &str = "CyberwareCustomizationPersistentData";

#[derive(Serialize, Deserialize)]
pub struct CosmeticSnapshot {
    system: CosmeticClass,
}
#[derive(Serialize, Deserialize)]
struct CosmeticClass {
    fields: Vec<CosmeticField>,
}
#[derive(Serialize, Deserialize)]
struct CosmeticField {
    name: String,
    kind: String,
    value: CosmeticValue,
}
#[derive(Serialize, Deserialize)]
enum CosmeticValue {
    Bytes(Vec<u8>),
    Name(String),
    Object(CosmeticClass),
    Array(Vec<CosmeticValue>),
}
fn scalar(kind: &str) -> Option<usize> {
    match kind {
        "Bool" | "Uint8" | "Int8" => Some(1),
        "Uint16" | "Int16" => Some(2),
        "Int32" | "Uint32" | "Float" => Some(4),
        "Int64" | "Uint64" | "Double" | "TweakDBID" => Some(8),
        _ => None,
    }
}
fn named(kind: &str) -> bool {
    kind == "CName" || kind.starts_with("gamedata")
}
// gameItemID is a tagged class with fields such as id and rngSeed, rather than
// an opaque inventory ID. Its package-local field/type names must also be rebound.
fn object(kind: &str) -> bool {
    !kind.contains(':') && (kind.starts_with("Cyberware") || kind == "gameItemID")
}
impl CosmeticClass {
    fn read(p: &Package, data: &[u8], depth: usize) -> Result<Self> {
        ensure!(depth < 12, "Cosmetic data nesting is too deep");
        let class = p.class(data)?;
        let fields = class
            .fields
            .into_iter()
            .map(|f| {
                let kind = p.names[f.kind as usize].clone();
                Ok(CosmeticField {
                    name: p.names[f.name as usize].clone(),
                    value: CosmeticValue::read(p, &kind, &f.data, depth + 1)?,
                    kind,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { fields })
    }
    fn names(&self, out: &mut Vec<String>, depth: usize) -> Result<()> {
        ensure!(
            depth < 12 && self.fields.len() < 1000,
            "Invalid cosmetic class"
        );
        let mut seen = std::collections::HashSet::new();
        for f in &self.fields {
            ensure!(
                seen.insert(&f.name),
                "Duplicate cosmetic property {}",
                f.name
            );
            out.push(f.name.clone());
            out.push(f.kind.clone());
            f.value.names(&f.kind, out, depth + 1)?;
        }
        Ok(())
    }
    fn encode(&self, p: &Package) -> Result<Vec<u8>> {
        let fields = self
            .fields
            .iter()
            .map(|f| {
                Ok(Field {
                    name: p.name_id(&f.name)?,
                    kind: p.name_id(&f.kind)?,
                    data: f.value.encode(p)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Class { fields }.encode())
    }
}
impl CosmeticValue {
    fn read(p: &Package, kind: &str, data: &[u8], depth: usize) -> Result<Self> {
        ensure!(depth < 12, "Cosmetic data nesting is too deep");
        if let Some(kind) = kind.strip_prefix("array:") {
            let count = u32_at(data, 0)?;
            ensure!(count < 1000, "Too many cosmetic values");
            let mut offset = 4;
            let mut values = Vec::new();
            for _ in 0..count {
                let size = p.value_len(
                    kind,
                    data.get(offset..).context("Truncated cosmetic array")?,
                    depth,
                )?;
                values.push(Self::read(
                    p,
                    kind,
                    &data[offset..offset + size],
                    depth + 1,
                )?);
                offset += size;
            }
            ensure!(offset == data.len(), "Unexpected cosmetic array trailer");
            return Ok(Self::Array(values));
        }
        if named(kind) {
            ensure!(data.len() == 2, "Invalid cosmetic name");
            let id = Reader::new(data, 0).u16()? as usize;
            return Ok(Self::Name(
                p.names.get(id).context("Invalid cosmetic name ID")?.clone(),
            ));
        }
        if let Some(size) = scalar(kind) {
            ensure!(data.len() == size, "Invalid cosmetic scalar");
            return Ok(Self::Bytes(data.to_vec()));
        }
        ensure!(
            object(kind),
            "Unsupported cosmetic value type {kind}; preset capture cancelled"
        );
        Ok(Self::Object(CosmeticClass::read(p, data, depth + 1)?))
    }
    fn names(&self, kind: &str, out: &mut Vec<String>, depth: usize) -> Result<()> {
        ensure!(depth < 12, "Cosmetic data nesting is too deep");
        match self {
            Self::Array(values) => {
                ensure!(values.len() < 1000, "Too many cosmetic values");
                let kind = kind
                    .strip_prefix("array:")
                    .context("Invalid cosmetic array type")?;
                for v in values {
                    v.names(kind, out, depth + 1)?;
                }
            }
            Self::Name(value) => {
                ensure!(named(kind), "Invalid cosmetic name type");
                out.push(value.clone());
            }
            Self::Bytes(value) => ensure!(
                scalar(kind) == Some(value.len()),
                "Invalid cosmetic scalar type"
            ),
            Self::Object(value) => {
                ensure!(object(kind), "Invalid cosmetic object type");
                value.names(out, depth + 1)?;
            }
        }
        Ok(())
    }
    fn encode(&self, p: &Package) -> Result<Vec<u8>> {
        match self {
            Self::Bytes(v) => Ok(v.clone()),
            Self::Name(v) => Ok(p.name_id(v)?.to_le_bytes().to_vec()),
            Self::Object(v) => v.encode(p),
            Self::Array(v) => {
                let mut bytes = (v.len() as u32).to_le_bytes().to_vec();
                for value in v {
                    bytes.extend(value.encode(p)?);
                }
                Ok(bytes)
            }
        }
    }
}
impl CosmeticSnapshot {
    pub fn capture(s: &Save) -> Result<Option<Self>> {
        let p = Package::from_save(s, NODE)?;
        if !p.chunks.iter().any(|(name, _, _)| name == SYSTEM) {
            return Ok(None);
        }
        let (_, start, end) = p
            .chunks
            .iter()
            .find(|(name, _, _)| name == SYSTEM)
            .context("No cyberware cosmetic system")?;
        Ok(Some(Self {
            system: CosmeticClass::read(&p, &p.bytes[*start..*end], 0)?,
        }))
    }
    pub fn validate(&self) -> Result<()> {
        self.system.names(&mut vec![], 0)
    }
    pub fn apply(&self, s: &mut Save) -> Result<()> {
        self.validate()?;
        let mut p = Package::from_save(s, NODE)?;
        let mut names = Vec::new();
        self.system.names(&mut names, 0)?;
        p.ensure_names(&names)?;
        let (index, _) = p.chunk(SYSTEM)?;
        let class = p.class(&self.system.encode(&p)?)?;
        p.replace_chunk(index, class)?;
        p.write(s, NODE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cosmetic_snapshot_rebinds_names_and_preserves_other_chunks() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut original = Save::load(std::path::Path::new("saveExample")).unwrap();
        let mut p = Package::from_save(&original, NODE).unwrap();
        let additions = vec![
            "preset_test_mantis".into(),
            "preset_test_color".into(),
            "Uint32".into(),
            "switcherName".into(),
            "CName".into(),
            "savedData".into(),
            DATA.into(),
            "customizationItemID".into(),
            "gameItemID".into(),
            "id".into(),
            "TweakDBID".into(),
            "rngSeed".into(),
        ];
        p.ensure_names(&additions).unwrap();
        let (index, mut system) = p.chunk(SYSTEM).unwrap();
        let mut data = system
            .field(&p, "savedData")
            .map(|f| p.class(&f.data).unwrap())
            .unwrap_or(Class { fields: vec![] });
        data.set(
            &p,
            "switcherName",
            "CName",
            p.name_id("preset_test_mantis")
                .unwrap()
                .to_le_bytes()
                .to_vec(),
        )
        .unwrap();
        data.set(
            &p,
            "preset_test_color",
            "Uint32",
            123u32.to_le_bytes().to_vec(),
        )
        .unwrap();
        let mut item = Class { fields: vec![] };
        item.set(
            &p,
            "id",
            "TweakDBID",
            0x1fb75f37c3u64.to_le_bytes().to_vec(),
        )
        .unwrap();
        item.set(
            &p,
            "rngSeed",
            "Uint32",
            1765185633u32.to_le_bytes().to_vec(),
        )
        .unwrap();
        data.set(&p, "customizationItemID", "gameItemID", item.encode())
            .unwrap();
        system.set(&p, "savedData", DATA, data.encode()).unwrap();
        p.replace_chunk(index, system).unwrap();
        p.write(&mut original, NODE).unwrap();
        let snapshot = CosmeticSnapshot::capture(&original).unwrap().unwrap();
        let mut destination = Save::load(std::path::Path::new("saveExample")).unwrap();
        let before = Package::from_save(&destination, NODE).unwrap();
        snapshot.apply(&mut destination).unwrap();
        let after = Package::from_save(&destination, NODE).unwrap();
        for (i, (name, a, b)) in before.chunks.iter().enumerate() {
            if name != SYSTEM {
                let (_, x, y) = after.chunks[i];
                assert_eq!(
                    &before.bytes[*a..*b],
                    &after.bytes[x..y],
                    "unrelated chunk {name} changed"
                );
            }
        }
        let restored = CosmeticSnapshot::capture(&destination).unwrap().unwrap();
        assert_eq!(
            serde_json::to_value(snapshot).unwrap(),
            serde_json::to_value(restored).unwrap()
        );
        assert!(Save::parse(destination.encode().unwrap()).is_ok());
    }
}
