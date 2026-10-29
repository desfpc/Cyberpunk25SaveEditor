//! UI translations only. Save identifiers and user-entered values are never translated.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Language {
    Russian,
    #[default]
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
        for prefix in [
            "Загружено:",
            "Loaded:",
            "Копия сохранена:",
            "Copy saved:",
            "Сохранение заменено. Резервная копия:",
            "Save replaced. Backup:",
        ] {
            if let Some(detail) = message.strip_prefix(prefix) {
                return format!("{}{}", self.text(prefix), detail);
            }
        }
        self.text(message).to_owned()
    }
}
const MESSAGES:&[(&str,&str)]=&[
    ("Все предметы", "All items"),
    ("Огнестрельное оружие", "Ranged weapons"),
    ("Оружие ближнего боя", "Melee weapons"),
    ("Модификации и обвесы", "Weapon mods and attachments"),
    ("Боеприпасы", "Ammunition"),
    ("Импланты", "Cyberware"),
    ("Одежда", "Clothing"),
    ("Расходники", "Consumables"),
    ("Компоненты и чертежи", "Components and crafting specs"),
    ("Скрипты", "Quickhacks"),
    ("Мусор", "Junk"),
    ("Прочее", "Other"),
    ("Неизвестные", "Unknown"),
    ("Только культовое", "Iconic only"),
    ("Только квестовые", "Quest items only"),
    ("Культовое", "Iconic"),
    ("Название или ID", "Name or ID"),
    ("Сбросить фильтры", "Reset filters"),
    ("Нет предметов, соответствующих фильтрам", "No items match the filters"),
    ("Предмет / ID", "Item / ID"),
    ("Лимит игры:", "Game limit:"),
    ("Этот уровень превышает стандартный лимит игры. Используйте его только с модом, который повышает соответствующий лимит.", "This level exceeds the standard game limit. Use it only with a mod that raises the corresponding limit."),
    ("Сбросить косметику имплантов?", "Reset cyberware cosmetics?"),
    ("Для выбранной косметики имплантов нет проверенного варианта другого тела. Можно продолжить со стандартной косметикой. Текущие цвета, бренд и другие косметические настройки имплантов будут сброшены. Установленные импланты и их характеристики сохранятся. Изменение можно отменить.", "The selected cyberware cosmetics have no verified counterpart for the other body. You can continue with default cosmetics. Current colors, brand and other cyberware cosmetic settings will be reset. Installed cyberware and its stats will be retained. You can undo this change."),
    ("Отмена", "Cancel"),
    ("Сменить тело и сбросить косметику", "Change body and reset cosmetics"),
    ("Тип тела изменён; косметика имплантов сброшена", "Body changed; cyberware cosmetics reset"),
    ("О нас", "About us"),
    ("Я разработчик из Лимасола, Кипр. Создаю веб-сервисы и инструменты на PHP и Go, развиваю проекты с открытым исходным кодом. В свободное время занимаюсь музыкой.", "I'm a developer based in Limassol, Cyprus. I build web services and tools with PHP and Go, and work on open-source projects. In my spare time, I make music."),
    ("Мои проекты", "My projects"),
    ("Портал с сервисами для повседневных задач.", "A portal with services for everyday tasks."),
    ("Мои проекты с открытым исходным кодом.", "My open-source projects."),
    ("Моя музыка", "My music"),
    ("Благодарности и лицензия", "Credits and license"),
    ("Исправить киберруки", "Repair cyberarm appearance"),
    ("Внешность киберрук исправлена", "Cyberarm appearance repaired"),
    ("Все изменения сохранены", "All changes saved"),
    ("Сохраните изменения перед продолжением.", "Save your changes before continuing."),
    ("Сохранить пресет персонажа", "Save character preset"),
    ("Загрузить пресет персонажа", "Load character preset"),
    ("Пресет персонажа сохранён", "Character preset saved"),
    ("Пресет персонажа загружен", "Character preset loaded"),
    ("Сохранение и загрузка пресетов выполняются кнопками. Загрузка пресета восстанавливает тело, голос и всю внешность.", "Use the buttons to save and load presets. Loading a preset restores body, voice and all appearance settings."),
    ("Заменить сохранение", "Replace save"),
    ("Заменять сейв (копия в old)", "Replace save (backup in old)"),
    ("Сохранение заменено. Резервная копия:", "Save replaced. Backup:"),
    ("Персонаж", "Player"),
    ("Инвентарь", "Inventory"),
    ("Внешность", "Appearance"),
    ("Тип тела", "Body type"),
    ("Тип голоса", "Voice type"),
    ("Мужское", "Male"),
    ("Женское", "Female"),
    ("Мужской", "Masculine"),
    ("Женский", "Feminine"),
    ("Тип тела изменён; внешность сброшена к базовому пресету", "Body type updated; appearance reset to the default preset"),
    ("Тип голоса изменён", "Voice type updated"),
    ("Смена тела сбрасывает настройки внешности к базовому пресету выбранного типа. Голос выбирается независимо. Изменения можно отменить.", "Changing body type resets appearance to the selected body's default preset. Voice is selected independently. Changes can be undone."),
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
    fn english_is_the_startup_default() {
        assert!(Language::default() == Language::English);
    }
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
