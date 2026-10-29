# Night City Save Editor

[Русский](#русский) · [English](#english)

## Русский

Нативное приложение на **Rust + egui** с тёмным интерфейсом на русском и английском языках для сохранений
**Cyberpunk 2077 2.5**. Поддерживаемая пара версий внутри `sav.dat`:
`saveVersion = 277`, `gameVersion = 2500`.

Репозиторий проекта: [desfpc/Cyberpunk25SaveEditor](https://github.com/desfpc/Cyberpunk25SaveEditor).

### Запуск

Готовая версия **0.1.9** — установка Rust и сборка не нужны:

| Платформа | Скачать | Содержимое |
| --- | --- | --- |
| Windows x64 | [NightCity-Windows-x64.zip](https://github.com/desfpc/Cyberpunk25SaveEditor/raw/refs/heads/main/downloads/NightCity-Windows-x64.zip) | `nightcity-save-editor.exe` |
| macOS Apple Silicon / Intel | [NightCity-macOS-universal.zip](https://github.com/desfpc/Cyberpunk25SaveEditor/raw/refs/heads/main/downloads/NightCity-macOS-universal.zip) | Универсальное приложение `.app` |

Скачайте ZIP для своей платформы, распакуйте и запустите приложение.
Пакеты хранятся прямо в папке [`downloads/`](downloads/), контрольные суммы —
в [`downloads/SHA256SUMS.txt`](downloads/SHA256SUMS.txt).
При локальной сборке пакеты создаются в `dist/`.

Язык интерфейса переключается в верхней панели: **Русский / English**.
Перевод применяется сразу, включая сообщения о последних действиях; загруженное
сохранение, значения полей и история отмены остаются на месте. По умолчанию
при запуске выбран английский язык. Внутренние идентификаторы игры не переводятся.

Откройте `sav.dat` кнопкой в верхней панели. При разработке также доступна
кнопка **Открыть saveExample**. Приложение работает локально, без аккаунта и сети.

Кнопка **Экспорт копии** предлагает выбрать родительскую папку и создаёт новую
папку `ManualSave-<номер>` с `sav.dat`, метаданными и скриншотом. После записи
файл повторно распаковывается и проверяется. Исходное сохранение не меняется.
Существующая папка назначения не перезаписывается. Отмена/повтор доступны для
последних 12 действий. При открытии другого файла и закрытии окна с изменениями
приложение предлагает сначала экспортировать копию.

Для использования результата закройте игру и поместите экспортированную папку
рядом с остальными игровыми сохранениями, затем загрузите её через меню игры.
Проверка структуры файла редактором не заменяет проверку загрузки в игре.

Кнопка **О нас** открывает страницу автора с проектами и музыкой; она доступна
и до загрузки сохранения.

### История изменений: 0.1.9 → 0.1.2

#### 0.1.9 — удобный инвентарь и уровни для модифицированной игры (29 октября 2026)

- Добавлены английские названия предметов с ID в скобках во всех языках интерфейса. Список инвентаря отсортирован по названию, полные сведения доступны при наведении.
- Добавлен поиск по названию предмета, строковому ID и числовому TDBID.
- Добавлены категории: огнестрельное и ближнее оружие, модификации и обвесы, боеприпасы, импланты, одежда, расходники, компоненты и чертежи, скрипты, мусор, прочее и неизвестные предметы.
- Добавлены фильтры «Только культовое» и «Только квестовые», счётчик найденных предметов и кнопка сброса фильтров. Категорию, поиск и оба фильтра можно использовать вместе.
- Для предметов без распознанного названия сохраняется отображение доступного ID/TDBID.
- Уровень персонажа, репутацию и навыки теперь можно повышать выше стандартных лимитов. При превышении лимита появляется предупреждение о необходимости соответствующего игрового мода; сам мод устанавливается отдельно.
- Двойной щелчок по числу позволяет ввести уровень с клавиатуры.

#### 0.1.8 — лицевые киберимпланты и смена тела (28 октября 2026)

- Исправлено исчезновение пункта лицевых киберимплантов у рипера. Исправление применяется автоматически при сохранении и работе с пресетами, в том числе со старыми файлами.
- Обновлён встроенный женский пресет. Рабочие варианты внешности, тон кожи, косметика киберрук и экипировка сохраняются.
- Если косметику имплантов нельзя перенести на другое тело, можно отменить смену тела или продолжить со стандартной косметикой. Установленные импланты, оружие и характеристики сохраняются; изменение можно отменить.

#### 0.1.7 — пресеты и очки развития (27 октября 2026)

- Смена тела, загрузка пресета и ремонт киберрук больше не требуют предварительного сохранения текущего пресета. Сохранение и загрузка пресетов доступны через отдельные кнопки.
- Исправлена ошибка, мешавшая сохранять пресеты с некоторыми косметическими настройками имплантов и переносить их между сейвами.
- Исправлено добавление очков способностей и атрибутов в новых сохранениях до первого повышения уровня или траты очков.

#### 0.1.6 — страница автора

- Добавлена страница «О нас / About us», доступная без загрузки сохранения: информация о разработчике и ссылки на nozhove.com и GitHub.
- Добавлены ссылки на музыкальные проекты Čerdyn, Kasapski Nozhove, Gut Splinters и W1NTERH8 в YouTube Music. Страница доступна на русском и английском.

#### 0.1.5 — исправление повторного исчезновения рук

- Исправлено повторное исчезновение мужских киберрук после обновления внешности имплантов в игре.
- Ремонт киберрук сохраняет выбранные цвета и косметику, лицо, тело, голос и экипировку.

#### 0.1.4 — внешний вид киберрук

- Исправлено отображение Gorilla Arms, Mantis Blades и Monowire; восстановлены элементальные варианты всех четырёх типов киберрук, включая Projectile Launcher.
- Добавлены подбор мужских и женских вариантов внешности и кнопка «Исправить киберруки» для существующих сохранений.
- Пресеты теперь сохраняют косметические настройки имплантов и позволяют переносить их между сейвами. Старые пресеты остаются совместимыми; инвентарь, установленные импланты, характеристики и квесты сохраняются.

#### 0.1.3 — пресеты и сохранение файлов

- Добавлены файлы `.ncpreset` для сохранения и переноса тела, голоса и полной внешности персонажа. Перед сменой тела или загрузкой другого пресета предлагается сохранить текущую внешность.
- Последняя открытая папка сохранения запоминается между запусками.
- Добавлена возможность заменить текущий сейв с резервной копией в папке `old`. Она выключена по умолчанию и сбрасывается при открытии другого сохранения.

#### 0.1.2 — исходная версия

- Добавлены независимый выбор тела и голоса, русский и английский интерфейс с английским по умолчанию, отмена и повтор изменений, экспорт отдельной копии сохранения.

### Возможности

| Раздел | Что реализовано |
| --- | --- |
| Персонаж | Пять атрибутов, уровень, репутация, пять навыков, очки развития и биочипа |
| Инвентарь | Контейнеры, деньги, количества существующих стеков, квестовые флаги, английские названия с ID, категории и совместные фильтры |
| Добавление ресурсов | Деньги, компоненты пяти уровней, четыре вида боеприпасов |
| Внешность | Мужское/женское тело и голос, расширенный редактор appearance definitions и morph targets |
| Гараж | Добавление/удаление доступного для вызова транспорта, сохранение неизвестных записей |
| Квестовые флаги | Поиск, редактирование, добавление по имени, пресеты доступности концовки/романов, JSON-экспорт |
| Диагностика | Версии, число узлов, размеры, повторная проверка записи |

Словарь содержит 40 548 технических имён записей. Отдельный справочник игры 2.5
содержит 10 156 записей с типами и 7 283 английскими названиями. Названия предметов
всегда английские, язык интерфейса переключается независимо. Если название или
строковый ID не найдены, отображается доступный ID/TDBID. Каталог транспорта взят из CyberCAT; неизвестные
уже открытые машины также показываются.

Очки развития можно добавлять до первого повышения уровня и траты очков:
отсутствующее нулевое поле `unspent` создаётся при редактировании.

Изменение характеристики синхронизирует `PlayerDevelopmentData` и соответствующий
forced modifier игрока в **новом StatsSystem 2.5** (`valuesOptimized`,
`sharedBufferSlices`, `sharedDataBuffer`). Если модификатор отсутствует,
неоднозначен или разделяется с другой сущностью, операция отклоняется целиком.
Неизвестные узлы, поля и байты остаются в сохранении. При изменении размера
узла пересчитываются смещения и размеры родительских узлов. При добавлении
предмета обновляются ID узлов, ссылки и счётчики инвентаря.

Смена типа тела сбрасывает внешность к базовому пресету выбранного типа и
синхронизирует пол в `PlayerPuppetPS`. Выбранный голос сохраняется. Переключатель
голоса меняет его независимо от тела. Пресеты сохраняются в отдельный файл
`.ncpreset` только кнопкой **Сохранить пресет персонажа**. Смена тела и ремонт
киберрук не открывают диалог сохранения пресета.
**Загрузить пресет персонажа** восстанавливает тело, голос и весь блок внешности:
ресурсы киберрук, морфы, настройки камер и теги. Загрузка не требует
предварительного сохранения текущей внешности. Инвентарь, установленные
импланты, характеристики и квесты пресет не меняет. Пресет версии 2 также сохраняет
все сериализованные поля косметической системы `CyberwareCustomizationSystem`,
включая выбранный тип и бренд. Имена полей и значений переносятся между сейвами
по именам, а не по локальным числовым ID. Доступна отмена/повтор.

Смена тела сохраняет косметические и элементальные варианты киберрук и подбирает
проверенные мужские/женские определения внешности. Встроенные пресеты обновлены
для 2.5: исправлены устаревшие имена Gorilla Arms, Mantis Blades и Monowire,
добавлены 16 элементальных групп. Мужской пресет использует проверенные по
исходному мужскому сейву отдельные FPP/TPP-группы, имена слотов и маршруты камер
для всех четырёх типов киберрук. Ремонт обновляет также старую объединённую
структуру, из-за которой руки могли повторно исчезать при обновлении внешности. Кнопка **Исправить киберруки** исправляет эти
ошибки в уже изменённом сейве без сброса лица, тела, голоса и снаряжения. Загрузка
старого пресета версии 1 тоже исправляет известные ссылки рук, сохраняя текущие
отдельные косметические настройки, которых в старом файле ещё не было.
Неизвестный гендерный вариант модифицированных ресурсов не подменяется догадкой:
при смене тела можно подтвердить сброс косметики к стандартной либо отменить операцию.

Последняя успешно открытая папка сейва запоминается между запусками. Переключатель
**Заменять сейв (копия в old)** по умолчанию выключен и сбрасывается при открытии
другого сохранения. При выключенном переключателе экспорт создаёт отдельную копию.
При включённом кнопка **Заменить сохранение** сначала проверяет отредактированную
копию и сохраняет текущие файлы в `old/backup-<метка времени>/`, затем заменяет
сейв и метаданные. Повторная запись создаёт новую резервную копию. Если исходный
`sav.dat` изменился на диске или резервную копию создать не удалось, замена
отклоняется.

### Текущие границы

Это первая рабочая версия, а не полная замена всех функций CyberCAT.

- Проверены предоставленные сохранения **macOS 2.5 / Phantom Liberty**. Пользователь
  подтвердил в игре смену тела, восстановление пресета и обновление внешности киберрук.
  Это не заменяет отдельную проверку остальных редактируемых параметров.
- macOS GUI проверен вручную. Windows EXE собран кросс-компиляцией;
  запуск на Windows пока не проверен. CI также поддерживает нативную MSVC-сборку.
- Релизы не подписаны сертификатами Apple/Microsoft и не нотарифицированы.
- Внешность редактируется по внутренним именам игры, без 3D-просмотра и
  гарантии существования введённого ресурса. Смена жизненного пути,
  импорт пресетов CyberCAT и автоматический подбор связанных ресурсов не реализованы.
- Добавление произвольного оружия, одежды, имплантов и модификаторов предметов
  не включено. Уникальные предметы можно просматривать и менять их квестовый флаг.
- Пресеты квестовых флагов не гарантируют сюжетный результат при любом состоянии квестов.
- Другие версии игры намеренно отклоняются, пока их формат не проверен.

### Разработка

Нужен актуальный стабильный Rust и системные инструменты сборки:
Xcode Command Line Tools на macOS, Visual Studio Build Tools с C++ на Windows.

```sh
cargo run --release -- saveExample/sav.dat
cargo run --release -- --inspect saveExample
cargo test --locked --lib --bins
# Локальные интеграционные тесты (при наличии исходного saveExample):
cargo test --locked --test real_save
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

```sh
# macOS, для текущей архитектуры
cargo build --release --locked
python3 scripts/package_macos.py

# Windows, в Developer PowerShell
cargo build --release --locked --target x86_64-pc-windows-msvc
```

Для универсального macOS-пакета соберите `aarch64-apple-darwin` и
`x86_64-apple-darwin`, объедините бинарники через `lipo -create` и передайте
результат в `scripts/package_macos.py --binary <путь>`.

`.github/workflows/build.yml` собирает Windows MSVC и обе архитектуры macOS,
запускает проверки и прикладывает ZIP-артефакты. Для тестов реального формата
используется исходный локальный пример в `saveExample/`, включая `sav.dat`,
`metadata.9.json` и `screenshot.png`. Некоторые проверки привязаны к данным этого
примера. Эта папка исключена из Git, личные сохранения не публикуются. В CI тесты на реальном сохранении запускаются только при наличии
примера; проверки перевода, форматирования, Clippy и сборки выполняются всегда.

Тесты проверяют побайтный round-trip без правок, чтение реальных данных,
сохранность неизвестных конечных узлов, добавление предметов/флагов,
синхронизацию характеристик, изменение гаража/внешности, обработку усечённых
файлов, отказ от неподдерживаемых правок и экспорт без изменения оригинала.

### Исходники и лицензии

Разработал **Sergei Peshalov** · [GitHub: desfpc](https://github.com/desfpc).

- `src/appearance.rs` — тип тела и голоса, базовые пресеты внешности.
- `src/binary.rs` — чтение VLQ и строк REDengine с проверкой границ.
- `src/item_catalog.rs` — названия предметов, категории и фильтры; данные в `assets/item_metadata.json`.
- `src/save.rs` — контейнер VASC, LZ4, узлы, инвентарь, флаги, внешность, гараж, экспорт.
- `src/package.rs` — RED packages, развитие персонажа, оптимизированные статы 2.5.
- `src/main.rs` — интерфейс и CLI диагностики.
- `src/i18n.rs` — русский и английский переводы интерфейса.
- `tests/real_save.rs` — проверки на `saveExample`.

- [CyberCAT-SimpleGUI](https://github.com/Deweh/CyberCAT-SimpleGUI) · исходный ориентир.
- [WolvenKit](https://github.com/WolvenKit/WolvenKit) · исследование формата.

GPL-3.0-only. Словари CyberCAT используются по MIT.
См. `THIRD_PARTY_NOTICES.md`, `LICENSE`, `assets/CyberCAT-LICENSE`.


---

## English

A native **Rust + egui** save editor with a dark interface in Russian and English
for **Cyberpunk 2077 2.5**. The supported version pair inside `sav.dat` is
`saveVersion = 277`, `gameVersion = 2500`.

Project repository: [desfpc/Cyberpunk25SaveEditor](https://github.com/desfpc/Cyberpunk25SaveEditor).

### Getting started

Download version **0.1.9** — no Rust installation or build step required:

| Platform | Download | Contents |
| --- | --- | --- |
| Windows x64 | [NightCity-Windows-x64.zip](https://github.com/desfpc/Cyberpunk25SaveEditor/raw/refs/heads/main/downloads/NightCity-Windows-x64.zip) | `nightcity-save-editor.exe` |
| macOS Apple Silicon / Intel | [NightCity-macOS-universal.zip](https://github.com/desfpc/Cyberpunk25SaveEditor/raw/refs/heads/main/downloads/NightCity-macOS-universal.zip) | Universal `.app` |

Download the ZIP for your platform, extract it and launch the application.
Packages are stored directly in [`downloads/`](downloads/). Checksums are in
[`downloads/SHA256SUMS.txt`](downloads/SHA256SUMS.txt).
Local packaging commands generate their output in `dist/`.

Switch the interface language in the top bar: **Русский / English**.
Translations apply immediately, including the latest status message. The loaded
save, field values and undo history stay in place. English is selected by default
at startup. Internal game identifiers are not translated.

Use **Open sav.dat** in the top bar to load a save. During development, the
**Open saveExample** button is also available when a local example exists.
The application runs locally without an account or network connection.

**Export copy** asks for a parent folder and creates a new `ManualSave-<number>`
folder containing `sav.dat`, metadata and the screenshot when available. The
written save is decompressed and verified again. The source save is left unchanged,
and an existing destination folder is never overwritten. Undo and redo cover the
last 12 actions. Before opening another file or closing a window with unexported
changes, the application offers to export a copy first.

To use the result, close the game and place the exported folder alongside your
other game saves, then load it from the game's menu. The editor's structural
verification does not replace checking that the edited save loads in the game.

**About us** opens the author’s projects and music page, including before loading
a save.

### Changelog: 0.1.9 → 0.1.2

#### 0.1.9 — inventory browser and levels for modded games (October 29, 2026)

- Added English item names with IDs in parentheses in both UI languages. Inventory is sorted by name, with full details available on hover.
- Added search by item name, record ID and numeric TDBID.
- Added categories for ranged and melee weapons, weapon mods and attachments, ammunition, cyberware, clothing, consumables, components and crafting specs, quickhacks, junk, other and unknown items.
- Added iconic-only and quest-only filters, a result counter and a reset button. Category, search and both filters can be combined.
- Items without a recognized name retain their available ID/TDBID.
- Character level, Street Cred and skills can now exceed standard limits. A warning explains when a game mod is needed to raise the relevant limit; install that mod separately.
- Double-click a number to enter a level with the keyboard.

#### 0.1.8 — facial cyberware and body changes (October 28, 2026)

- Fixed the missing facial-cyberware option at ripperdocs. Repair is automatic when saving or working with presets, including older files.
- Updated the bundled female preset. Working appearance selections, skin tone, cyberarm cosmetics and equipment are retained.
- When cyberware cosmetics cannot transfer to another body, you can cancel the body change or continue with default cosmetics. Installed cyberware, weapons and stats are retained; the change can be undone.

#### 0.1.7 — presets and development points (October 27, 2026)

- Body changes, preset loading and cyberarm repair no longer require saving the current preset first. Dedicated buttons save and load presets.
- Fixed an error that prevented saving presets with certain cyberware cosmetics and transferring them between saves.
- Fixed adding perk and attribute points to fresh saves before the first level-up or point expenditure.

#### 0.1.6 — author page

- Added an About us page available without loading a save, with developer information and links to nozhove.com and GitHub.
- Added YouTube Music links for Čerdyn, Kasapski Nozhove, Gut Splinters and W1NTERH8. The page is available in English and Russian.

#### 0.1.5 — arms disappearing again after a refresh

- Fixed male cyberarms disappearing again after refreshing cyberware appearance in game.
- Cyberarm repair retains selected colors and cosmetics, face, body, voice and equipment.

#### 0.1.4 — cyberarm appearance

- Fixed Gorilla Arms, Mantis Blades and Monowire appearance and restored elemental variants for all four cyberarm types, including Projectile Launcher.
- Added matching male/female appearance variants and a Repair cyberarm appearance button for existing saves.
- Presets now capture cyberware cosmetics and support transferring them between saves. Older presets remain compatible; inventory, installed cyberware, stats and quests are retained.

#### 0.1.3 — presets and save handling

- Added `.ncpreset` files to save and transfer body, voice and complete character appearance. Saving the current appearance is offered before changing body or loading another preset.
- The last opened save folder is remembered between launches.
- Added optional replacement of the current save with a backup in the `old` folder. The option is off by default and resets when another save is opened.

#### 0.1.2 — initial version

- Added independent body and voice selection, Russian and English UI with English by default, undo/redo and export of a separate save copy.

### Features

| Section | Implemented functionality |
| --- | --- |
| Player | Five attributes, level, Street Cred, five skills, development and Relic points |
| Inventory | Containers, money, existing stack quantities, quest flags, English names with IDs, categories and combined filters |
| Add resources | Money, five tiers of crafting components and four ammunition types |
| Appearance | Male/female body and voice controls, advanced editing of appearance definitions and morph targets |
| Garage | Add or remove callable vehicles while preserving unknown entries |
| Quest facts | Search, edit, add by name, ending/romance availability presets and JSON export |
| Diagnostics | Version information, node counts, sizes and write/read-back verification |

The record dictionary contains 40,548 technical names. Separate 2.5 metadata covers
10,156 records with types and 7,283 English item names. Item names stay English
in both UI languages. Missing names or record strings fall back to the available
ID/TDBID. The vehicle catalog comes from CyberCAT; already unlocked vehicles that are
missing from the catalog are also displayed.

Editing a stat synchronizes `PlayerDevelopmentData` and the player's corresponding
forced modifier in the **new StatsSystem 2.5** layout (`valuesOptimized`,
`sharedBufferSlices`, `sharedDataBuffer`). If the modifier is missing, ambiguous or
shared with another entity, the entire edit is rejected. Unknown nodes, fields and
bytes remain in the save. Resizing a node updates offsets and parent node sizes.
Adding an item updates node IDs, links and inventory counts.

Development points can be added before the first level-up or point expenditure.
An omitted zero-valued `unspent` field is created when edited.

Changing body type resets appearance to the selected body's default preset and
synchronizes gender in `PlayerPuppetPS`, preserving the selected voice. Presets
are saved to a `.ncpreset` file only with **Save character preset**. Changing body
type and repairing cyberarms do not open the preset save dialog. Loading a preset
restores its body type, voice and the complete appearance block, including cyberarm
resource groups, morphs, perspectives and tags.
Loading another preset does not require saving the current appearance first.
Presets do not change inventory, equipped cyberware, stats or quests. Version 2
also captures every serialized field of `CyberwareCustomizationSystem`, including
selected type and brand. Package-local name IDs are rebound by name on import.
Voice can still be changed independently, and appearance changes support undo/redo.

The last successfully opened save folder is remembered between launches. Use
**Replace save (backup in old)** to save directly into the loaded save folder.
This switch is always off at startup and resets when another save is opened.
With it off, **Export copy** retains the existing behavior. With it on, **Replace
save** first verifies an edited copy and backs up the current files to a new
`old/backup-<timestamp>/` folder, then replaces the save and its metadata. Each
replacement keeps its own backup. Replacement is refused if the source `sav.dat`
has changed on disk since it was loaded, or if the backup cannot be created.

Body changes preserve cyberarm cosmetic and elemental selections and map verified
male/female appearance definitions. The bundled 2.5 presets fix obsolete Gorilla
Arms, Mantis Blades and Monowire names and include all 16 elemental groups.
The male preset uses native 2.5 FPP/TPP groups, slot names and camera routing for
all four cyberarm types, checked against an originally male save. Repair upgrades
legacy combined groups as well, addressing disappearance after an appearance refresh.
**Repair cyberarm appearance** fixes these known errors in existing saves without
resetting the face, body, voice or equipment. Importing a version 1 preset also
repairs known arm references while retaining the destination's separate cosmetic
settings, which older presets did not capture. When cyberware cosmetics have no verified counterpart for a body change, the editor asks whether to cancel or use default cosmetics. Confirming resets cosmetic settings while retaining equipment and stats; Undo restores the previous state. Other save errors still stop the operation.
In-game rendering still needs to be checked after loading the repaired save.

### Current limitations

This is an initial working version and does not replace every CyberCAT feature.

- Checked against supplied **macOS 2.5 / Phantom Liberty** saves. The user confirmed
  body changes, preset restoration and cyberarm appearance refresh in game.
  Other editable parameters still require their own in-game checks.
- The macOS GUI was checked manually. The Windows EXE was cross-compiled but has
  not yet been run on Windows. CI also supports a native MSVC build.
- Builds are not signed with Apple/Microsoft developer certificates or notarized.
- Appearance editing uses internal game names, without a 3D preview or validation
  that the entered resource exists. Life path changes, preset
  import from CyberCAT and automatic selection of related resources are not implemented.
- Adding arbitrary weapons, clothing, cyberware and item modifiers is not included.
  Existing unique items can be inspected and their quest flags can be changed.
- Quest fact presets cannot guarantee a story outcome for every quest state.
- Other game versions are deliberately rejected until their formats are verified.

### Development

Use the current stable Rust toolchain and system build tools: Xcode Command Line
Tools on macOS, or Visual Studio Build Tools with C++ support on Windows.

```sh
cargo run --release -- saveExample/sav.dat
cargo run --release -- --inspect saveExample
cargo test --locked --lib --bins
# Local integration tests (requires the original saveExample fixture):
cargo test --locked --test real_save
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

```sh
# macOS, for the current architecture
cargo build --release --locked
python3 scripts/package_macos.py

# Windows, in Developer PowerShell
cargo build --release --locked --target x86_64-pc-windows-msvc
```

For a universal macOS package, build `aarch64-apple-darwin` and
`x86_64-apple-darwin`, merge the binaries with `lipo -create`, and pass the result
to `scripts/package_macos.py --binary <path>`.

`.github/workflows/build.yml` builds Windows MSVC and both macOS architectures,
runs checks and uploads ZIP artifacts. Real-save integration tests use the original local
`saveExample/` fixture (`sav.dat`, `metadata.9.json` and `screenshot.png`); some
assertions expect values from that specific save. That folder is excluded from
Git; personal saves are not published.
CI runs the real-save integration tests only when a fixture is available.
Translation tests, formatting, Clippy and builds always run.

The tests cover byte-identical round-trips without edits, reading actual save
data, preserving unknown leaf nodes, adding items/facts, synchronizing stats,
editing vehicles/appearance, truncated files, rejecting unsupported edits and
exporting without changing the source. Translation tests check language switching,
status messages and preserving file paths and internal identifiers.

### Source and licenses

Developed by **Sergei Peshalov** · [GitHub: desfpc](https://github.com/desfpc).

- `src/appearance.rs` — body/voice types and default appearance presets.
- `src/binary.rs` — bounds-checked REDengine VLQ and string reading.
- `src/item_catalog.rs` — item names, categories and filters; data in `assets/item_metadata.json`.
- `src/save.rs` — VASC container, LZ4, nodes, inventory, facts, appearance, garage and export.
- `src/package.rs` — RED packages, player development and optimized 2.5 stats.
- `src/main.rs` — interface and diagnostic CLI.
- `src/i18n.rs` — Russian and English interface translations.
- `tests/real_save.rs` — integration tests using a local `saveExample`.

- [CyberCAT-SimpleGUI](https://github.com/Deweh/CyberCAT-SimpleGUI) · reference project.
- [WolvenKit](https://github.com/WolvenKit/WolvenKit) · save format research.

GPL-3.0-only. CyberCAT dictionaries are used under the MIT license.
See `THIRD_PARTY_NOTICES.md`, `LICENSE` and `assets/CyberCAT-LICENSE`.
