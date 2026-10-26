//! Cyberarm appearance repair for the verified 2.5 resource layout.
use crate::{
    appearance::Gender,
    binary::{string, Reader},
    save::Save,
};
use anyhow::{ensure, Result};
use std::collections::HashMap;

const NODE: &str = "CharacetrCustomization_Appearances";
#[derive(Clone)]
struct ArmAppearance {
    resource: u64,
    definition: String,
    name: String,
    flags: Vec<u8>,
}
#[derive(Clone)]
struct ArmGroup {
    name: String,
    appearances: Vec<ArmAppearance>,
    morphs: Vec<u8>,
}
fn count(r: &mut Reader<'_>) -> Result<u32> {
    let n = r.u32()?;
    ensure!(n < 1000, "Invalid arm appearance count");
    Ok(n)
}
fn read_groups(r: &mut Reader<'_>) -> Result<Vec<ArmGroup>> {
    let mut groups = Vec::new();
    for _ in 0..count(r)? {
        let name = r.string()?;
        let mut appearances = Vec::new();
        for _ in 0..count(r)? {
            appearances.push(ArmAppearance {
                resource: r.u64()?,
                definition: r.string()?,
                name: r.string()?,
                flags: r.bytes(8)?.to_vec(),
            });
        }
        let start = r.pos;
        for _ in 0..count(r)? {
            r.string()?;
            r.string()?;
            r.bytes(8)?;
        }
        groups.push(ArmGroup {
            name,
            appearances,
            morphs: r.data[start..r.pos].to_vec(),
        });
    }
    Ok(groups)
}
fn definitions() -> &'static HashMap<String, Vec<String>> {
    static CATALOG: std::sync::OnceLock<HashMap<String, Vec<String>>> = std::sync::OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/cyberarm_definitions_25.json"))
            .expect("Valid bundled cyberarm definitions")
    })
}
fn normalize_definition(resource: u64, definition: &str, body: Gender) -> Result<String> {
    let Some(allowed) = definitions().get(&resource.to_string()) else {
        return Ok(definition.to_owned());
    };
    let mut result = definition.to_owned();
    // These legacy names are dismemberment configurations, not appearance definitions.
    for legacy in [
        "_a0_000_strong_arms__",
        "_a0_000_mantisblade__",
        "_a0_000_personal_link__",
    ] {
        if !allowed.contains(&result) {
            result = result.replace(legacy, "__");
        }
    }
    let (from, to, old_suffix, new_suffix) = match body {
        Gender::Female => ("male_", "female_", "&Male", "&Female"),
        Gender::Male => ("female_", "male_", "&Female", "&Male"),
    };
    if let Some(tail) = result.strip_prefix(from) {
        result = format!("{to}{tail}");
    }
    if result.ends_with(old_suffix) {
        result = format!("{}{new_suffix}", result.trim_end_matches(old_suffix));
    }
    // Some native male slots intentionally use pwa nail resources. Those are
    // selected by the game's slot schema, not by a blanket pma/pwa replacement.
    if result == "holstered_arms_data_male" && body == Gender::Female {
        result = "holstered_arms_data_female".into();
    }
    if result == "holstered_arms_data_female" && body == Gender::Male {
        result = "holstered_arms_data_male".into();
    }
    ensure!(allowed.contains(&result), "Unsupported cyberarm appearance {definition} for resource {resource}; no verified counterpart for {}", body.name());
    Ok(result)
}
fn encode_groups(groups: Vec<ArmGroup>) -> Vec<u8> {
    let mut arms = (groups.len() as u32).to_le_bytes().to_vec();
    for group in groups {
        arms.extend(string(&group.name));
        arms.extend((group.appearances.len() as u32).to_le_bytes());
        for a in group.appearances {
            arms.extend(a.resource.to_le_bytes());
            arms.extend(string(&a.definition));
            arms.extend(string(&a.name));
            arms.extend(a.flags);
        }
        arms.extend(group.morphs);
    }
    arms
}
fn arm_groups(s: &Save) -> Result<(usize, usize, Vec<ArmGroup>)> {
    let n = s.node(NODE)?;
    let payload = &s.data[n.offset + 4..n.offset + n.size];
    let mut r = Reader::new(payload, 11);
    read_groups(&mut r)?;
    let start = r.pos;
    let groups = read_groups(&mut r)?;
    Ok((start, r.pos, groups))
}
type Perspective = (String, String, String);
struct ArmTemplate {
    groups: Vec<ArmGroup>,
    perspectives: Vec<Perspective>,
}
fn read_perspectives(r: &mut Reader<'_>) -> Result<Vec<Perspective>> {
    (0..count(r)?)
        .map(|_| Ok((r.string()?, r.string()?, r.string()?)))
        .collect()
}
fn encode_perspectives(values: &[Perspective]) -> Vec<u8> {
    let mut out = (values.len() as u32).to_le_bytes().to_vec();
    for (name, fpp, tpp) in values {
        out.extend(string(name));
        out.extend(string(fpp));
        out.extend(string(tpp));
    }
    out
}
fn arm_template(body: Gender) -> &'static ArmTemplate {
    static MALE: std::sync::OnceLock<ArmTemplate> = std::sync::OnceLock::new();
    static FEMALE: std::sync::OnceLock<ArmTemplate> = std::sync::OnceLock::new();
    let cell = if body == Gender::Male { &MALE } else { &FEMALE };
    cell.get_or_init(|| {
        let payload =
            crate::appearance::default_appearance(&[0; 11], body).expect("Valid bundled preset");
        let mut r = Reader::new(&payload, 11);
        read_groups(&mut r).expect("Valid bundled head groups");
        let groups = read_groups(&mut r).expect("Valid bundled arm groups");
        read_groups(&mut r).expect("Valid bundled body groups");
        let perspectives = read_perspectives(&mut r).expect("Valid bundled perspectives");
        ArmTemplate {
            groups,
            perspectives,
        }
    })
}
fn canonical_label(label: &str) -> &str {
    label
        .strip_suffix("_fpp")
        .or_else(|| label.strip_suffix("_tpp"))
        .unwrap_or(label)
}
fn legacy_group(name: &str, template: &ArmTemplate) -> bool {
    template.groups.iter().any(|g| {
        g.name.strip_suffix("_fpp") == Some(name) || g.name.strip_suffix("_tpp") == Some(name)
    })
}
fn upgrade_group(source: &ArmGroup, template: &ArmGroup, body: Gender) -> Result<ArmGroup> {
    let mut appearances = Vec::new();
    let mut used = std::collections::HashSet::new();
    for expected in &template.appearances {
        let matches: Vec<_> = source
            .appearances
            .iter()
            .enumerate()
            .filter(|(_, a)| canonical_label(&a.name) == canonical_label(&expected.name))
            .collect();
        ensure!(
            matches.len() <= 1,
            "Ambiguous cyberarm slot {}",
            expected.name
        );
        let mut appearance = expected.clone();
        if let Some(&(index, original)) = matches.first() {
            used.insert(index);
            appearance.definition = original.definition.clone();
            appearance.flags = original.flags.clone();
            // Known vanilla FPP/TPP slots use different resources. Unknown mod
            // resources are retained rather than being replaced with vanilla ones.
            if !definitions().contains_key(&original.resource.to_string()) {
                appearance.resource = original.resource;
            }
        }
        // Follow the native slot, including the pwa nail variant intentionally
        // used by male Mantis Blades. Retain the chosen nail color/skin tone.
        if definitions().contains_key(&appearance.resource.to_string()) {
            if expected.definition.contains("_pwa_") {
                appearance.definition = appearance.definition.replace("_pma_", "_pwa_");
            } else if expected.definition.contains("_pma_") {
                appearance.definition = appearance.definition.replace("_pwa_", "_pma_");
            }
        }
        appearance.definition =
            normalize_definition(appearance.resource, &appearance.definition, body)?;
        appearances.push(appearance);
    }
    for (i, original) in source.appearances.iter().enumerate() {
        if used.contains(&i) {
            continue;
        }
        let vanilla = arm_template(Gender::Male)
            .groups
            .iter()
            .chain(&arm_template(Gender::Female).groups)
            .flat_map(|g| &g.appearances)
            .any(|a| canonical_label(&a.name) == canonical_label(&original.name));
        // A legacy combined group contains FPP-only slots (e.g. personal link)
        // which must not be copied to its new TPP counterpart.
        if vanilla {
            continue;
        }
        let mut extra = original.clone();
        extra.definition = normalize_definition(extra.resource, &extra.definition, body)?;
        appearances.push(extra);
    }
    Ok(ArmGroup {
        name: template.name.clone(),
        appearances,
        morphs: source.morphs.clone(),
    })
}
impl Save {
    pub(crate) fn preserve_cyberarm_cosmetics(&mut self, source: &Save) -> Result<()> {
        let body = self.appearance_types()?.body;
        let (_, _, original) = arm_groups(source)?;
        let (start, end, mut target) = arm_groups(self)?;
        for mut group in original
            .into_iter()
            .filter(|g| g.name.starts_with("elemental_") || g.name == "glamdoc")
        {
            for a in &mut group.appearances {
                // Unknown gendered mod resources need an explicit mapping, never a guessed path.
                ensure!(
                    definitions().contains_key(&a.resource.to_string())
                        || !(a.definition.contains("male")
                            || a.definition.contains("Female")
                            || a.definition.contains("Male")),
                    "No verified gender mapping for custom cyberarm resource {}",
                    a.resource
                );
                a.definition = normalize_definition(a.resource, &a.definition, body)?;
            }
            if let Some(existing) = target.iter_mut().find(|g| g.name == group.name) {
                *existing = group;
            } else {
                target.push(group);
            }
        }
        let n = self.node(NODE)?;
        let payload = &self.data[n.offset + 4..n.offset + n.size];
        let mut result = payload[..start].to_vec();
        result.extend(encode_groups(target));
        result.extend(&payload[end..]);
        self.replace_node(NODE, result)
    }
    /// Repairs appearance names, slot schemas and camera mappings. All other
    /// appearance sections, equipment and cosmetic selections are retained.
    pub fn repair_cyberarm_appearance(&mut self) -> Result<()> {
        let body = self.appearance_types()?.body;
        let reference = arm_template(body);
        let n = self.node(NODE)?;
        let payload = &self.data[n.offset + 4..n.offset + n.size];
        let mut r = Reader::new(payload, 11);
        read_groups(&mut r)?;
        let start = r.pos;
        let original = read_groups(&mut r)?;
        let arms_end = r.pos;
        read_groups(&mut r)?; // Preserve the body section byte for byte.
        let perspectives_start = r.pos;
        let mut perspectives = read_perspectives(&mut r)?;
        let perspectives_end = r.pos;
        let mut groups = Vec::new();
        for group in &original {
            if reference.groups.iter().any(|g| g.name == group.name) {
                let expected = reference
                    .groups
                    .iter()
                    .find(|g| g.name == group.name)
                    .unwrap();
                groups.push(upgrade_group(group, expected, body)?);
            } else if !legacy_group(&group.name, reference) {
                groups.push(group.clone());
            }
        }
        for expected in &reference.groups {
            if !groups.iter().any(|g| g.name == expected.name) {
                let legacy = expected
                    .name
                    .strip_suffix("_fpp")
                    .or_else(|| expected.name.strip_suffix("_tpp"));
                let source = legacy.and_then(|name| original.iter().find(|g| g.name == name));
                groups.push(match source {
                    Some(source) => upgrade_group(source, expected, body)?,
                    None => expected.clone(),
                });
            }
        }
        // Known mappings are replaced, and missing ones are installed. Unknown
        // mappings are preserved; obsolete default-arm routing on male V is removed.
        if body == Gender::Male {
            perspectives.retain(|p| p.0 != "holstered_default");
        }
        for expected in &reference.perspectives {
            if let Some(current) = perspectives.iter_mut().find(|p| p.0 == expected.0) {
                *current = expected.clone();
            } else {
                perspectives.push(expected.clone());
            }
        }
        let mut result = payload[..start].to_vec();
        result.extend(encode_groups(groups));
        result.extend(&payload[arms_end..perspectives_start]);
        result.extend(encode_perspectives(&perspectives));
        result.extend(&payload[perspectives_end..]);
        if result != payload {
            self.replace_node(NODE, result)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Save {
        Save::load(std::path::Path::new("saveExample")).unwrap()
    }
    #[test]
    fn legacy_arm_repair_restores_elements_without_touching_other_appearance_or_nodes() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        for body in [Gender::Male, Gender::Female] {
            let mut save = sample();
            save.set_body_type(body).unwrap();
            let (start, end, mut groups) = arm_groups(&save).unwrap();
            let n = save.node(NODE).unwrap();
            let payload = save.data[n.offset + 4..n.offset + n.size].to_vec();
            groups.retain(|g| {
                !g.name.starts_with("elemental_") && g.name != "nails" && g.name != "glamdoc"
            });
            for g in &mut groups {
                for a in &mut g.appearances {
                    if a.name.contains("strong_arms_colors") {
                        a.definition = a.definition.replace("__", "_a0_000_strong_arms__");
                    }
                    if a.name.contains("mantis_arms_colors") {
                        a.definition = a.definition.replace("__", "_a0_000_mantisblade__");
                    }
                }
            }
            let mut damaged = payload[..start].to_vec();
            damaged.extend(encode_groups(groups));
            damaged.extend(&payload[end..]);
            save.replace_node(NODE, damaged).unwrap();
            let before = save.clone();
            save.repair_cyberarm_appearance().unwrap();
            let (a, b, groups) = arm_groups(&save).unwrap();
            let n = save.node(NODE).unwrap();
            let repaired = &save.data[n.offset + 4..n.offset + n.size];
            assert_eq!(&repaired[..a], &payload[..start]);
            assert_eq!(&repaired[b..], &payload[end..]);
            assert_eq!(
                groups
                    .iter()
                    .filter(|g| g.name.starts_with("elemental_"))
                    .count(),
                16
            );
            for group in groups {
                for appearance in group.appearances {
                    assert!(definitions()[&appearance.resource.to_string()]
                        .contains(&appearance.definition));
                }
            }
            for (n, after) in before.nodes.iter().zip(&save.nodes) {
                if n.child == u32::MAX && n.name != NODE {
                    assert_eq!(
                        &before.data[n.offset..n.offset + n.size],
                        &save.data[after.offset..after.offset + after.size],
                        "{} changed",
                        n.name
                    );
                }
            }
            // A preset written by 0.1.3 may contain the same damaged references.
            let mut legacy = serde_json::to_value(before.character_preset().unwrap()).unwrap();
            legacy["version"] = 1.into();
            legacy
                .as_object_mut()
                .unwrap()
                .remove("cyberware_customization");
            let legacy: crate::appearance::CharacterPreset =
                serde_json::from_value(legacy).unwrap();
            let mut imported = before.clone();
            imported.apply_character_preset(&legacy).unwrap();
            let n = imported.node(NODE).unwrap();
            let fixed_node = save.node(NODE).unwrap();
            assert!(
                imported.data[n.offset + 4..n.offset + n.size]
                    == save.data[fixed_node.offset + 4..fixed_node.offset + fixed_node.size]
            );
            let repaired = save.data.clone();
            save.repair_cyberarm_appearance().unwrap();
            assert_eq!(repaired, save.data);
        }
    }
    #[test]
    fn gender_mapping_preserves_elemental_cyberware_styles() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut save = sample();
        let (start, end, mut groups) = arm_groups(&save).unwrap();
        groups
            .iter_mut()
            .find(|g| g.name == "elemental_nanowire_base")
            .unwrap()
            .appearances[0]
            .definition = "female_6thstreet_base".into();
        let n = save.node(NODE).unwrap();
        let payload = save.data[n.offset + 4..n.offset + n.size].to_vec();
        let mut payload_new = payload[..start].to_vec();
        payload_new.extend(encode_groups(groups));
        payload_new.extend(&payload[end..]);
        save.replace_node(NODE, payload_new).unwrap();
        for (body, expected) in [
            (Gender::Male, "male_6thstreet_base"),
            (Gender::Female, "female_6thstreet_base"),
        ] {
            save.set_body_type(body).unwrap();
            let (_, _, groups) = arm_groups(&save).unwrap();
            assert_eq!(
                groups
                    .iter()
                    .find(|g| g.name == "elemental_nanowire_base")
                    .unwrap()
                    .appearances[0]
                    .definition,
                expected
            );
        }
    }
    #[test]
    fn every_bundled_arm_definition_is_a_real_25_appearance() {
        for json in [
            include_str!("../assets/appearance_male.json"),
            include_str!("../assets/appearance_female.json"),
        ] {
            let value: serde_json::Value = serde_json::from_str(json).unwrap();
            for g in value["Preset"]["armsGroups"].as_array().unwrap() {
                for a in g["customization"].as_array().unwrap() {
                    let resource = a["resource"]["DepotPath"].as_u64().unwrap().to_string();
                    assert!(
                        definitions()[&resource]
                            .iter()
                            .any(|d| d == a["definition"].as_str().unwrap()),
                        "Invalid appearance {}",
                        a["definition"]
                    );
                }
            }
        }
    }
    #[test]
    fn male_camera_routes_survive_upgrade_from_combined_legacy_groups() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut save = sample();
        save.set_body_type(Gender::Male).unwrap();
        let (start, end, mut groups) = arm_groups(&save).unwrap();
        let n = save.node(NODE).unwrap();
        let payload = save.data[n.offset + 4..n.offset + n.size].to_vec();
        let mut r = Reader::new(&payload, end);
        read_groups(&mut r).unwrap();
        let perspectives_start = r.pos;
        let routes = read_perspectives(&mut r).unwrap();
        let perspectives_end = r.pos;
        assert_eq!(
            routes.len(),
            5,
            "native male layout must route all four cyberarm types"
        );
        groups.retain(|g| !g.name.starts_with("holstered_") || !g.name.ends_with("_tpp"));
        for g in &mut groups {
            if g.name.starts_with("holstered_") && g.name.ends_with("_fpp") {
                g.name = g.name.trim_end_matches("_fpp").into();
            }
            for a in &mut g.appearances {
                a.name = a.name.trim_end_matches("_fpp").into();
            }
        }
        let mut damaged = payload[..start].to_vec();
        damaged.extend(encode_groups(groups));
        damaged.extend(&payload[end..perspectives_start]);
        damaged.extend(encode_perspectives(&routes[..1]));
        damaged.extend(&payload[perspectives_end..]);
        save.replace_node(NODE, damaged).unwrap();
        save.repair_cyberarm_appearance().unwrap();
        let (_, end, groups) = arm_groups(&save).unwrap();
        let n = save.node(NODE).unwrap();
        let payload = &save.data[n.offset + 4..n.offset + n.size];
        let mut r = Reader::new(payload, end);
        read_groups(&mut r).unwrap();
        let routes = read_perspectives(&mut r).unwrap();
        for kind in ["strong", "mantis", "nanowire", "launcher"] {
            let logical = format!("holstered_{kind}");
            let route = routes
                .iter()
                .find(|p| p.0 == logical)
                .expect("missing refresh route");
            assert_eq!(route.1, format!("{logical}_fpp"));
            assert_eq!(route.2, format!("{logical}_tpp"));
            for name in [&route.1, &route.2] {
                assert!(groups.iter().any(|g| g.name == *name));
            }
            assert!(!groups.iter().any(|g| g.name == logical));
        }
        let tpp = groups
            .iter()
            .find(|g| g.name == "holstered_strong_tpp")
            .unwrap();
        assert_eq!(
            tpp.appearances
                .iter()
                .find(|a| a.name == "h_strong_arms_colors_base_tpp")
                .unwrap()
                .resource,
            17608390161697568906
        );
        let unholstered = groups
            .iter()
            .find(|g| g.name == "unholstered_strong")
            .unwrap();
        assert!(unholstered
            .appearances
            .iter()
            .any(|a| a.name == "u_strong_arms_colors_base_fpp"));
        let before = save.data.clone();
        save.repair_cyberarm_appearance().unwrap();
        assert!(before == save.data);
    }
}
