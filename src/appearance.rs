//! Version-277 appearance controls. Body changes reset gender-specific resources and
//! update the player's persistent gender; voice changes touch only the brain flag.
use crate::{
    binary::{put32, string, vlq, Reader},
    save::{fnv64, Save},
};
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};

const NODE: &str = "CharacetrCustomization_Appearances";

/// Lossless appearance snapshot, including resources, morphs, perspectives and tags.
#[derive(Deserialize, Serialize)]
pub struct CharacterPreset {
    format: String,
    version: u32,
    game: u32,
    payload: Vec<u8>,
}

impl CharacterPreset {
    pub fn read(path: &std::path::Path) -> Result<Self> {
        ensure!(
            std::fs::metadata(path)?.len() <= 16 * 1024 * 1024,
            "Preset file is too large"
        );
        let preset: Self = serde_json::from_slice(&std::fs::read(path)?)?;
        preset.validate()?;
        Ok(preset)
    }

    pub fn write(&self, path: &std::path::Path) -> Result<()> {
        use std::io::Write;
        self.validate()?;
        let bytes = serde_json::to_vec_pretty(self)?;
        // Never overwrite a previous snapshot, including automatic backups.
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        Ok(())
    }

    fn validate(&self) -> Result<AppearanceTypes> {
        ensure!(
            self.format == "NightCityCharacterPreset" && self.version == 1 && self.game == 2500,
            "Unsupported character preset format"
        );
        let mut r = Reader::new(&self.payload, 0);
        ensure!(r.u8()? == 1, "No appearance preset");
        r.u32()?;
        ensure!(r.u32()? == 13, "Unsupported appearance version");
        let types = AppearanceTypes {
            body: Gender::from_flag(r.u8()?)?,
            voice: Gender::from_flag(r.u8()?)?,
        };
        for _ in 0..3 {
            let count = r.u32()?;
            ensure!(count < 1000, "Invalid preset group count");
            for _ in 0..count {
                r.string()?;
                let count = r.u32()?;
                ensure!(count < 1000, "Invalid preset appearance count");
                for _ in 0..count {
                    r.u64()?;
                    r.string()?;
                    r.string()?;
                    r.bytes(8)?;
                }
                let count = r.u32()?;
                ensure!(count < 1000, "Invalid preset morph count");
                for _ in 0..count {
                    r.string()?;
                    r.string()?;
                    r.bytes(8)?;
                }
            }
        }
        let count = r.u32()?;
        ensure!(count < 1000, "Invalid preset perspective count");
        for _ in 0..count {
            r.string()?;
            r.string()?;
            r.string()?;
        }
        let count = r.vlq()?;
        ensure!((0..1000).contains(&count), "Invalid preset tag count");
        for _ in 0..count {
            r.string()?;
        }
        ensure!(
            r.pos == self.payload.len(),
            "Unexpected trailing preset data"
        );
        Ok(types)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}
impl Gender {
    fn from_flag(flag: u8) -> Result<Self> {
        match flag {
            0 => Ok(Self::Female),
            1 => Ok(Self::Male),
            _ => bail!("Invalid appearance gender flag"),
        }
    }
    fn flag(self) -> u8 {
        u8::from(self == Self::Male)
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Male => "Male",
            Self::Female => "Female",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppearanceTypes {
    pub body: Gender,
    pub voice: Gender,
}

#[derive(Deserialize)]
struct PresetFile {
    #[serde(rename = "Preset")]
    preset: Preset,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Preset {
    is_male: u8,
    head_groups: Vec<Group>,
    arms_groups: Vec<Group>,
    body_groups: Vec<Group>,
    perspective_info: Vec<Perspective>,
    tags: Tags,
}
#[derive(Deserialize)]
struct Group {
    name: String,
    customization: Vec<Customization>,
    morphs: Vec<Morph>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Customization {
    resource: Resource,
    definition: String,
    name: String,
    censor_flag: String,
    censor_flag_action: String,
}
#[derive(Deserialize)]
struct Resource {
    #[serde(rename = "DepotPath")]
    depot_path: u64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Morph {
    region_name: String,
    target_name: String,
    censor_flag: String,
    censor_flag_action: String,
}
#[derive(Deserialize)]
struct Perspective {
    name: String,
    fpp: String,
    tpp: String,
}
#[derive(Deserialize)]
struct Tags {
    tags: Vec<String>,
}

fn censorship(out: &mut Vec<u8>, flag: &str, action: &str) -> Result<()> {
    let flag = match flag {
        "0" => 0,
        "Censor_Nudity" => 1,
        _ => bail!("Unsupported preset censorship flag"),
    };
    let action = match action {
        "Activate" => 0u32,
        "Deactivate" => 1,
        "EquipItem" => 2,
        "UnequipItem" => 3,
        "Refresh" => 4,
        _ => bail!("Unsupported preset action"),
    };
    out.extend(u32::to_le_bytes(flag));
    out.extend(action.to_le_bytes());
    Ok(())
}
fn groups(out: &mut Vec<u8>, groups: &[Group]) -> Result<()> {
    out.extend((groups.len() as u32).to_le_bytes());
    for group in groups {
        out.extend(string(&group.name));
        out.extend((group.customization.len() as u32).to_le_bytes());
        for item in &group.customization {
            out.extend(item.resource.depot_path.to_le_bytes());
            out.extend(string(&item.definition));
            out.extend(string(&item.name));
            censorship(out, &item.censor_flag, &item.censor_flag_action)?;
        }
        out.extend((group.morphs.len() as u32).to_le_bytes());
        for morph in &group.morphs {
            out.extend(string(&morph.region_name));
            out.extend(string(&morph.target_name));
            censorship(out, &morph.censor_flag, &morph.censor_flag_action)?;
        }
    }
    Ok(())
}
fn default_appearance(header: &[u8], body: Gender) -> Result<Vec<u8>> {
    let raw = match body {
        Gender::Male => include_str!("../assets/appearance_male.json"),
        Gender::Female => include_str!("../assets/appearance_female.json"),
    };
    let preset = serde_json::from_str::<PresetFile>(raw)?.preset;
    ensure!(
        Gender::from_flag(preset.is_male)? == body,
        "Preset gender mismatch"
    );
    // Keep the save's platform, preset format version and independently selected voice.
    let mut out = header.to_vec();
    ensure!(out.len() == 11, "Invalid appearance header");
    out[9] = body.flag();
    groups(&mut out, &preset.head_groups)?;
    groups(&mut out, &preset.arms_groups)?;
    groups(&mut out, &preset.body_groups)?;
    out.extend((preset.perspective_info.len() as u32).to_le_bytes());
    for perspective in &preset.perspective_info {
        out.extend(string(&perspective.name));
        out.extend(string(&perspective.fpp));
        out.extend(string(&perspective.tpp));
    }
    out.extend(vlq(preset.tags.tags.len() as i32));
    for tag in &preset.tags.tags {
        out.extend(string(tag));
    }
    Ok(out)
}
impl Save {
    pub fn character_preset(&self) -> Result<CharacterPreset> {
        self.appearance_types()?;
        let n = self.node(NODE)?;
        let preset = CharacterPreset {
            format: "NightCityCharacterPreset".into(),
            version: 1,
            game: self.game,
            payload: self.data[n.offset + 4..n.offset + n.size].to_vec(),
        };
        preset.validate()?;
        Ok(preset)
    }

    pub fn apply_character_preset(&mut self, preset: &CharacterPreset) -> Result<()> {
        let types = preset.validate()?;
        self.appearance_types()?;
        let mut candidate = self.clone();
        candidate.set_puppet_gender(types.body)?;
        let n = candidate.node(NODE)?;
        let mut payload = preset.payload.clone();
        // Platform and format header belong to the destination save.
        payload[..9].copy_from_slice(&candidate.data[n.offset + 4..n.offset + 13]);
        candidate.replace_node(NODE, payload)?;
        candidate.appearances()?;
        if let Some(metadata) = &mut candidate.metadata {
            metadata["Data"]["metadata"]["bodyGender"] = types.body.name().into();
            metadata["Data"]["metadata"]["brainGender"] = types.voice.name().into();
        }
        *self = candidate;
        Ok(())
    }

    pub fn appearance_types(&self) -> Result<AppearanceTypes> {
        let n = self.node(NODE)?;
        ensure!(n.size >= 15, "Truncated appearance header");
        let mut r = Reader::new(&self.data, n.offset + 4);
        ensure!(r.u8()? == 1, "No appearance preset");
        r.u32()?;
        let version = r.u32()?;
        ensure!(
            version == 13,
            "Unsupported appearance preset version {version}; expected 2.5 version 13"
        );
        Ok(AppearanceTypes {
            body: Gender::from_flag(r.u8()?)?,
            voice: Gender::from_flag(r.u8()?)?,
        })
    }
    pub fn set_voice_type(&mut self, voice: Gender) -> Result<()> {
        let current = self.appearance_types()?;
        if current.voice == voice {
            return Ok(());
        }
        let offset = self.node(NODE)?.offset + 14;
        self.data[offset] = voice.flag();
        if let Some(metadata) = &mut self.metadata {
            metadata["Data"]["metadata"]["brainGender"] = voice.name().into();
        }
        Ok(())
    }
    pub fn set_body_type(&mut self, body: Gender) -> Result<()> {
        let current = self.appearance_types()?;
        if current.body == body {
            return Ok(());
        }
        let mut candidate = self.clone();
        candidate.set_puppet_gender(body)?;
        let n = candidate.node(NODE)?;
        let header = &candidate.data[n.offset + 4..n.offset + 15];
        let payload = default_appearance(header, body)?;
        candidate.replace_node(NODE, payload)?;
        ensure!(
            candidate.appearance_types()?
                == AppearanceTypes {
                    body,
                    voice: current.voice
                },
            "Appearance type verification failed"
        );
        candidate.appearances()?;
        if let Some(metadata) = &mut candidate.metadata {
            metadata["Data"]["metadata"]["bodyGender"] = body.name().into();
        }
        *self = candidate;
        Ok(())
    }
    fn set_puppet_gender(&mut self, body: Gender) -> Result<()> {
        let (size_pos, start, end) = self.persistent_entry("PlayerPuppetPS")?;
        let n = self.node("PersistencySystem2")?;
        let base = n.offset + 4;
        let mut payload = self.data[base..n.offset + n.size].to_vec();
        let name = fnv64("gender").to_le_bytes();
        let matches: Vec<_> = self.data[start..end]
            .windows(8)
            .enumerate()
            .filter_map(|(i, x)| (x == name).then_some(start + i))
            .collect();
        ensure!(matches.len() <= 1, "Ambiguous player gender property");
        if let Some(&offset) = matches.first() {
            let mut r = Reader::new(&self.data, offset + 8);
            ensure!(
                r.u64()? == fnv64("CName"),
                "Unsupported player gender property type"
            );
            let value = r.u64()?;
            ensure!(
                r.pos <= end && [fnv64("Male"), fnv64("Female")].contains(&value),
                "Unsupported player gender value"
            );
            payload
                .get_mut(offset + 16 - base..offset + 24 - base)
                .context("Player gender outside persistency entry")?
                .copy_from_slice(&fnv64(body.name()).to_le_bytes());
        } else {
            // Root persistent classes have no terminator. Default-valued properties
            // can be absent; append the known CName property inside this entry only.
            let mut field = name.to_vec();
            field.extend(fnv64("CName").to_le_bytes());
            field.extend(fnv64(body.name()).to_le_bytes());
            payload.splice(end - base..end - base, field);
            put32(&mut payload, size_pos - base, (end - start + 24) as u32)?;
        }
        self.replace_node("PersistencySystem2", payload)
    }
}
