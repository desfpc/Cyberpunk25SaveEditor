#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod i18n;
mod settings;
use anyhow::{Context, Result};
use eframe::egui::{self, Color32, RichText};
use i18n::Language;
use nightcity_save_editor::{
    appearance::{CharacterPreset, Gender},
    binary::{put32, u32_at},
    cyberware::IncompatibleCyberwareCosmetics,
    package::PlayerStat,
    save::{fact_hash, tdb, Appearance, Fact, Item, Save, Vehicle, STACK_RECORDS},
};
use std::path::PathBuf;
const CYAN: Color32 = Color32::from_rgb(59, 224, 211);
const YELLOW: Color32 = Color32::from_rgb(242, 235, 111);
const MUTED: Color32 = Color32::from_rgb(139, 154, 176);
const BG: Color32 = Color32::from_rgb(12, 17, 26);
#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Player,
    Inventory,
    Appearance,
    Vehicles,
    Facts,
    Tools,
    About,
}
impl Tab {
    fn title(self, lang: Language) -> &'static str {
        lang.text(match self {
            Self::Player => "Персонаж",
            Self::Inventory => "Инвентарь",
            Self::Appearance => "Внешность",
            Self::Vehicles => "Гараж",
            Self::Facts => "Квестовые флаги",
            Self::Tools => "Данные сохранения",
            Self::About => "О нас",
        })
    }
}
struct App {
    settings: settings::Settings,
    replace_save: bool,
    language: Language,
    save: Option<Save>,
    undo: Vec<Save>,
    redo: Vec<Save>,
    tab: Tab,
    search: String,
    status: String,
    error: bool,
    facts: Vec<Fact>,
    items: Vec<Item>,
    stats: Vec<PlayerStat>,
    appearances: Vec<Appearance>,
    vehicles: Vec<Vehicle>,
    catalog: Vec<String>,
    inventory: u64,
    new_fact: String,
    new_value: u32,
    stack: usize,
    stack_qty: u32,
    appearance_index: Option<usize>,
    appearance_value: String,
    dirty: bool,
    clean_data: Vec<u8>,
    pending_open: Option<PathBuf>,
    confirm_close: bool,
    allow_close: bool,
    cache_errors: Vec<String>,
    pending_body_change: Option<Gender>,
}
impl Default for App {
    fn default() -> Self {
        Self {
            settings: settings::Settings::load(),
            replace_save: false,
            language: Language::default(),
            save: None,
            undo: vec![],
            redo: vec![],
            tab: Tab::Player,
            search: String::new(),
            status: "Выберите sav.dat или откройте пример сохранения".into(),
            error: false,
            facts: vec![],
            items: vec![],
            stats: vec![],
            appearances: vec![],
            vehicles: vec![],
            catalog: serde_json::from_str(include_str!("../assets/vehicles.json"))
                .unwrap_or_default(),
            inventory: 1,
            new_fact: String::new(),
            new_value: 1,
            stack: 0,
            stack_qty: 1000,
            appearance_index: None,
            appearance_value: String::new(),
            dirty: false,
            clean_data: Vec::new(),
            pending_open: None,
            confirm_close: false,
            allow_close: false,
            cache_errors: vec![],
            pending_body_change: None,
        }
    }
}
impl App {
    fn report(&mut self, r: Result<()>, success: &str) {
        match r {
            Ok(()) => {
                self.error = false;
                self.status = success.into();
            }
            Err(e) => {
                self.error = true;
                self.status = format!("{e:#}");
            }
        }
    }
    fn refresh(&mut self) {
        self.cache_errors.clear();
        if let Some(s) = &self.save {
            self.dirty = s.data != self.clean_data;
            macro_rules! read {
                ($field:ident,$method:ident) => {
                    match s.$method() {
                        Ok(v) => self.$field = v,
                        Err(e) => {
                            self.$field.clear();
                            self.cache_errors
                                .push(format!("{}: {e:#}", stringify!($method)));
                        }
                    }
                };
            }
            read!(facts, facts);
            read!(items, items);
            read!(stats, player_stats);
            read!(appearances, appearances);
            read!(vehicles, vehicles);
            if !self.items.iter().any(|i| i.inventory == self.inventory) {
                self.inventory = self.items.first().map(|i| i.inventory).unwrap_or(1);
            }
        }
    }
    fn open(&mut self, p: PathBuf) {
        match Save::load(&p) {
            Ok(s) => {
                let directory = s
                    .source
                    .as_ref()
                    .and_then(|p| p.parent())
                    .map(PathBuf::from);
                self.clean_data = s.data.clone();
                self.save = Some(s);
                self.undo.clear();
                self.redo.clear();
                self.dirty = false;
                self.appearance_index = None;
                self.refresh();
                self.status = format!("Загружено: {}", p.display());
                self.error = false;
                self.replace_save = false;
                self.pending_body_change = None;
                if let Some(directory) = directory {
                    if let Err(e) = self.settings.remember(&directory) {
                        self.report(
                            Err(e.context(
                                "Save loaded, but the last folder could not be remembered",
                            )),
                            "",
                        );
                    }
                }
            }
            Err(e) => self.report(Err(e), ""),
        }
    }
    fn edit(&mut self, label: &str, f: impl FnOnce(&mut Save) -> Result<()>) {
        if let Some(s) = &self.save {
            let before = s.clone();
            let mut after = before.clone();
            match f(&mut after) {
                Ok(()) => {
                    if after.data != before.data {
                        self.undo.push(before);
                        if self.undo.len() > 12 {
                            self.undo.remove(0);
                        }
                        self.redo.clear();
                        self.save = Some(after);
                        self.dirty = true;
                        self.refresh();
                    }
                    self.report(Ok(()), label);
                }
                Err(e) => self.report(Err(e), ""),
            }
        }
    }
    fn export(&mut self) {
        let lang = self.language;
        let Some(s) = self.save.as_ref() else { return };
        if self.replace_save {
            let result = s.replace_original().and_then(|backup| {
                let reopened = Save::load(s.source.as_ref().context("No source save")?)?;
                Ok((backup, reopened))
            });
            match result {
                Ok((backup, reopened)) => {
                    self.clean_data = reopened.data.clone();
                    self.save = Some(reopened);
                    self.refresh();
                    self.report(
                        Ok(()),
                        &format!(
                            "{} {}",
                            lang.text("Сохранение заменено. Резервная копия:"),
                            backup.display()
                        ),
                    );
                }
                Err(e) => self.report(Err(e), ""),
            }
            return;
        }
        if let Some(parent) = self
            .save_dialog()
            .set_title(lang.text("Папка для новой копии сохранения"))
            .pick_folder()
        {
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let name = format!("ManualSave-{stamp}");
            let destination = parent.join(name);
            let result = s.export(&destination);
            if result.is_ok() {
                self.clean_data = s.data.clone();
                self.dirty = false;
            }
            self.report(
                result,
                &format!("Копия сохранена: {}", destination.display()),
            );
        }
    }
    fn save_dialog(&self) -> rfd::FileDialog {
        let dialog = rfd::FileDialog::new();
        match &self.settings.last_save_directory {
            Some(path) => dialog.set_directory(path),
            None => dialog,
        }
    }
    fn save_preset(&mut self) {
        let title = self.language.text("Сохранить пресет персонажа");
        let Some(s) = &self.save else { return };
        let gender = s.appearance_types().map(|t| t.body.name()).unwrap_or("V");
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let Some(path) = self
            .save_dialog()
            .set_title(title)
            .add_filter("Night City character preset", &["ncpreset"])
            .set_file_name(format!("V-{gender}-{stamp}.ncpreset"))
            .save_file()
        else {
            return;
        };
        let result = s.character_preset().and_then(|p| p.write(&path));
        self.report(result, self.language.text("Пресет персонажа сохранён"));
    }
    fn load_preset(&mut self) {
        let Some(path) = self
            .save_dialog()
            .set_title(self.language.text("Загрузить пресет персонажа"))
            .add_filter("Night City character preset", &["ncpreset"])
            .pick_file()
        else {
            return;
        };
        let preset = match CharacterPreset::read(&path) {
            Ok(preset) => preset,
            Err(e) => {
                self.report(Err(e), "");
                return;
            }
        };
        self.edit(
            self.language.text("Пресет персонажа загружен"),
            |s| s.apply_character_preset(&preset),
        );
        self.appearance_index = None;
        self.appearance_value.clear();
    }
    fn top(&mut self, ctx: &egui::Context) {
        let lang = self.language;
        egui::TopBottomPanel::top("top")
            .exact_height(76.)
            .show(ctx, |ui| {
                if self.pending_body_change.is_some() {
                    ui.disable();
                }
                ui.horizontal_centered(|ui| {
                    ui.add_space(16.);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("NIGHT CITY").size(23.).strong().color(YELLOW));
                        ui.label(
                            RichText::new("SAVE EDITOR  /  RUST EDITION")
                                .size(10.)
                                .color(MUTED),
                        );
                    });
                    ui.add_space(28.);
                    ui.label(RichText::new(" 2.5 ").strong().color(CYAN));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(16.);
                        if ui
                            .add_enabled(
                                self.save.is_some(),
                                egui::Button::new(
                                    RichText::new(lang.text(if self.replace_save {
                                        "Заменить сохранение"
                                    } else {
                                        "Экспорт копии"
                                    }))
                                    .color(BG)
                                    .strong(),
                                )
                                .fill(CYAN)
                                .min_size(egui::vec2(145., 35.)),
                            )
                            .clicked()
                        {
                            self.export();
                        }
                        if ui
                            .add_sized([140., 35.], egui::Button::new(lang.text("Открыть sav.dat")))
                            .clicked()
                        {
                            if let Some(p) = self
                                .save_dialog()
                                .set_title(lang.text("Открыть sav.dat"))
                                .add_filter("Cyberpunk save", &["dat"])
                                .pick_file()
                            {
                                if self.dirty {
                                    self.pending_open = Some(p);
                                } else {
                                    self.open(p);
                                }
                            }
                        }
                        egui::ComboBox::from_id_salt("language")
                            .width(110.)
                            .selected_text(self.language.label())
                            .show_ui(ui, |ui| {
                                for language in [Language::Russian, Language::English] {
                                    ui.selectable_value(
                                        &mut self.language,
                                        language,
                                        language.label(),
                                    );
                                }
                            });
                    });
                });
            });
    }
    fn sidebar(&mut self, ctx: &egui::Context) {
        let lang = self.language;
        egui::SidePanel::left("navigation")
            .exact_width(224.)
            .resizable(false)
            .show(ctx, |ui| {
                if self.pending_body_change.is_some() {
                    ui.disable();
                }
                ui.add_space(24.);
                ui.label(
                    RichText::new(lang.text(" РЕДАКТОР СОХРАНЕНИЯ"))
                        .size(10.)
                        .color(MUTED),
                );
                ui.add_space(15.);
                for (i, t) in [
                    Tab::Player,
                    Tab::Inventory,
                    Tab::Appearance,
                    Tab::Vehicles,
                    Tab::Facts,
                    Tab::Tools,
                    Tab::About,
                ]
                .into_iter()
                .enumerate()
                {
                    let selected = self.tab == t;
                    let text = RichText::new(format!("{:02}     {}", i + 1, t.title(lang)))
                        .color(if selected { CYAN } else { MUTED })
                        .size(14.);
                    if ui
                        .add_sized(
                            [208., 38.],
                            egui::Button::new(text).fill(if selected {
                                Color32::from_rgb(22, 47, 54)
                            } else {
                                Color32::TRANSPARENT
                            }),
                        )
                        .clicked()
                    {
                        self.tab = t;
                        self.search.clear();
                    }
                    ui.add_space(4.);
                }
                ui.add_space(24.);
                ui.separator();
                ui.add_space(15.);
                ui.add_enabled(
                    self.save.is_some(),
                    egui::Checkbox::new(
                        &mut self.replace_save,
                        lang.text("Заменять сейв (копия в old)"),
                    ),
                );
                if let Some(s) = &self.save {
                    if let Some(m) = s
                        .metadata
                        .as_ref()
                        .and_then(|v| v.pointer("/Data/metadata"))
                    {
                        ui.label(
                            RichText::new(m["name"].as_str().unwrap_or(lang.text("Сохранение")))
                                .strong(),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{} · {}",
                                m["lifePath"].as_str().unwrap_or("V"),
                                m["platform"].as_str().unwrap_or("PC")
                            ))
                            .color(MUTED),
                        );
                        ui.label(
                            RichText::new(format!(
                                "{:.1} {}",
                                m["playTime"].as_f64().unwrap_or(0.) / 3600.,
                                lang.text("ч. в Найт-Сити")
                            ))
                            .color(MUTED),
                        );
                    }
                    ui.add_space(12.);
                    ui.label(
                        RichText::new(if self.dirty {
                            lang.text("Есть изменения в памяти")
                        } else {
                            lang.text("Все изменения сохранены")
                        })
                        .color(if self.dirty { YELLOW } else { CYAN }),
                    );
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(16.);
                    ui.label(
                        RichText::new(format!("v{} · Windows / macOS", env!("CARGO_PKG_VERSION")))
                            .size(11.)
                            .color(MUTED),
                    );
                    ui.label(
                        RichText::new(lang.text("Экспорт с повторной проверкой"))
                            .size(11.)
                            .color(MUTED),
                    );
                });
            });
    }
    fn player(&mut self, ui: &mut egui::Ui) {
        let lang = self.language;
        ui.label(RichText::new(lang.text("Настройте V под свой стиль игры.")).color(MUTED));
        ui.add_space(20.);
        let mut change = None;
        ui.columns(3, |columns| {
            for (column, group) in ["Attributes", "Progression", "Points"]
                .into_iter()
                .enumerate()
            {
                egui::Frame::none()
                    .fill(Color32::from_rgb(19, 28, 41))
                    .rounding(8.)
                    .inner_margin(14.)
                    .show(&mut columns[column], |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.label(
                            RichText::new(lang.text(match group {
                                "Attributes" => "АТРИБУТЫ",
                                "Progression" => "УРОВЕНЬ И НАВЫКИ",
                                _ => "ОЧКИ РАЗВИТИЯ",
                            }))
                            .size(11.)
                            .color(CYAN),
                        );
                        ui.add_space(18.);
                        for row in self
                            .stats
                            .iter()
                            .filter(|s| s.group == group && s.key != "Secondary")
                        {
                            ui.label(stat_label(&row.key, lang));
                            ui.horizontal(|ui| {
                                let mut v = row.value;
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut v)
                                            .range(
                                                if group == "Points" {
                                                    0
                                                } else if group == "Attributes" {
                                                    3
                                                } else {
                                                    1
                                                }
                                                    ..=row.max,
                                            )
                                            .speed(0.15),
                                    )
                                    .changed()
                                {
                                    change = Some((group.to_owned(), row.key.clone(), v));
                                }
                                ui.label(RichText::new(format!("/ {}", row.max)).color(MUTED));
                            });
                            ui.add_space(16.);
                        }
                    });
            }
        });
        if let Some((g, k, v)) = change {
            self.edit(
                lang.text("Характеристика обновлена"),
                |s| s.set_player_stat(&g, &k, v),
            );
        }
        ui.add_space(20.);
        ui.label(
            RichText::new(
                lang.text("Характеристики синхронизируются с модификаторами StatsSystem 2.5."),
            )
            .small()
            .color(MUTED),
        );
    }
    fn inventory(&mut self, ui: &mut egui::Ui) {
        let lang = self.language;
        let mut invs: Vec<_> = self.items.iter().map(|i| i.inventory).collect();
        invs.sort_unstable();
        invs.dedup();
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("inventory")
                .selected_text(inv_label(self.inventory, lang))
                .show_ui(ui, |ui| {
                    for id in invs {
                        ui.selectable_value(&mut self.inventory, id, inv_label(id, lang));
                    }
                });
            ui.add_space(16.);
            ui.label(RichText::new(lang.text("Поиск")).color(MUTED));
            ui.text_edit_singleline(&mut self.search);
        });
        ui.add_space(12.);
        let money = self
            .items
            .iter()
            .find(|i| i.inventory == self.inventory && i.id == tdb("Items.money"))
            .cloned();
        if let (Some(item), Some(s)) = (money, &self.save) {
            if let Some(p) = item.quantity {
                let mut value = u32_at(&s.data, p).unwrap_or(0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("EURODOLLARS").color(YELLOW));
                    if ui
                        .add(egui::DragValue::new(&mut value).range(0..=999999999))
                        .changed()
                    {
                        self.edit(lang.text("Баланс изменён"), |s| {
                            put32(&mut s.data, p, value)
                        });
                    }
                });
            }
        }
        ui.add_space(10.);
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("stack")
                .width(230.)
                .selected_text(STACK_RECORDS[self.stack])
                .show_ui(ui, |ui| {
                    for (i, r) in STACK_RECORDS.iter().enumerate() {
                        ui.selectable_value(&mut self.stack, i, *r);
                    }
                });
            ui.add(egui::DragValue::new(&mut self.stack_qty).range(1..=999999999));
            if ui.button(lang.text("Добавить ресурс")).clicked() {
                let (inv, record, qty) =
                    (self.inventory, STACK_RECORDS[self.stack], self.stack_qty);
                self.edit(lang.text("Ресурс добавлен"), |s| {
                    s.add_stack(inv, record, qty)
                });
            }
        });
        ui.label(RichText::new(lang.text("Добавление: деньги, компоненты и боеприпасы. Неизвестные предметы показаны по TDBID.")).small().color(MUTED));
        ui.add_space(15.);
        let filter = self.search.to_lowercase();
        let rows: Vec<_> = self
            .items
            .iter()
            .filter(|i| i.inventory == self.inventory && i.name.to_lowercase().contains(&filter))
            .cloned()
            .collect();
        ui.label(format!("{} {}", rows.len(), lang.text("предметов")));
        let mut quantity = None;
        let mut flag = None;
        let name_width = (ui.available_width() - 280.).clamp(180., 560.);
        egui::ScrollArea::vertical()
            .id_salt("items")
            .show(ui, |ui| {
                egui::Grid::new("items_grid")
                    .num_columns(3)
                    .striped(true)
                    .min_col_width(110.)
                    .spacing([20., 8.])
                    .show(ui, |ui| {
                        ui.strong(lang.text("Предмет / TDBID"));
                        ui.strong(lang.text("Количество"));
                        ui.strong(lang.text("Квестовый"));
                        ui.end_row();
                        for item in rows {
                            ui.add_sized(
                                [name_width, 22.],
                                egui::Label::new(&item.name).truncate(),
                            )
                            .on_hover_text(format!(
                                "ID {:010X} · node @ {:X}",
                                item.id, item.offset
                            ));
                            if let Some(p) = item.quantity {
                                let mut v =
                                    u32_at(&self.save.as_ref().unwrap().data, p).unwrap_or(1);
                                if ui
                                    .add(egui::DragValue::new(&mut v).range(1..=999999999))
                                    .changed()
                                {
                                    quantity = Some((p, v));
                                }
                            } else {
                                ui.label(RichText::new(lang.text("Уникальный")).color(MUTED));
                            }
                            let mut quest = self.save.as_ref().unwrap().data[item.flags] & 1 != 0;
                            if ui.checkbox(&mut quest, "").changed() {
                                flag = Some((item.flags, quest));
                            }
                            ui.end_row();
                        }
                    });
            });
        if let Some((p, v)) = quantity {
            self.edit(lang.text("Количество изменено"), |s| {
                put32(&mut s.data, p, v)
            });
        }
        if let Some((p, v)) = flag {
            self.edit(lang.text("Флаг предмета изменён"), |s| {
                s.data[p] = (s.data[p] & !1) | u8::from(v);
                Ok(())
            });
        }
    }
    fn facts(&mut self, ui: &mut egui::Ui) {
        let lang = self.language;
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.search);
            if ui.button(lang.text("Секретная концовка")).clicked() {
                self.edit(
                    lang.text("Установлен sq032_johnny_friend = 1"),
                    |s| s.set_fact(fact_hash("sq032_johnny_friend"), 1),
                );
            }
            if ui.button(lang.text("Романтические линии")).clicked() {
                self.edit(
                    lang.text("Установлены флаги romanceable"),
                    |s| {
                        for name in [
                            "judy_romanceable",
                            "river_romanceable",
                            "panam_romanceable",
                            "kerry_romanceable",
                        ] {
                            s.set_fact(fact_hash(name), 1)?;
                        }
                        Ok(())
                    },
                );
            }
        });
        ui.label(
            RichText::new(lang.text(
                "Пресеты меняют флаги доступности; дальнейший сюжет зависит от состояния квестов.",
            ))
            .small()
            .color(MUTED),
        );
        ui.add_space(10.);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.new_fact)
                    .hint_text(lang.text("Имя нового факта"))
                    .desired_width(290.),
            );
            ui.add(egui::DragValue::new(&mut self.new_value));
            if ui
                .add_enabled(
                    !self.new_fact.trim().is_empty(),
                    egui::Button::new(lang.text("Установить факт")),
                )
                .clicked()
            {
                let name = self.new_fact.trim().to_owned();
                let v = self.new_value;
                self.edit(lang.text("Факт установлен"), |s| {
                    s.set_fact(fact_hash(&name), v)
                });
            }
        });
        ui.add_space(15.);
        let filter = self.search.to_lowercase();
        let rows: Vec<_> = self
            .facts
            .iter()
            .filter(|f| {
                f.name.to_lowercase().contains(&filter)
                    || format!("{:08x}", f.hash).contains(&filter)
            })
            .cloned()
            .collect();
        ui.label(format!(
            "{} / {} {}",
            rows.len(),
            self.facts.len(),
            lang.text("флагов")
        ));
        let mut patch = None;
        egui::ScrollArea::vertical().id_salt("facts").show_rows(
            ui,
            24.,
            rows.len(),
            |ui, range| {
                for i in range {
                    let f = &rows[i];
                    ui.push_id(f.offset, |ui| {
                        ui.horizontal(|ui| {
                            let mut value = f.value;
                            if ui
                                .add(egui::DragValue::new(&mut value).range(0..=u32::MAX))
                                .changed()
                            {
                                patch = Some((f.offset, value));
                            }
                            ui.label(&f.name)
                                .on_hover_text(format!("FNV1a {:08X}", f.hash));
                        });
                    });
                }
            },
        );
        if let Some((p, v)) = patch {
            self.edit(
                lang.text("Значение флага изменено"),
                |s| put32(&mut s.data, p, v),
            );
        }
    }
    fn vehicles(&mut self, ui: &mut egui::Ui) {
        let lang = self.language;
        ui.label(
            RichText::new(lang.text("Автомобили и мотоциклы в списке вызова V.")).color(MUTED),
        );
        ui.add_space(12.);
        ui.text_edit_singleline(&mut self.search);
        ui.add_space(12.);
        let mut catalog = self.catalog.clone();
        for v in &self.vehicles {
            if !catalog.iter().any(|s| tdb(s) == v.id) {
                catalog.push(format!("#{:010X}", v.id));
            }
        }
        let filter = self.search.to_lowercase();
        let rows: Vec<_> = catalog
            .into_iter()
            .filter(|s| s.to_lowercase().contains(&filter))
            .collect();
        let mut change = None;
        egui::ScrollArea::vertical().id_salt("garage").show_rows(
            ui,
            28.,
            rows.len(),
            |ui, range| {
                for i in range {
                    let name = &rows[i];
                    let id = if let Some(hash) = name.strip_prefix('#') {
                        u64::from_str_radix(hash, 16).unwrap_or(0)
                    } else {
                        tdb(name)
                    };
                    let mut unlocked = self.vehicles.iter().any(|v| v.id == id);
                    if ui.checkbox(&mut unlocked, name).changed() {
                        change = Some((id, unlocked));
                    }
                }
            },
        );
        if let Some((id, v)) = change {
            self.edit(
                lang.text("Список транспорта обновлён"),
                |s| s.set_vehicle(id, v),
            );
        }
    }
    fn appearance(&mut self, ui: &mut egui::Ui) {
        let lang = self.language;
        if let Some(save) = &self.save {
            match save.appearance_types() {
                Ok(types) => {
                    let mut body = types.body;
                    let mut voice = types.voice;
                    ui.horizontal_wrapped(|ui| {
                        ui.label(lang.text("Тип тела"));
                        ui.selectable_value(&mut body, Gender::Male, lang.text("Мужское"));
                        ui.selectable_value(&mut body, Gender::Female, lang.text("Женское"));
                        ui.add_space(20.);
                        ui.label(lang.text("Тип голоса"));
                        ui.selectable_value(&mut voice, Gender::Male, lang.text("Мужской"));
                        ui.selectable_value(&mut voice, Gender::Female, lang.text("Женский"));
                    });
                    ui.label(RichText::new(lang.text("Смена тела сбрасывает настройки внешности к базовому пресету выбранного типа. Голос выбирается независимо. Изменения можно отменить.")).small().color(MUTED));
                    if body != types.body {
                        self.request_body_change(body);
                    } else if voice != types.voice {
                        self.edit(lang.text("Тип голоса изменён"), |s| {
                            s.set_voice_type(voice)
                        });
                    }
                }
                Err(e) => {
                    ui.colored_label(YELLOW, format!("{}: {e:#}", lang.text("Тип тела")));
                }
            }
        }
        ui.horizontal_wrapped(|ui| {
            if ui.button(lang.text("Сохранить пресет персонажа")).clicked()
            {
                self.save_preset();
            }
            if ui.button(lang.text("Загрузить пресет персонажа")).clicked()
            {
                self.load_preset();
            }
            if ui.button(lang.text("Исправить киберруки")).clicked() {
                self.edit(
                    lang.text("Внешность киберрук исправлена"),
                    |s| s.repair_cyberarm_appearance(),
                );
                self.appearance_index = None;
                self.appearance_value.clear();
            }
        });
        ui.label(RichText::new(lang.text("Сохранение и загрузка пресетов выполняются кнопками. Загрузка пресета восстанавливает тело, голос и всю внешность.")).small().color(MUTED));
        ui.add_space(12.);
        ui.label(
            RichText::new(lang.text("Расширенное редактирование записей внешности V."))
                .color(MUTED),
        );
        ui.label(RichText::new(lang.text("Используйте действующие имена ресурсов игры. Визуальный предпросмотр модели не включён.")).small().color(YELLOW));
        ui.add_space(12.);
        ui.text_edit_singleline(&mut self.search);
        ui.add_space(12.);
        if let Some(i) = self.appearance_index {
            ui.horizontal(|ui| {
                ui.label(lang.text("Определение / morph target:"));
                ui.text_edit_singleline(&mut self.appearance_value);
                if ui.button(lang.text("Применить")).clicked() {
                    let v = self.appearance_value.clone();
                    self.edit(lang.text("Внешность обновлена"), |s| {
                        s.set_appearance(i, &v)
                    });
                }
            });
            ui.add_space(12.);
        }
        let filter = self.search.to_lowercase();
        egui::ScrollArea::vertical()
            .id_salt("appearance")
            .show(ui, |ui| {
                for (i, a) in self.appearances.iter().enumerate().filter(|(_, a)| {
                    format!("{} {} {}", a.group, a.label, a.value)
                        .to_lowercase()
                        .contains(&filter)
                }) {
                    ui.push_id(i, |ui| {
                        egui::Frame::none()
                            .fill(Color32::from_rgb(20, 28, 40))
                            .inner_margin(10.)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&a.group).small().color(CYAN));
                                        ui.label(format!("{}   /   {}", a.label, a.value));
                                    });
                                    if ui.small_button(lang.text("Изменить")).clicked() {
                                        self.appearance_index = Some(i);
                                        self.appearance_value = a.value.clone();
                                    }
                                });
                            });
                        ui.add_space(4.);
                    });
                }
            });
    }
    fn request_body_change(&mut self, body: Gender) {
        let Some(mut candidate) = self.save.clone() else {
            return;
        };
        match candidate.set_body_type(body) {
            Ok(()) => {
                self.edit(
                    self.language
                        .text("Тип тела изменён; внешность сброшена к базовому пресету"),
                    |s| {
                        *s = candidate;
                        Ok(())
                    },
                );
                self.appearance_index = None;
                self.appearance_value.clear();
            }
            Err(error) if error.is::<IncompatibleCyberwareCosmetics>() => {
                self.pending_body_change = Some(body);
            }
            Err(error) => self.report(Err(error), ""),
        }
    }

    fn confirm_body_change(&mut self, confirmed: bool) {
        if let Some(body) = self.pending_body_change.take().filter(|_| confirmed) {
            self.edit(
                self.language
                    .text("Тип тела изменён; косметика имплантов сброшена"),
                |s| s.set_body_type_without_cosmetics(body),
            );
            self.appearance_index = None;
            self.appearance_value.clear();
        }
    }

    fn tools(&mut self, ui: &mut egui::Ui) {
        let lang = self.language;
        if let Some(s) = &self.save {
            ui.label(format!(
                "{} {} · {} {}",
                lang.text("Версия сохранения:"),
                s.version,
                lang.text("Версия игры:"),
                s.game
            ));
            ui.label(format!(
                "{} {} · {:.2} {} · {:.2} {}",
                s.nodes.len(),
                lang.text("узлов"),
                s.original.len() as f64 / 1048576.,
                lang.text("MB на диске"),
                s.data.len() as f64 / 1048576.,
                lang.text("MB распаковано")
            ));
            ui.add_space(16.);
            ui.label(RichText::new(lang.text("Совместимость проверена на предоставленном сохранении macOS 2.5. Проверка загрузки отредактированной копии в игре выполняется отдельно.")).color(YELLOW));
            ui.add_space(16.);
            if ui
                .button(lang.text("Проверить запись и повторное чтение"))
                .clicked()
            {
                let r = s.encode().map(|_| ());
                self.report(r, lang.text("Структура записанной копии прошла проверку"));
            }
            ui.add_space(16.);
        }
        ui.horizontal(|ui| {
            if ui.button(lang.text("Экспорт флагов JSON")).clicked() {
                if let Some(p) = rfd::FileDialog::new()
                    .set_title(lang.text("Экспорт флагов JSON"))
                    .set_file_name("quest-facts.json")
                    .save_file()
                {
                    let rows: Vec<_> = self
                        .facts
                        .iter()
                        .map(|f| serde_json::json!({"name":f.name,"hash":f.hash,"value":f.value}))
                        .collect();
                    self.report(
                        serde_json::to_vec_pretty(&rows)
                            .map_err(Into::into)
                            .and_then(|b| std::fs::write(p, b).map_err(Into::into)),
                        lang.text("Флаги экспортированы"),
                    );
                }
            }
        });
    }
    fn about(&self, ui: &mut egui::Ui) {
        let lang = self.language;
        egui::ScrollArea::vertical().id_salt("about").show(ui, |ui| {
            ui.set_max_width(ui.available_width().min(760.));
            ui.label(RichText::new("Sergei Peshalov / desfpc").size(23.).strong().color(YELLOW));
            ui.add_space(10.);
            ui.label(lang.text("Я разработчик из Лимасола, Кипр. Создаю веб-сервисы и инструменты на PHP и Go, развиваю проекты с открытым исходным кодом. В свободное время занимаюсь музыкой."));
            ui.add_space(24.);
            ui.label(RichText::new(lang.text("Мои проекты")).strong().color(CYAN));
            ui.add_space(8.);
            ui.hyperlink_to("nozhove.com", "https://nozhove.com");
            ui.label(RichText::new(lang.text("Портал с сервисами для повседневных задач.")).small().color(MUTED));
            ui.add_space(10.);
            ui.hyperlink_to("GitHub · desfpc", "https://github.com/desfpc");
            ui.label(RichText::new(lang.text("Мои проекты с открытым исходным кодом.")).small().color(MUTED));
            ui.add_space(24.);
            ui.label(RichText::new(lang.text("Моя музыка")).strong().color(CYAN));
            ui.add_space(8.);
            for (name, url) in [
                ("Čerdyn", "https://music.youtube.com/channel/UCkWguomieoA4iJkBpQTFhCQ"),
                ("Kasapski Nozhove", "https://music.youtube.com/channel/UC8ZQp1eYK3oO--Ir35gAChw"),
                ("Gut Splinters", "https://music.youtube.com/channel/UCySV3dn2r4YlcQEVZ0kmRJA"),
                ("W1NTERH8", "https://music.youtube.com/channel/UCiNgoh6kHN0EPxhVd1MrRbw"),
            ] {
                ui.hyperlink_to(name, url);
                ui.add_space(5.);
            }
            ui.add_space(24.);
            ui.separator();
            ui.add_space(8.);
            ui.label(RichText::new(format!("Night City Save Editor · v{}", env!("CARGO_PKG_VERSION"))).small().color(MUTED));
            egui::CollapsingHeader::new(lang.text("Благодарности и лицензия")).show(ui, |ui| {
                ui.label(lang.text("Нативный интерфейс egui. Rust, LZ4, формат REDengine."));
                ui.hyperlink_to(lang.text("CyberCAT-SimpleGUI · исходный ориентир"), "https://github.com/Deweh/CyberCAT-SimpleGUI");
                ui.hyperlink_to(lang.text("WolvenKit · исследование формата"), "https://github.com/WolvenKit/WolvenKit");
                ui.label(lang.text("GPL-3.0-only. Словари CyberCAT используются по MIT."));
            });
        });
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let lang = self.language;
        if ctx.input(|i| i.viewport().close_requested()) && self.dirty && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_close = true;
        }
        self.top(ctx);
        self.sidebar(ctx);
        egui::TopBottomPanel::bottom("status")
            .min_height(40.)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        if self.error {
                            Color32::LIGHT_RED
                        } else {
                            MUTED
                        },
                        if self.error {
                            format!("{}: {}", lang.text("Ошибка"), self.status)
                        } else {
                            lang.message(&self.status)
                        },
                    );
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.pending_body_change.is_some() {
                ui.disable();
            }
            ui.add_space(16.);
            ui.horizontal(|ui| {
                ui.heading(RichText::new(self.tab.title(lang)).size(28.));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_enabled(
                            !self.redo.is_empty(),
                            egui::Button::new(lang.text("Повторить")),
                        )
                        .clicked()
                    {
                        if let Some(s) = self.redo.pop() {
                            self.undo.push(self.save.take().unwrap());
                            self.save = Some(s);
                            self.refresh();
                        }
                    }
                    if ui
                        .add_enabled(
                            !self.undo.is_empty(),
                            egui::Button::new(lang.text("Отменить")),
                        )
                        .clicked()
                    {
                        if let Some(s) = self.undo.pop() {
                            self.redo.push(self.save.take().unwrap());
                            self.save = Some(s);
                            self.refresh();
                        }
                    }
                });
            });
            ui.add_space(16.);
            if self.tab == Tab::About {
                self.about(ui);
            } else if self.save.is_none() {
                ui.add_space(70.);
                ui.label(
                    RichText::new(lang.text("ВАШ НОВЫЙ БИЛД НАЧИНАЕТСЯ ЗДЕСЬ"))
                        .color(CYAN)
                        .size(13.),
                );
                ui.add_space(15.);
                ui.heading(lang.text("Найт-Сити. Ваши правила."));
                ui.add_space(12.);
                ui.label(lang.text("Откройте сохранение Cyberpunk 2077 версии 2.5."));
                ui.label(
                    lang.text("Характеристики, ресурсы, транспорт, внешность и квестовые флаги."),
                );
                ui.add_space(22.);
                let example = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("saveExample/sav.dat");
                if example.exists() && ui.button(lang.text("Открыть saveExample")).clicked()
                {
                    self.open(example);
                }
            } else {
                if !self.cache_errors.is_empty() {
                    egui::CollapsingHeader::new(
                        RichText::new(lang.text("Некоторые разделы недоступны")).color(YELLOW),
                    )
                    .show(ui, |ui| {
                        for e in &self.cache_errors {
                            ui.label(e);
                        }
                    });
                }
                match self.tab {
                    Tab::Player => {
                        egui::ScrollArea::vertical()
                            .show(ui, |ui| self.player(ui))
                            .inner
                    }
                    Tab::Inventory => self.inventory(ui),
                    Tab::Appearance => self.appearance(ui),
                    Tab::Vehicles => self.vehicles(ui),
                    Tab::Facts => self.facts(ui),
                    Tab::Tools => self.tools(ui),
                    Tab::About => self.about(ui),
                }
            }
        });
        if self.pending_body_change.is_none() && (self.pending_open.is_some() || self.confirm_close)
        {
            egui::Window::new(lang.text("Изменения не экспортированы"))
                .id(egui::Id::new("unsaved_changes"))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
                .show(ctx, |ui| {
                    ui.label(lang.text("Сохраните изменения перед продолжением."));
                    ui.horizontal(|ui| {
                        if ui.button(lang.text("Остаться")).clicked() {
                            self.pending_open = None;
                            self.confirm_close = false;
                        }
                        if ui
                            .button(lang.text(if self.replace_save {
                                "Заменить сохранение"
                            } else {
                                "Экспорт копии"
                            }))
                            .clicked()
                        {
                            self.export();
                        }
                        if ui.button(lang.text("Отбросить изменения")).clicked() {
                            if let Some(p) = self.pending_open.take() {
                                self.open(p);
                            }
                            if self.confirm_close {
                                self.allow_close = true;
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    });
                });
        }
        if self.pending_body_change.is_some() {
            let mut open = true;
            let mut decision = None;
            egui::Window::new(lang.text("Сбросить косметику имплантов?"))
                .id(egui::Id::new("body_cosmetic_reset"))
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
                .show(ctx, |ui| {
                    ui.set_max_width(480.);
                    ui.label(lang.text("Для выбранной косметики имплантов нет проверенного варианта другого тела. Можно продолжить со стандартной косметикой. Текущие цвета, бренд и другие косметические настройки имплантов будут сброшены. Установленные импланты и их характеристики сохранятся. Изменение можно отменить."));
                    ui.add_space(12.);
                    ui.horizontal_wrapped(|ui| {
                        if ui.button(lang.text("Отмена")).clicked() {
                            decision = Some(false);
                        }
                        if ui.button(lang.text("Сменить тело и сбросить косметику")).clicked() {
                            decision = Some(true);
                        }
                    });
                });
            if !open || ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                decision = Some(false);
            }
            if let Some(confirmed) = decision {
                self.confirm_body_change(confirmed);
            }
        }
    }
}
fn stat_label(s: &str, lang: Language) -> &str {
    lang.text(match s {
        "Strength" => "Сила",
        "Reflexes" => "Реакция",
        "TechnicalAbility" => "Техника",
        "Intelligence" => "Интеллект",
        "Cool" => "Хладнокровие",
        "Level" => "Уровень",
        "StreetCred" => "Репутация",
        "CoolSkill" => "Киллер",
        "IntelligenceSkill" => "Нетраннер",
        "ReflexesSkill" => "Синоби",
        "StrengthSkill" => "Соло",
        "TechnicalAbilitySkill" => "Инженер",
        "Attribute" => "Очки атрибутов",
        "Primary" => "Очки способностей",
        "Secondary" => "Очки развития (Secondary)",
        "Espionage" => "Очки биочипа",
        _ => s,
    })
}
fn inv_label(id: u64, lang: Language) -> String {
    match id {
        1 => lang.text("Инвентарь V").into(),
        1000 => lang.text("Тайник").into(),
        _ => format!("{} {id}", lang.text("Контейнер")),
    }
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|a| a == "--inspect") {
        let p = args
            .get(2)
            .context("Usage: nightcity-save-editor --inspect <sav.dat>")?;
        let s = Save::load(std::path::Path::new(p))?;
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::json!({"gameVersion":s.game,"saveVersion":s.version,"nodes":s.nodes.len(),"facts":s.facts()?.len(),"items":s.items()?.len(),"vehicles":s.vehicles()?.len(),"appearanceEntries":s.appearances()?.len(),"stats":s.player_stats()?.iter().map(|v|serde_json::json!({"group":v.group,"name":v.key,"value":v.value})).collect::<Vec<_>>()})
            )?
        );
        return Ok(());
    }
    let initial = args.get(1).map(PathBuf::from);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Night City · Cyberpunk 2077 Save Editor")
            .with_inner_size([1240., 840.])
            .with_min_inner_size([960., 680.]),
        ..Default::default()
    };
    eframe::run_native(
        "Night City",
        options,
        Box::new(move |cc| {
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = BG;
            visuals.window_fill = Color32::from_rgb(18, 25, 37);
            visuals.extreme_bg_color = Color32::from_rgb(8, 13, 20);
            visuals.selection.bg_fill = Color32::from_rgb(22, 75, 79);
            visuals.selection.stroke = egui::Stroke::new(1_f32, CYAN);
            visuals.widgets.inactive.bg_fill = Color32::from_rgb(27, 37, 52);
            visuals.widgets.hovered.bg_fill = Color32::from_rgb(37, 58, 71);
            cc.egui_ctx.set_visuals(visuals);
            let mut style = (*cc.egui_ctx.style()).clone();
            style.spacing.item_spacing = egui::vec2(10., 8.);
            style.spacing.button_padding = egui::vec2(12., 8.);
            cc.egui_ctx.set_style(style);
            let mut app = App::default();
            if let Some(p) = initial {
                app.open(p);
            }
            Ok(Box::new(app))
        }),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))
}

#[cfg(test)]
mod body_change_tests {
    use super::*;
    fn incompatible_save() -> Save {
        let mut s = Save::load(std::path::Path::new("saveExample")).unwrap();
        let (index, row) = s
            .appearances()
            .unwrap()
            .into_iter()
            .enumerate()
            .find(|(_, a)| a.group.starts_with("Arms / elemental_"))
            .unwrap();
        s.data[row.start - 8..row.start].copy_from_slice(&123u64.to_le_bytes());
        s.set_appearance(index, "female_unmapped_cosmetic").unwrap();
        s
    }
    #[test]
    fn incompatible_body_change_requires_consent_and_can_be_cancelled_or_undone() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let original = incompatible_save();
        let mut app = App {
            save: Some(original.clone()),
            ..App::default()
        };
        app.request_body_change(Gender::Male);
        assert_eq!(app.pending_body_change, Some(Gender::Male));
        assert!(app.save.as_ref().unwrap().data == original.data);
        assert!(app.undo.is_empty());
        app.confirm_body_change(false);
        assert!(app.pending_body_change.is_none());
        assert!(app.save.as_ref().unwrap().data == original.data);
        assert!(app.undo.is_empty());
        app.request_body_change(Gender::Male);
        app.confirm_body_change(true);
        assert!(app.pending_body_change.is_none());
        assert!(!app.error);
        assert!(app.dirty);
        assert_eq!(
            app.save.as_ref().unwrap().appearance_types().unwrap().body,
            Gender::Male
        );
        assert_eq!(app.undo.len(), 1);
        assert!(app.undo.pop().unwrap().data == original.data);
    }
    #[test]
    fn structural_errors_do_not_offer_a_cosmetic_reset() {
        if !std::path::Path::new("saveExample/sav.dat").exists() {
            return;
        }
        let mut original = incompatible_save();
        let n = original.node("PersistencySystem2").unwrap();
        let at = n.offset + 4;
        original.data[at..at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        let mut app = App {
            save: Some(original.clone()),
            ..App::default()
        };
        app.request_body_change(Gender::Male);
        assert!(app.pending_body_change.is_none());
        assert!(app.error);
        assert!(app.save.as_ref().unwrap().data == original.data);
        assert!(app.undo.is_empty());
    }
}
