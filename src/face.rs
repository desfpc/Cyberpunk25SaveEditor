//! Compatibility repair for the legacy female facial-cyberware selection.
//!
//! The 2.5 appearance resource still contains cyberware_01, but saves using it
//! can lose the facial-cyberware control at a ripperdoc. Replacing the selection
//! with cyberware_15 restored that control in the reported save. This is an
//! output migration, not a reset of the separate cyberarm cosmetic system.
use crate::{
    appearance::Gender,
    binary::{string, Reader},
    save::Save,
};
use anyhow::Result;
use std::borrow::Cow;

const NODE: &str = "CharacetrCustomization_Appearances";
const RESOURCE: u64 = 6513893019731746558;
const PREFIX: &str = "hx_000_pwa__cyberware_";

// These replacements exist in the installed 2.5 appearance resource. Preserve
// the skin selection; expand legacy default tones where a native tone exists.
const SKINS: &[(&str, &str)] = &[
    ("01_ca_pale", "01_ca_pale_00_warm_ivory"),
    ("02_ca_limestone", "02_ca_limestone_00_beige"),
    ("03_ca_senna", "03_ca_senna_00_amber"),
    ("04_ca_almond", "04_ca_almond_00_umber"),
    ("05_bl_espresso", "05_bl_espresso"),
    ("06_bl_dark", "06_bl_dark"),
    ("01_ca_pale_00_warm_ivory", "01_ca_pale_00_warm_ivory"),
    ("02_ca_limestone_00_beige", "02_ca_limestone_00_beige"),
    ("03_ca_senna_00_amber", "03_ca_senna_00_amber"),
    ("03_ca_senna_01_honey", "03_ca_senna_01_honey"),
    ("03_ca_senna_02_band", "03_ca_senna_02_band"),
    ("04_ca_almond_00_umber", "04_ca_almond_00_umber"),
];

fn selection(value: &str) -> Option<(&str, &str)> {
    let (variant, skin) = value.strip_prefix(PREFIX)?.split_once("__")?;
    if variant.len() != 2
        || !variant.bytes().all(|b| b.is_ascii_digit())
        || !("01"..="16").contains(&variant)
        || !SKINS.iter().any(|(known, _)| *known == skin)
    {
        return None;
    }
    Some((variant, skin))
}

impl Save {
    fn repaired_face_payload(&self) -> Result<Option<Vec<u8>>> {
        if self.appearance_types()?.body != Gender::Female {
            return Ok(None);
        }
        let rows = self.appearances()?;
        let slots = rows
            .iter()
            .filter(|a| {
                a.group.starts_with("Head / ")
                    && !a.group.ends_with(" / morph")
                    && a.label.starts_with("cyberware_")
            })
            .filter(|a| {
                Reader::new(&self.data, a.start - 8).u64().ok() == Some(RESOURCE)
                    && selection(&a.value).is_some()
            })
            .collect::<Vec<_>>();
        if !slots.iter().any(|a| {
            let (variant, _) = selection(&a.value).unwrap();
            variant == "01" || a.label != format!("cyberware_{variant}")
        }) {
            return Ok(None);
        }

        // A partially edited preset may already contain a working selection.
        // Prefer that selection over changing the character to the fallback.
        let existing = slots
            .iter()
            .filter(|a| selection(&a.value).is_some_and(|(v, _)| v != "01"))
            .min_by_key(|a| a.group != "Head / character_customization");
        let value = match existing {
            Some(a) => a.value.clone(),
            None => {
                let legacy = slots.first().unwrap();
                let (_, skin) = selection(&legacy.value).unwrap();
                let (_, replacement) = SKINS.iter().find(|(s, _)| *s == skin).unwrap();
                format!("{PREFIX}15__{replacement}")
            }
        };
        let (variant, _) = selection(&value).unwrap();
        let label = format!("cyberware_{variant}");
        let n = self.node(NODE)?;
        let base = n.offset + 4;
        let mut payload = self.data[base..n.offset + n.size].to_vec();
        // Offsets refer to the original buffer: patch from right to left.
        for a in slots.into_iter().rev() {
            let mut r = Reader::new(&self.data, a.end);
            r.string()?;
            let mut replacement = string(&value);
            replacement.extend(string(&label));
            payload.splice(a.start - base..r.pos - base, replacement);
        }
        Ok(Some(payload))
    }

    pub(crate) fn repair_facial_cyberware(&mut self) -> Result<bool> {
        if let Some(payload) = self.repaired_face_payload()? {
            self.replace_node(NODE, payload)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn normalized_appearance(&self) -> Result<Cow<'_, Self>> {
        // Work on a copy so export and preset capture keep the in-memory undo
        // state and source file intact. Healthy selections remain byte-exact.
        if let Some(payload) = self.repaired_face_payload()? {
            let mut candidate = self.clone();
            candidate.replace_node(NODE, payload)?;
            Ok(Cow::Owned(candidate))
        } else {
            Ok(Cow::Borrowed(self))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::appearance::CharacterPreset;
    use std::path::PathBuf;

    fn sample() -> Save {
        Save::load(std::path::Path::new("saveExample")).unwrap()
    }

    fn set_selection(save: &mut Save, variant: &str, skin: &str) {
        let rows = save.appearances().unwrap();
        let n = save.node(NODE).unwrap();
        let base = n.offset + 4;
        let mut payload = save.data[base..n.offset + n.size].to_vec();
        for a in rows
            .iter()
            .rev()
            .filter(|a| a.group.starts_with("Head / ") && a.label.starts_with("cyberware_"))
        {
            let mut r = Reader::new(&save.data, a.end);
            r.string().unwrap();
            let mut value = string(&format!("{PREFIX}{variant}__{skin}"));
            value.extend(string(&format!("cyberware_{variant}")));
            payload.splice(a.start - base..r.pos - base, value);
        }
        save.replace_node(NODE, payload).unwrap();
    }

    fn assert_selection(save: &Save, variant: &str, skin: &str) {
        let rows = save.appearances().unwrap();
        let slots = rows
            .iter()
            .filter(|a| a.group.starts_with("Head / ") && a.label.starts_with("cyberware_"));
        let mut count = 0;
        for row in slots {
            assert_eq!(row.label, format!("cyberware_{variant}"));
            assert_eq!(row.value, format!("{PREFIX}{variant}__{skin}"));
            count += 1;
        }
        assert!(count >= 2);
    }

    fn assert_other_data_unchanged(before: &Save, after: &Save) {
        assert_eq!(before.nodes.len(), after.nodes.len());
        for (a, b) in before.nodes.iter().zip(&after.nodes) {
            assert_eq!(a.name, b.name);
            if a.name != NODE && a.child == u32::MAX {
                assert_eq!(
                    &before.data[a.offset..a.offset + a.size],
                    &after.data[b.offset..b.offset + b.size],
                    "Changed unrelated node {}",
                    a.name
                );
            }
        }
        let a = before.appearances().unwrap();
        let b = after.appearances().unwrap();
        assert_eq!(a.len(), b.len());
        for (a, b) in a.iter().zip(&b) {
            if a.group.starts_with("Head / ") && a.label.starts_with("cyberware_") {
                continue;
            }
            assert_eq!(
                (&a.group, &a.label, &a.value),
                (&b.group, &b.label, &b.value)
            );
        }
        // Everything following the last changed head entry remains byte-exact,
        // including head morphs, arm/body data, perspectives and tags.
        let tail = |s: &Save| {
            let end = s
                .appearances()
                .unwrap()
                .into_iter()
                .filter(|a| a.group.starts_with("Head / ") && a.label.starts_with("cyberware_"))
                .map(|a| a.end + string(&a.label).len())
                .max()
                .unwrap();
            let n = s.node(NODE).unwrap();
            s.data[end..n.offset + n.size].to_vec()
        };
        assert_eq!(tail(before), tail(after));
    }

    fn folder(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "nightcity-face-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&p).unwrap();
        p
    }

    #[test]
    fn legacy_selection_is_migrated_on_encode_even_without_other_edits() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        for &(skin, replacement) in SKINS {
            let mut broken = sample();
            set_selection(&mut broken, "01", skin);
            // Model an old file loaded without any user edits.
            let raw = broken.encode_normalized().unwrap();
            let broken = Save::parse(raw).unwrap();
            let before = broken.data.clone();
            let fixed = Save::parse(broken.encode().unwrap()).unwrap();
            assert_selection(&fixed, "15", replacement);
            assert_other_data_unchanged(&broken, &fixed);
            assert_eq!(before, broken.data, "Export mutated the source state");
            assert_eq!(fixed.encode().unwrap(), fixed.original);
        }
    }

    #[test]
    fn healthy_game_selection_and_male_appearance_are_unchanged() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut healthy = sample();
        set_selection(&mut healthy, "16", "01_ca_pale");
        let raw = healthy.encode_normalized().unwrap();
        let healthy = Save::parse(raw.clone()).unwrap();
        assert_eq!(healthy.encode().unwrap(), raw);
        assert_selection(&healthy, "16", "01_ca_pale");

        // Do not guess replacements for mod-defined tones or resources.
        let mut custom = sample();
        set_selection(&mut custom, "01", "custom_skin");
        assert!(!custom.repair_facial_cyberware().unwrap());
        set_selection(&mut custom, "01", "01_ca_pale");
        for a in custom
            .appearances()
            .unwrap()
            .into_iter()
            .filter(|a| a.group.starts_with("Head / ") && a.label.starts_with("cyberware_"))
        {
            custom.data[a.start - 8..a.start].copy_from_slice(&123u64.to_le_bytes());
        }
        assert!(!custom.repair_facial_cyberware().unwrap());

        let mut male = sample();
        male.set_body_type(Gender::Male).unwrap();
        let before = male.data.clone();
        assert!(!male.repair_facial_cyberware().unwrap());
        assert_eq!(male.data, before);

        male.set_body_type(Gender::Female).unwrap();
        assert_selection(&male, "15", "03_ca_senna_00_amber");
    }

    #[test]
    fn mixed_old_preset_preserves_the_existing_working_selection() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut mixed = sample();
        set_selection(&mut mixed, "16", "01_ca_pale");
        let row = mixed
            .appearances()
            .unwrap()
            .into_iter()
            .position(|a| a.group == "Head / face" && a.label == "cyberware_16")
            .unwrap();
        mixed
            .set_appearance(row, "hx_000_pwa__cyberware_01__01_ca_pale")
            .unwrap();
        assert!(mixed.repair_facial_cyberware().unwrap());
        assert_selection(&mixed, "16", "01_ca_pale");

        // Older manual edits can change the definition but retain the old label.
        set_selection(&mut mixed, "01", "01_ca_pale");
        let indices = mixed
            .appearances()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, a)| a.label == "cyberware_01")
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for i in indices {
            mixed
                .set_appearance(i, "hx_000_pwa__cyberware_16__01_ca_pale")
                .unwrap();
        }
        assert!(mixed.repair_facial_cyberware().unwrap());
        assert_selection(&mixed, "16", "01_ca_pale");
    }

    #[test]
    fn old_v1_v2_presets_are_migrated_on_apply_and_new_capture() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut broken = sample();
        set_selection(&mut broken, "01", "03_ca_senna_01_honey");
        let n = broken.node(NODE).unwrap();
        let old_payload = broken.data[n.offset + 4..n.offset + n.size].to_vec();
        let captured = serde_json::to_value(broken.character_preset().unwrap()).unwrap();
        assert_ne!(captured["payload"], serde_json::json!(old_payload));

        for version in [1, 2] {
            let mut old = captured.clone();
            old["payload"] = serde_json::json!(old_payload);
            old["version"] = version.into();
            if version == 1 {
                old.as_object_mut()
                    .unwrap()
                    .remove("cyberware_customization");
            }
            let old: CharacterPreset = serde_json::from_value(old).unwrap();
            let mut destination = sample();
            destination.apply_character_preset(&old).unwrap();
            assert_selection(&destination, "15", "03_ca_senna_01_honey");
            let captured: CharacterPreset = serde_json::from_value(captured.clone()).unwrap();
            destination.apply_character_preset(&captured).unwrap();
            assert_selection(&destination, "15", "03_ca_senna_01_honey");
        }
        assert_selection(&broken, "01", "03_ca_senna_01_honey");
    }

    #[test]
    fn export_and_replacement_verify_migrated_data_and_back_up_original() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let root = folder("export");
        let source = root.join("ManualSave-1");
        std::fs::create_dir(&source).unwrap();
        let mut broken = sample();
        set_selection(&mut broken, "01", "01_ca_pale");
        let original = broken.encode_normalized().unwrap();
        std::fs::write(source.join("sav.dat"), &original).unwrap();
        let loaded = Save::load(&source).unwrap();
        let copy = root.join("ManualSave-2");
        loaded.export(&copy).unwrap();
        assert_selection(
            &Save::load(&copy).unwrap(),
            "15",
            "01_ca_pale_00_warm_ivory",
        );
        assert_eq!(std::fs::read(source.join("sav.dat")).unwrap(), original);
        let backup = loaded.replace_original().unwrap();
        assert_selection(
            &Save::load(&source).unwrap(),
            "15",
            "01_ca_pale_00_warm_ivory",
        );
        assert_eq!(std::fs::read(backup.join("sav.dat")).unwrap(), original);
        assert!(
            loaded.replace_original().is_err(),
            "Stale source must be refused"
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
