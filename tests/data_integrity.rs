//! Preservation checks for the writer, including opaque weapon statistics.
//! An additional local save can be audited with NIGHTCITY_AUDIT_SAVE; no personal
//! fixture is included in the repository and all edits stay in memory or /tmp.
use nightcity_save_editor::{
    appearance::Gender,
    binary::{put32, Reader},
    package::Package,
    save::{fact_hash, fnv64, Save, STACK_RECORDS},
};
use std::{collections::BTreeMap, path::PathBuf};

fn fixture() -> Option<Save> {
    let path = std::env::var_os("NIGHTCITY_AUDIT_SAVE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("saveExample"));
    if !path.exists() {
        return None;
    }
    Some(Save::load(&path).unwrap())
}

fn node_bytes(save: &Save, name: &str) -> Vec<u8> {
    let n = save.node(name).unwrap();
    save.data[n.offset + 4..n.offset + n.size].to_vec()
}

fn leaves(save: &Save, except: &[&str]) -> BTreeMap<(String, usize), Vec<u8>> {
    let mut counts = BTreeMap::new();
    let mut out = BTreeMap::new();
    for n in &save.nodes {
        if n.child != u32::MAX || except.contains(&n.name.as_str()) {
            continue;
        }
        let count = counts.entry(n.name.clone()).or_insert(0);
        // Node IDs can change when inserting inventory entries. Their payloads
        // must not change as a side effect of renumbering the node table.
        out.insert(
            (n.name.clone(), *count),
            save.data[n.offset + 4..n.offset + n.size].to_vec(),
        );
        *count += 1;
    }
    out
}

fn script_chunks(save: &Save, except: &[&str]) -> Vec<(String, Vec<u8>)> {
    let p = Package::from_save(save, "ScriptableSystemsContainer").unwrap();
    p.chunks
        .iter()
        .filter(|(name, _, _)| !except.contains(&name.as_str()))
        .map(|(name, start, end)| (name.clone(), p.bytes[*start..*end].to_vec()))
        .collect()
}

fn roundtrip(save: &Save) -> Save {
    Save::parse(save.encode().unwrap()).unwrap()
}

fn assert_rejected_edit_is_atomic(original: &Save, edited: &Save) {
    assert!(original.data == edited.data);
    assert!(original.nodes == edited.nodes);
    assert!(original.metadata == edited.metadata);
}

#[test]
fn player_edits_preserve_all_nonplayer_stats_and_inventory() {
    let Some(original) = fixture() else { return };
    let original_stats = Package::from_save(&original, "StatsSystem").unwrap();
    let (_, root) = original_stats.chunk("gameStatsStateMapStructure").unwrap();
    let rows = original_stats
        .array(&root.field(&original_stats, "valuesOptimized").unwrap().data)
        .unwrap();
    let slices = original_stats
        .array(
            &root
                .field(&original_stats, "sharedBufferSlices")
                .unwrap()
                .data,
        )
        .unwrap();
    let original_buffer = &root
        .field(&original_stats, "sharedDataBuffer")
        .unwrap()
        .data;
    for stat in original.player_stats().unwrap() {
        let mut edited = original.clone();
        let value = if stat.value < stat.max {
            stat.value + 1
        } else {
            stat.value - 1
        };
        edited
            .set_player_stat(&stat.group, &stat.key, value)
            .unwrap();
        let edited = roundtrip(&edited);
        assert!(
            leaves(&original, &["StatsSystem", "ScriptableSystemsContainer"])
                == leaves(&edited, &["StatsSystem", "ScriptableSystemsContainer"])
        );
        assert!(node_bytes(&original, "inventory") == node_bytes(&edited, "inventory"));
        assert!(
            script_chunks(&original, &["PlayerDevelopmentData"])
                == script_chunks(&edited, &["PlayerDevelopmentData"])
        );
        let p = Package::from_save(&edited, "StatsSystem").unwrap();
        let (_, after) = p.chunk("gameStatsStateMapStructure").unwrap();
        assert_eq!(p.names, original_stats.names);
        for field in &root.fields {
            let name = &original_stats.names[field.name as usize];
            if name != "sharedDataBuffer" {
                assert!(
                    field.data == after.field(&p, name).unwrap().data,
                    "Changed stats field {name}"
                );
            }
        }
        let buffer = &after.field(&p, "sharedDataBuffer").unwrap().data;
        assert_eq!(buffer.len(), original_buffer.len());
        let mut permitted = vec![];
        for row in &rows {
            let player = row
                .field(&original_stats, "entityHash")
                .is_some_and(|f| Reader::new(&f.data, 0).u64().unwrap() == 1);
            let Some(field) = row.field(&original_stats, "bufferSliceIndices") else {
                continue;
            };
            let mut r = Reader::new(&field.data, 0);
            for _ in 0..r.u32().unwrap() {
                let slice = &slices[r.u16().unwrap() as usize];
                let combined = slice.integer(&original_stats, "combined").unwrap();
                let start = slice.integer(&original_stats, "offset").unwrap() as usize + 4;
                let end = start + (combined & 0x3fffffff) as usize;
                if player && combined >> 30 == 1 {
                    let mut r = Reader::new(original_buffer, start);
                    while r.pos < end {
                        let kind = r.string().unwrap();
                        let key = r.u64().unwrap();
                        let operation = r.u32().unwrap();
                        match kind.as_str() {
                            "gameConstantStatModifier" => {
                                if stat.group != "Points"
                                    && key == fnv64(&stat.key)
                                    && operation == 0
                                {
                                    permitted.push(r.pos..r.pos + 4);
                                    assert_eq!(
                                        f32::from_bits(Reader::new(buffer, r.pos).u32().unwrap()),
                                        value as f32
                                    );
                                }
                                r.bytes(4).unwrap();
                            }
                            "gameCombinedStatModifier" => {
                                r.bytes(20).unwrap();
                            }
                            "gameCurveStatModifier" => {
                                r.string().unwrap();
                                r.string().unwrap();
                                r.bytes(12).unwrap();
                            }
                            _ => panic!("Unknown modifier {kind}"),
                        }
                    }
                    assert_eq!(r.pos, end);
                    continue; // Only the player's forced modifier may change.
                }
                assert!(
                    buffer[start..end] == original_buffer[start..end],
                    "Changed nonplayer/saved stats while editing {}",
                    stat.key
                );
            }
        }
        assert_eq!(permitted.len(), usize::from(stat.group != "Points"));
        for (offset, (a, b)) in original_buffer.iter().zip(buffer).enumerate() {
            assert!(
                a == b || permitted.iter().any(|r| r.contains(&offset)),
                "Unrelated player stat changed at buffer offset {offset}"
            );
        }
        assert!(edited
            .player_stats()
            .unwrap()
            .iter()
            .any(|s| s.group == stat.group && s.key == stat.key && s.value == value));
    }
}

#[test]
fn appearance_presets_facts_and_garage_preserve_weapon_data() {
    let Some(original) = fixture() else { return };
    let preset = original.character_preset().unwrap();
    for action in 0..8 {
        let mut edited = original.clone();
        let mut allowed = vec![];
        match action {
            0 => {} // Export without edits.
            1 => {
                let voice = if edited.appearance_types().unwrap().voice == Gender::Female {
                    Gender::Male
                } else {
                    Gender::Female
                };
                edited.set_voice_type(voice).unwrap();
                allowed.push("CharacetrCustomization_Appearances");
            }
            2 => {
                let body = if edited.appearance_types().unwrap().body == Gender::Female {
                    Gender::Male
                } else {
                    Gender::Female
                };
                if let Err(error) = edited.set_body_type(body) {
                    assert_rejected_edit_is_atomic(&original, &edited);
                    if error
                        .is::<nightcity_save_editor::cyberware::IncompatibleCyberwareCosmetics>()
                    {
                        // Model explicit UI consent and verify the fallback too.
                        edited.set_body_type_without_cosmetics(body).unwrap();
                        allowed.push("ScriptableSystemsContainer");
                        assert!(
                            script_chunks(&original, &["CyberwareCustomizationSystem"])
                                == script_chunks(&edited, &["CyberwareCustomizationSystem"])
                        );
                    } else {
                        eprintln!("Body change safely rejected: {error:#}");
                        continue;
                    }
                }
                allowed.extend(["CharacetrCustomization_Appearances", "PersistencySystem2"]);
            }
            3 => {
                if let Err(error) = edited.apply_character_preset(&preset) {
                    assert_rejected_edit_is_atomic(&original, &edited);
                    eprintln!("Preset safely rejected: {error:#}");
                    continue;
                }
                allowed.extend([
                    "CharacetrCustomization_Appearances",
                    "PersistencySystem2",
                    "ScriptableSystemsContainer",
                ]);
                assert!(
                    script_chunks(&original, &["CyberwareCustomizationSystem"])
                        == script_chunks(&edited, &["CyberwareCustomizationSystem"])
                );
            }
            4 => {
                edited.repair_cyberarm_appearance().unwrap();
                allowed.push("CharacetrCustomization_Appearances");
            }
            5 => {
                edited
                    .set_fact(fact_hash("nightcity_integrity_audit"), 1)
                    .unwrap();
                allowed.push("FactsTable");
            }
            6 => {
                let vehicle = edited.vehicles().unwrap().remove(0);
                edited.set_vehicle(vehicle.id, false).unwrap();
                allowed.push("PersistencySystem2");
            }
            7 => {
                let vehicle = edited.vehicles().unwrap().remove(0);
                edited.set_vehicle(vehicle.id, false).unwrap();
                edited.set_vehicle(vehicle.id, true).unwrap();
                allowed.push("PersistencySystem2");
            }
            _ => unreachable!(),
        }
        let edited = roundtrip(&edited);
        assert!(
            node_bytes(&original, "StatsSystem") == node_bytes(&edited, "StatsSystem"),
            "Stats changed in action {action}"
        );
        assert!(
            node_bytes(&original, "inventory") == node_bytes(&edited, "inventory"),
            "Inventory changed in action {action}"
        );
        assert!(
            leaves(&original, &allowed) == leaves(&edited, &allowed),
            "Unrelated nodes changed in action {action}"
        );
    }
}

#[test]
fn resource_quantity_and_flags_preserve_other_item_payloads_and_stats() {
    let Some(original) = fixture() else { return };
    for action in 0..4 {
        let mut edited = original.clone();
        let item = edited
            .items()
            .unwrap()
            .into_iter()
            .find(|i| i.quantity.is_some())
            .unwrap();
        match action {
            0 => put32(&mut edited.data, item.quantity.unwrap(), 123).unwrap(),
            1 => edited.data[item.flags] ^= 1,
            2 => edited.add_stack(item.inventory, "Items.money", 1).unwrap(),
            3 => {
                let items = edited.items().unwrap();
                let (inventory, record) = items
                    .iter()
                    .find_map(|container| {
                        STACK_RECORDS
                            .iter()
                            .find(|r| {
                                !items.iter().any(|i| {
                                    i.inventory == container.inventory
                                        && i.id == nightcity_save_editor::save::tdb(r)
                                })
                            })
                            .map(|record| (container.inventory, *record))
                    })
                    .unwrap();
                edited.add_stack(inventory, record, 1).unwrap();
            }
            _ => unreachable!(),
        }
        let edited = roundtrip(&edited);
        assert!(leaves(&original, &["itemData"]) == leaves(&edited, &["itemData"]));
        let item_payloads = |s: &Save| {
            s.items()
                .unwrap()
                .into_iter()
                .filter(|i| i.quantity.is_none())
                .map(|i| {
                    let n = s.nodes.iter().find(|n| n.offset == i.offset).unwrap();
                    (
                        (i.inventory, s.data[i.offset + 4..i.offset + 20].to_vec()),
                        s.data[n.offset + 4..n.offset + n.size].to_vec(),
                    )
                })
                .collect::<BTreeMap<_, _>>()
        };
        assert!(
            item_payloads(&original) == item_payloads(&edited),
            "Equipment changed in action {action}"
        );
    }
}

#[test]
fn repeated_exports_and_backup_replacement_preserve_weapon_data() {
    let Some(original) = fixture() else { return };
    let root = std::env::temp_dir().join(format!(
        "nightcity-writer-audit-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    let source = root.join("ManualSave-1");
    original.export(&source).unwrap();
    let source_bytes = std::fs::read(source.join("sav.dat")).unwrap();
    let mut loaded = Save::load(&source).unwrap();
    loaded.export(&root.join("ManualSave-2")).unwrap();
    assert!(std::fs::read(root.join("ManualSave-2/sav.dat")).unwrap() == source_bytes);
    loaded
        .set_fact(fact_hash("nightcity_writer_audit"), 1)
        .unwrap();
    loaded.export(&root.join("ManualSave-3")).unwrap();
    let exported = Save::load(&root.join("ManualSave-3")).unwrap();
    assert!(node_bytes(&original, "StatsSystem") == node_bytes(&exported, "StatsSystem"));
    assert!(node_bytes(&original, "inventory") == node_bytes(&exported, "inventory"));
    assert!(std::fs::read(source.join("sav.dat")).unwrap() == source_bytes);
    let backup = loaded.replace_original().unwrap();
    assert!(std::fs::read(backup.join("sav.dat")).unwrap() == source_bytes);
    let replaced = Save::load(&source).unwrap();
    assert!(node_bytes(&original, "StatsSystem") == node_bytes(&replaced, "StatsSystem"));
    assert!(node_bytes(&original, "inventory") == node_bytes(&replaced, "inventory"));
    assert!(loaded.replace_original().is_err());
    std::fs::remove_dir_all(root).unwrap();
}
