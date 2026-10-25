use nightcity_save_editor::{
    binary::u32_at,
    save::{fact_hash, Save},
};
fn sample() -> Save {
    Save::load(std::path::Path::new("saveExample")).unwrap()
}
#[test]
fn exact_roundtrip() {
    let s = sample();
    assert_eq!(s.game, 2500);
    assert_eq!(s.nodes.len(), 617);
    assert_eq!(s.encode().unwrap(), s.original);
}
#[test]
fn inventory_and_facts() {
    let s = sample();
    let f = s.facts().unwrap();
    assert!(f.len() > 5000);
    assert!(s.items().unwrap().len() > 300);
    println!(
        "facts {} inventory {} appearances {} vehicles {}",
        f.len(),
        s.items().unwrap().len(),
        s.appearances().unwrap().len(),
        s.vehicles().unwrap().len()
    );
}
#[test]
fn patch_fact_and_item() {
    let mut s = sample();
    let f = s.facts().unwrap()[0].clone();
    s.set_fact(f.hash, f.value + 1).unwrap();
    let item = s
        .items()
        .unwrap()
        .into_iter()
        .find(|i| i.quantity.is_some())
        .unwrap();
    let q = item.quantity.unwrap();
    nightcity_save_editor::binary::put32(&mut s.data, q, 999).unwrap();
    let read = Save::parse(s.encode().unwrap()).unwrap();
    assert_eq!(read.facts().unwrap()[0].value, f.value + 1);
    assert_eq!(u32_at(&read.data, q).unwrap(), 999);
}
#[test]
fn add_fact_moves_nodes_preserving_content() {
    let mut s = sample();
    let before = s
        .nodes
        .iter()
        .filter(|n| n.child == u32::MAX && n.name != "FactsTable")
        .map(|n| (n.name.clone(), s.data[n.offset..n.offset + n.size].to_vec()))
        .collect::<Vec<_>>();
    let hash = fact_hash("nightcity_editor_test_unique_2500");
    s.set_fact(hash, 42).unwrap();
    let r = Save::parse(s.encode().unwrap()).unwrap();
    assert_eq!(
        r.facts()
            .unwrap()
            .iter()
            .find(|f| f.hash == hash)
            .unwrap()
            .value,
        42
    );
    let after = r
        .nodes
        .iter()
        .filter(|n| n.child == u32::MAX && n.name != "FactsTable")
        .map(|n| (n.name.clone(), r.data[n.offset..n.offset + n.size].to_vec()))
        .collect::<Vec<_>>();
    for (a, b) in before.iter().zip(after.iter()) {
        assert!(a == b, "Changed node {}", a.0);
    }
}
#[test]
fn read_and_patch_player() {
    let mut s = sample();
    let stats = s.player_stats().unwrap();
    assert_eq!(stats.iter().find(|s| s.key == "Level").unwrap().value, 27);
    for (g, k, v) in [
        ("Attributes", "TechnicalAbility", 20),
        ("Progression", "Level", 45),
        ("Points", "Primary", 50),
        ("Points", "Attribute", 25),
    ] {
        s.set_player_stat(g, k, v).unwrap();
        let r = Save::parse(s.encode().unwrap()).unwrap();
        assert_eq!(
            r.player_stats()
                .unwrap()
                .iter()
                .find(|s| s.group == g && s.key == k)
                .unwrap()
                .value,
            v
        );
    }
}
#[test]
fn appearance_and_garage() {
    let mut s = sample();
    let a = s.appearances().unwrap();
    assert!(!a.is_empty());
    s.set_appearance(0, &format!("{}_test", a[0].value))
        .unwrap();
    let original = s.vehicles().unwrap();
    let id = nightcity_save_editor::save::tdb("Vehicle.v_sport1_quadra_turbo_player");
    s.set_vehicle(id, true).unwrap();
    let r = Save::parse(s.encode().unwrap()).unwrap();
    assert!(r.vehicles().unwrap().iter().any(|v| v.id == id));
    s.set_vehicle(id, false).unwrap();
    assert_eq!(
        s.vehicles().unwrap().len(),
        original.iter().filter(|v| v.id != id).count()
    );
}
#[test]
fn malformed_input_returns_error() {
    let s = sample();
    for len in [0, 4, 20, 100, 500, s.original.len() - 1] {
        assert!(Save::parse(s.original[..len].to_vec()).is_err());
    }
    let mut b = s.original.clone();
    b[8] = 0;
    assert!(Save::parse(b).is_err());
}
#[test]
fn add_stack_roundtrip() {
    let mut s = sample();
    let inv = s.items().unwrap()[0].inventory;
    let record = nightcity_save_editor::save::STACK_RECORDS
        .iter()
        .find(|r| {
            !s.items()
                .unwrap()
                .iter()
                .any(|i| i.inventory == inv && i.id == nightcity_save_editor::save::tdb(r))
        })
        .unwrap();
    s.add_stack(inv, record, 123).unwrap();
    let r = Save::parse(s.encode().unwrap()).unwrap();
    assert_eq!(r.items().unwrap().len(), 407);
    let added = r
        .items()
        .unwrap()
        .into_iter()
        .find(|i| i.id == nightcity_save_editor::save::tdb(record) && i.inventory == inv)
        .unwrap();
    assert_eq!(u32_at(&r.data, added.quantity.unwrap()).unwrap(), 123);
}
#[test]
fn stats_update_both_systems_only_for_player() {
    use nightcity_save_editor::{binary::string, save::fnv64};
    let mut s = sample();
    let n = s.node("StatsSystem").unwrap();
    let before = s.data[n.offset..n.offset + n.size].to_vec();
    let mut marker = string("gameConstantStatModifier");
    marker.extend(fnv64("TechnicalAbility").to_le_bytes());
    marker.extend(0u32.to_le_bytes());
    marker.extend(17f32.to_le_bytes());
    let hits: Vec<_> = before
        .windows(marker.len())
        .enumerate()
        .filter_map(|(i, b)| (b == marker).then_some(i))
        .collect();
    assert_eq!(hits.len(), 1);
    s.set_player_stat("Attributes", "TechnicalAbility", 20)
        .unwrap();
    let n = s.node("StatsSystem").unwrap();
    let after = &s.data[n.offset..n.offset + n.size];
    let mut expected = before.clone();
    let offset = hits[0] + marker.len() - 4;
    expected[offset..offset + 4].copy_from_slice(&20f32.to_le_bytes());
    assert!(expected == after);
    assert_eq!(
        s.metadata.as_ref().unwrap()["Data"]["metadata"]["technicalAbility"],
        20
    );
}
#[test]
fn export_is_a_complete_copy_and_refuses_overwrite() {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };
    let parent = std::env::temp_dir().join(format!(
        "nightcity-export-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&parent).unwrap();
    let destination = parent.join("ManualSave-900");
    let mut s = sample();
    let original = fs::read("saveExample/sav.dat").unwrap();
    s.set_player_stat("Progression", "Level", 40).unwrap();
    s.set_fact(fact_hash("nightcity_test_export"), 7).unwrap();
    s.export(&destination).unwrap();
    let reopened = Save::load(&destination).unwrap();
    assert!(destination.join("screenshot.png").exists());
    assert_eq!(
        reopened.metadata.as_ref().unwrap()["Data"]["metadata"]["name"],
        "ManualSave-900"
    );
    assert_eq!(
        reopened.metadata.as_ref().unwrap()["Data"]["metadata"]["level"],
        40
    );
    assert_eq!(
        reopened
            .player_stats()
            .unwrap()
            .iter()
            .find(|s| s.key == "Level")
            .unwrap()
            .value,
        40
    );
    assert!(s.export(&destination).is_err());
    assert!(fs::read("saveExample/sav.dat").unwrap() == original);
    fs::remove_dir_all(parent).unwrap();
}
#[test]
fn unsupported_edits_are_atomic() {
    let mut s = sample();
    let before = s.data.clone();
    assert!(s.set_player_stat("Attributes", "Cool", 999).is_err());
    assert!(s.data == before);
    assert!(s.add_stack(1, "Items.NotARealUniqueWeapon", 10).is_err());
    assert!(s.data == before);
}
#[test]
fn item_catalog_resolves_example() {
    let s = sample();
    let items = s.items().unwrap();
    let known = items
        .iter()
        .filter(|i| !i.name.starts_with("TDBID"))
        .count();
    assert!(known > 380, "only {known} item names resolved");
}

#[test]
fn voice_type_changes_only_its_flag_and_metadata() {
    use nightcity_save_editor::appearance::Gender;
    let mut s = sample();
    let types = s.appearance_types().unwrap();
    assert_eq!(types.body, Gender::Female);
    assert_eq!(types.voice, Gender::Female);
    let mut expected = s.data.clone();
    let offset = s.node("CharacetrCustomization_Appearances").unwrap().offset + 14;
    expected[offset] = 1;
    s.set_voice_type(Gender::Male).unwrap();
    assert!(s.data == expected);
    assert_eq!(
        s.metadata.as_ref().unwrap()["Data"]["metadata"]["brainGender"],
        "Male"
    );
    let r = Save::parse(s.encode().unwrap()).unwrap();
    assert_eq!(r.appearance_types().unwrap().voice, Gender::Male);
    assert_eq!(r.appearance_types().unwrap().body, Gender::Female);
    s.set_voice_type(Gender::Female).unwrap();
    assert!(s.data == sample().data);
}

#[test]
fn body_type_roundtrips_both_presets_and_preserves_voice() {
    use nightcity_save_editor::{
        appearance::{AppearanceTypes, Gender},
        binary::Reader,
        save::fnv64,
    };
    fn persistent_entries(s: &Save) -> Vec<(u64, u64, Vec<u8>)> {
        let n = s.node("PersistencySystem2").unwrap();
        let mut r = Reader::new(&s.data, n.offset + 4);
        let count = r.u32().unwrap();
        r.bytes(count as usize * 4 + 4).unwrap();
        let count = r.u32().unwrap();
        let mut rows = Vec::new();
        for _ in 0..count {
            let id = r.u64().unwrap();
            if id == 0 {
                rows.push((id, 0, vec![]));
                continue;
            }
            let kind = r.u64().unwrap();
            let len = r.u32().unwrap();
            rows.push((id, kind, r.bytes(len as usize).unwrap().to_vec()));
        }
        rows
    }
    for voice in [Gender::Male, Gender::Female] {
        let mut s = sample();
        s.set_voice_type(voice).unwrap();
        let untouched: Vec<_> = persistent_entries(&s)
            .into_iter()
            .filter(|(_, kind, _)| *kind != fnv64("PlayerPuppetPS"))
            .collect();
        let original_nodes: Vec<_> = s
            .nodes
            .iter()
            .filter(|n| {
                n.child == u32::MAX
                    && !["PersistencySystem2", "CharacetrCustomization_Appearances"]
                        .contains(&n.name.as_str())
            })
            .map(|n| (n.name.clone(), s.data[n.offset..n.offset + n.size].to_vec()))
            .collect();
        let original_header = s.node("CharacetrCustomization_Appearances").unwrap().offset + 4;
        let header = s.data[original_header..original_header + 9].to_vec();
        for body in [Gender::Male, Gender::Female] {
            s.set_body_type(body).unwrap();
            assert_eq!(
                s.appearance_types().unwrap(),
                AppearanceTypes { body, voice }
            );
            assert!(!s.appearances().unwrap().is_empty());
            let current_header = s.node("CharacetrCustomization_Appearances").unwrap().offset + 4;
            assert_eq!(&s.data[current_header..current_header + 9], header);
            assert_eq!(
                s.metadata.as_ref().unwrap()["Data"]["metadata"]["bodyGender"],
                body.name()
            );
            let rows = persistent_entries(&s);
            let player = &rows
                .iter()
                .find(|(_, kind, _)| *kind == fnv64("PlayerPuppetPS"))
                .unwrap()
                .2;
            let mut signature = fnv64("gender").to_le_bytes().to_vec();
            signature.extend(fnv64("CName").to_le_bytes());
            signature.extend(fnv64(body.name()).to_le_bytes());
            assert_eq!(player.windows(24).filter(|w| *w == signature).count(), 1);
            assert!(
                rows.into_iter()
                    .filter(|(_, kind, _)| *kind != fnv64("PlayerPuppetPS"))
                    .collect::<Vec<_>>()
                    == untouched
            );
            let r = Save::parse(s.encode().unwrap()).unwrap();
            assert_eq!(
                r.appearance_types().unwrap(),
                AppearanceTypes { body, voice }
            );
            let after: Vec<_> = r
                .nodes
                .iter()
                .filter(|n| {
                    n.child == u32::MAX
                        && !["PersistencySystem2", "CharacetrCustomization_Appearances"]
                            .contains(&n.name.as_str())
                })
                .map(|n| (n.name.clone(), r.data[n.offset..n.offset + n.size].to_vec()))
                .collect();
            assert!(original_nodes == after);
        }
    }
}

#[test]
fn body_change_fails_atomically_when_player_persistency_is_missing() {
    use nightcity_save_editor::{appearance::Gender, save::fnv64};
    let mut s = sample();
    let hash = fnv64("PlayerPuppetPS").to_le_bytes();
    let n = s.node("PersistencySystem2").unwrap();
    let offsets: Vec<_> = s.data[n.offset..n.offset + n.size]
        .windows(8)
        .enumerate()
        .filter_map(|(i, b)| (b == hash).then_some(n.offset + i))
        .collect();
    assert_eq!(offsets.len(), 1);
    s.data[offsets[0]..offsets[0] + 8].fill(0);
    let before = s.data.clone();
    let metadata = s.metadata.clone();
    assert!(s.set_body_type(Gender::Male).is_err());
    assert!(s.data == before);
    assert_eq!(s.metadata, metadata);
}
