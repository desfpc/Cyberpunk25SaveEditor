//! UI translations only. Save identifiers and user-entered values are never translated.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Language {
    #[default]
    Russian,
    English,
}
impl Language {
    pub fn label(self) -> &'static str {
        match self {
            Self::Russian => "Русский",
            Self::English => "English",
        }
    }
    pub fn text(self, text: &str) -> &str {
        for &(ru, en) in MESSAGES {
            if text == ru || text == en {
                return match self {
                    Self::Russian => ru,
                    Self::English => en,
                };
            }
        }
        text
    }
    /// Translate an existing status again when the user changes language.
    pub fn message(self, message: &str) -> String {
        for prefix in ["Загружено:", "Loaded:", "Копия сохранена:", "Copy saved:"]
        {
            if let Some(detail) = message.strip_prefix(prefix) {
                return format!("{}{}", self.text(prefix), detail);
            }
        }
        self.text(message).to_owned()
    }
}
const MESSAGES:&[(&str,&str)]=&[
    ("Персонаж", "Player"),
    ("Инвентарь", "Inventory"),
    ("Внешность", "Appearance"),
    ("Гараж", "Garage"),
    ("Квестовые флаги", "Quest facts"),
    ("Данные сохранения", "Save information"),
    ("Выберите sav.dat или откройте пример сохранения", "Select sav.dat or open the example save"),
    ("Папка для новой копии сохранения", "Choose a folder for the new save copy"),
    ("Экспорт копии", "Export copy"),
    ("Открыть sav.dat", "Open sav.dat"),
    (" РЕДАКТОР СОХРАНЕНИЯ", " SAVE EDITOR"),
    ("Сохранение", "Save"),
    ("Есть изменения в памяти", "Unexported changes"),
    ("Исходный файл не изменён", "Original file unchanged"),
    ("Экспорт с повторной проверкой", "Export with verification"),
    ("Настройте V под свой стиль игры.", "Build V around your play style."),
    ("АТРИБУТЫ", "ATTRIBUTES"),
    ("УРОВЕНЬ И НАВЫКИ", "LEVEL AND SKILLS"),
    ("ОЧКИ РАЗВИТИЯ", "DEVELOPMENT POINTS"),
    ("Характеристика обновлена", "Player stat updated"),
    ("Характеристики синхронизируются с модификаторами StatsSystem 2.5.", "Stats are synchronized with the StatsSystem 2.5 modifiers."),
    ("Поиск", "Search"),
    ("Баланс изменён", "Balance updated"),
    ("Добавить ресурс", "Add resource"),
    ("Ресурс добавлен", "Resource added"),
    ("Добавление: деньги, компоненты и боеприпасы. Неизвестные предметы показаны по TDBID.", "Add money, crafting components and ammunition. Unknown items are shown by TDBID."),
    ("Предмет / TDBID", "Item / TDBID"),
    ("Количество", "Quantity"),
    ("Квестовый", "Quest item"),
    ("Уникальный", "Unique"),
    ("Количество изменено", "Quantity updated"),
    ("Флаг предмета изменён", "Item flag updated"),
    ("Секретная концовка", "Secret ending"),
    ("Установлен sq032_johnny_friend = 1", "Set sq032_johnny_friend = 1"),
    ("Романтические линии", "Romance options"),
    ("Установлены флаги romanceable", "Romanceable flags set"),
    ("Пресеты меняют флаги доступности; дальнейший сюжет зависит от состояния квестов.", "Presets change availability flags; story outcomes also depend on quest progress."),
    ("Имя нового факта", "New fact name"),
    ("Установить факт", "Set fact"),
    ("Факт установлен", "Fact set"),
    ("Значение флага изменено", "Fact value updated"),
    ("Автомобили и мотоциклы в списке вызова V.", "Cars and motorcycles available in V’s vehicle call list."),
    ("Список транспорта обновлён", "Vehicle list updated"),
    ("Расширенное редактирование записей внешности V.", "Advanced editing of V’s appearance entries."),
    ("Используйте действующие имена ресурсов игры. Визуальный предпросмотр модели не включён.", "Use valid game resource names. A 3D model preview is not included."),
    ("Определение / morph target:", "Definition / morph target:"),
    ("Применить", "Apply"),
    ("Внешность обновлена", "Appearance updated"),
    ("Изменить", "Edit"),
    ("Совместимость проверена на предоставленном сохранении macOS 2.5. Проверка загрузки отредактированной копии в игре выполняется отдельно.", "Compatibility was checked against the supplied macOS 2.5 save. Loading an edited copy in the game requires a separate check."),
    ("Проверить запись и повторное чтение", "Verify write and read-back"),
    ("Структура записанной копии прошла проверку", "Written copy passed structural verification"),
    ("Экспорт флагов JSON", "Export facts as JSON"),
    ("Флаги экспортированы", "Facts exported"),
    ("О ПРОЕКТЕ", "ABOUT"),
    ("Разработал Sergei Peshalov", "Developed by Sergei Peshalov"),
    ("Нативный интерфейс egui. Rust, LZ4, формат REDengine.", "Native egui interface. Rust, LZ4, REDengine format."),
    ("CyberCAT-SimpleGUI · исходный ориентир", "CyberCAT-SimpleGUI · reference project"),
    ("WolvenKit · исследование формата", "WolvenKit · format research"),
    ("GPL-3.0-only. Словари CyberCAT используются по MIT.", "GPL-3.0-only. CyberCAT dictionaries are used under the MIT license."),
    ("Повторить", "Redo"),
    ("Отменить", "Undo"),
    ("ВАШ НОВЫЙ БИЛД НАЧИНАЕТСЯ ЗДЕСЬ", "YOUR NEXT BUILD STARTS HERE"),
    ("Найт-Сити. Ваши правила.", "Night City. Your rules."),
    ("Откройте сохранение Cyberpunk 2077 версии 2.5.", "Open a Cyberpunk 2077 version 2.5 save."),
    ("Характеристики, ресурсы, транспорт, внешность и квестовые флаги.", "Player stats, resources, vehicles, appearance and quest facts."),
    ("Открыть saveExample", "Open saveExample"),
    ("Некоторые разделы недоступны", "Some sections are unavailable"),
    ("Изменения не экспортированы", "Changes have not been exported"),
    ("Экспортируйте копию, чтобы сохранить внесённые изменения.", "Export a copy to save your changes."),
    ("Остаться", "Stay"),
    ("Отбросить изменения", "Discard changes"),
    ("Сила", "Body"),
    ("Реакция", "Reflexes"),
    ("Техника", "Technical Ability"),
    ("Интеллект", "Intelligence"),
    ("Хладнокровие", "Cool"),
    ("Уровень", "Level"),
    ("Репутация", "Street Cred"),
    ("Киллер", "Headhunter"),
    ("Нетраннер", "Netrunner"),
    ("Синоби", "Shinobi"),
    ("Соло", "Solo"),
    ("Инженер", "Engineer"),
    ("Очки атрибутов", "Attribute points"),
    ("Очки способностей", "Perk points"),
    ("Очки развития (Secondary)", "Development points (Secondary)"),
    ("Очки биочипа", "Relic points"),
    ("Инвентарь V", "V’s inventory"),
    ("Тайник", "Stash"),
    ("Загружено:", "Loaded:"),
    ("Копия сохранена:", "Copy saved:"),
    ("ч. в Найт-Сити", "hours in Night City"),
    ("предметов", "items"),
    ("флагов", "facts"),
    ("узлов", "nodes"),
    ("MB на диске", "MB on disk"),
    ("MB распаковано", "MB unpacked"),
    ("Контейнер", "Container"),
    ("Версия сохранения:", "Save version:"),
    ("Версия игры:", "Game version:"),
    ("Ошибка", "Error"),
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn switching_retranslates_status_without_touching_paths() {
        let ru = "Загружено: /tmp/Сохранение/sav.dat";
        let en = Language::English.message(ru);
        assert_eq!(en, "Loaded: /tmp/Сохранение/sav.dat");
        assert_eq!(Language::Russian.message(&en), ru);
        assert_eq!(
            Language::Russian.message("Player stat updated"),
            "Характеристика обновлена"
        );
    }
    #[test]
    fn english_catalog_is_complete_and_unambiguous() {
        for (i, &(ru, en)) in MESSAGES.iter().enumerate() {
            assert!(!en.is_empty());
            assert!(
                !en.chars()
                    .any(|c| ('А'..='я').contains(&c) || c == 'ё' || c == 'Ё'),
                "Untranslated: {ru}"
            );
            assert_eq!(Language::Russian.text(en), ru);
            assert_eq!(Language::English.text(ru), en);
            assert!(
                !MESSAGES[..i].iter().any(|(r, e)| *r == ru || *e == en),
                "Duplicate translation: {ru}"
            );
        }
        assert_eq!(Language::English.text("Items.money"), "Items.money");
    }
}
