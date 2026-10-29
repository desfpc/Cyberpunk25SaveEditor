//! Read-only inventory presentation. No item payloads or statistics are changed.
use crate::save::tdb;
use serde::Deserialize;
use std::{collections::HashMap, sync::OnceLock};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemCategory {
    #[default]
    All,
    Ranged,
    Melee,
    WeaponMods,
    Ammo,
    Cyberware,
    Clothing,
    Consumables,
    Crafting,
    Quickhacks,
    Junk,
    Other,
    Unknown,
}
impl ItemCategory {
    pub const ALL: [Self; 13] = [
        Self::All,
        Self::Ranged,
        Self::Melee,
        Self::WeaponMods,
        Self::Ammo,
        Self::Cyberware,
        Self::Clothing,
        Self::Consumables,
        Self::Crafting,
        Self::Quickhacks,
        Self::Junk,
        Self::Other,
        Self::Unknown,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::All => "Все предметы",
            Self::Ranged => "Огнестрельное оружие",
            Self::Melee => "Оружие ближнего боя",
            Self::WeaponMods => "Модификации и обвесы",
            Self::Ammo => "Боеприпасы",
            Self::Cyberware => "Импланты",
            Self::Clothing => "Одежда",
            Self::Consumables => "Расходники",
            Self::Crafting => "Компоненты и чертежи",
            Self::Quickhacks => "Скрипты",
            Self::Junk => "Мусор",
            Self::Other => "Прочее",
            Self::Unknown => "Неизвестные",
        }
    }
}

#[derive(Deserialize)]
struct Metadata {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    iconic: bool,
    recipe: bool,
}

pub struct ItemDefinition {
    pub record: Option<String>,
    pub name: Option<String>,
    pub kind: String,
    pub category: ItemCategory,
    pub iconic: bool,
    search: String,
}
impl ItemDefinition {
    pub fn title(&self) -> &str {
        self.name
            .as_deref()
            .or(self.record.as_deref())
            .unwrap_or("Unknown item")
    }
    pub fn label(&self, id: u64) -> String {
        let identifier = self
            .record
            .clone()
            .unwrap_or_else(|| format!("TDBID {id:010X}"));
        if self.name.is_some() {
            format!("{} ({identifier})", self.title())
        } else {
            identifier
        }
    }
}

fn classify(kind: &str, recipe: bool) -> ItemCategory {
    if recipe || kind == "Gen_CraftingMaterial" {
        ItemCategory::Crafting
    } else if kind == "Con_Ammo" {
        ItemCategory::Ammo
    } else if matches!(
        kind,
        "Wea_Katana"
            | "Wea_Axe"
            | "Wea_OneHandedClub"
            | "Wea_TwoHandedClub"
            | "Wea_Melee"
            | "Wea_Chainsword"
            | "Wea_Fists"
            | "Wea_Knife"
            | "Wea_Machete"
            | "Wea_LongBlade"
            | "Wea_Hammer"
            | "Wea_Sword"
    ) {
        ItemCategory::Melee
    } else if kind.starts_with("Wea_") {
        ItemCategory::Ranged
    } else if kind == "Prt_Program" {
        ItemCategory::Quickhacks
    } else if kind.starts_with("Cyb_") || kind.starts_with("Cyberware") {
        ItemCategory::Cyberware
    } else if kind.starts_with("Clo_") {
        ItemCategory::Clothing
    } else if kind.starts_with("Con_") || kind == "Gad_Grenade" {
        ItemCategory::Consumables
    } else if kind == "Gen_Junk" || kind == "Gen_Jewellery" {
        ItemCategory::Junk
    } else if kind.starts_with("Prt_") && !kind.contains("FabricEnhancer") {
        ItemCategory::WeaponMods
    } else {
        ItemCategory::Other
    }
}

fn catalog() -> &'static HashMap<u64, ItemDefinition> {
    static CATALOG: OnceLock<HashMap<u64, ItemDefinition>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let names: Vec<String> = serde_json::from_str(include_str!("../assets/items.json"))
            .expect("Valid item record dictionary");
        let metadata: HashMap<String, Metadata> =
            serde_json::from_str(include_str!("../assets/item_metadata.json"))
                .expect("Valid item metadata dictionary");
        let mut result = HashMap::new();
        for record in names {
            result.insert(
                tdb(&record),
                ItemDefinition {
                    search: record.to_lowercase(),
                    record: Some(record),
                    name: None,
                    kind: String::new(),
                    category: ItemCategory::Unknown,
                    iconic: false,
                },
            );
        }
        for (record, meta) in metadata {
            let (id, record) = if let Some(hex) = record.strip_prefix("TDBID ") {
                (
                    u64::from_str_radix(hex, 16).expect("Valid metadata TDBID"),
                    None,
                )
            } else {
                (tdb(&record), Some(record))
            };
            let name = (!meta.name.is_empty()).then_some(meta.name);
            let search = format!(
                "{} {}",
                name.as_deref().unwrap_or(""),
                record.as_deref().unwrap_or("")
            )
            .to_lowercase();
            result.insert(
                id,
                ItemDefinition {
                    record,
                    name,
                    search,
                    category: classify(&meta.kind, meta.recipe),
                    kind: meta.kind,
                    iconic: meta.iconic,
                },
            );
        }
        // Existing supported stack records are absent from the upstream name dictionary.
        for (record, name) in [
            ("Items.HandgunAmmo", "Handgun Ammo"),
            ("Items.RifleAmmo", "Rifle Ammo"),
            ("Items.ShotgunAmmo", "Shotgun Ammo"),
            ("Items.SniperRifleAmmo", "Sniper Rifle Ammo"),
        ] {
            result.insert(
                tdb(record),
                ItemDefinition {
                    record: Some(record.into()),
                    name: Some(name.into()),
                    search: format!("{name} {record}").to_lowercase(),
                    kind: "Ammo".into(),
                    category: ItemCategory::Ammo,
                    iconic: false,
                },
            );
        }
        result
    })
}

pub fn get(id: u64) -> Option<&'static ItemDefinition> {
    catalog().get(&id)
}
pub fn label(id: u64) -> String {
    get(id)
        .map(|item| item.label(id))
        .unwrap_or_else(|| format!("TDBID {id:010X}"))
}

#[derive(Default)]
pub struct ItemFilter {
    pub category: ItemCategory,
    pub iconic_only: bool,
    pub quest_only: bool,
    pub search: String,
}
impl ItemFilter {
    pub fn matches(&self, id: u64, quest: bool) -> bool {
        let entry = get(id);
        let category = entry.map_or(ItemCategory::Unknown, |entry| entry.category);
        (self.category == ItemCategory::All || category == self.category)
            && (!self.iconic_only || entry.is_some_and(|entry| entry.iconic))
            && (!self.quest_only || quest)
            && self.search.split_whitespace().all(|word| {
                let word = word.to_lowercase();
                entry.is_some_and(|entry| entry.search.contains(&word))
                    || format!("tdbid {id:010x}").contains(word.trim_start_matches("0x"))
                    || id.to_string() == word
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_catalog_names_types_and_iconic_flags() {
        for (record, name, category) in [
            (
                "Items.Preset_Liberty_Yorinobu",
                "Kongou",
                ItemCategory::Ranged,
            ),
            ("Items.Preset_Katana_Saburo", "Satori", ItemCategory::Melee),
            ("Items.money", "Eurodollars", ItemCategory::Other),
            (
                "Items.LegendaryMaterial1",
                "Tier 5 Item Components",
                ItemCategory::Crafting,
            ),
        ] {
            let item = get(tdb(record)).unwrap();
            assert_eq!(item.name.as_deref(), Some(name));
            assert_eq!(item.category, category);
            assert_eq!(item.label(tdb(record)), format!("{name} ({record})"));
        }
        assert!(get(tdb("Items.Preset_Liberty_Yorinobu")).unwrap().iconic);
        assert_eq!(
            get(tdb("Items.CommonAchillesRecipe")).unwrap().category,
            ItemCategory::Crafting
        );
        assert_eq!(
            get(tdb("Items.HandgunAmmo")).unwrap().category,
            ItemCategory::Ammo
        );
        assert_eq!(label(u64::MAX), "TDBID FFFFFFFFFFFFFFFF");
    }

    #[test]
    fn combined_filters_and_search_preserve_instance_identity() {
        let kongou = tdb("Items.Preset_Liberty_Yorinobu");
        let mut filter = ItemFilter {
            category: ItemCategory::Ranged,
            iconic_only: true,
            quest_only: true,
            search: "KONGOU yorinobu".into(),
        };
        assert!(filter.matches(kongou, true));
        assert!(!filter.matches(kongou, false));
        assert!(!filter.matches(tdb("Items.Preset_Katana_Saburo"), true));
        filter.search = format!("0x{kongou:X}");
        assert!(filter.matches(kongou, true));
        filter.search = kongou.to_string();
        assert!(filter.matches(kongou, true));
        let unknown = ItemFilter {
            category: ItemCategory::Unknown,
            search: "FFFFFFFF".into(),
            ..Default::default()
        };
        assert!(unknown.matches(u64::MAX, false));
        assert!(!unknown.matches(kongou, false));
    }
}
