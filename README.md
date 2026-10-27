# Night City Save Editor

[Русский](#русский) · [English](#english)

## Русский

Нативное приложение на **Rust + egui** с тёмным интерфейсом на русском и английском языках для сохранений
**Cyberpunk 2077 2.5**. Поддерживаемая пара версий внутри `sav.dat`:
`saveVersion = 277`, `gameVersion = 2500`.

Репозиторий проекта: [desfpc/Cyberpunk25SaveEditor](https://github.com/desfpc/Cyberpunk25SaveEditor).

### Запуск

Готовая версия **0.1.7** — установка Rust и сборка не нужны:

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

### История изменений 0.1.2 → 0.1.7

Изменения за **26–27 октября 2026 года**. Версия 0.1.2 — исходная точка сравнения.

#### 0.1.2 — исходная версия

- Независимый выбор тела и голоса, русский и английский интерфейс с английским по умолчанию, отмена/повтор и экспорт отдельной копии сохранения.

#### 0.1.3 — пресеты и сохранение файлов

- Добавлены файлы `.ncpreset`: сохраняются тело, голос и полный блок внешности — ресурсы, морфы, группы рук, настройки камер и теги. Перед сменой тела или загрузкой другого пресета предлагается сохранить текущую внешность; отмена диалога отменяет изменение.
- Последняя успешно открытая папка сейва запоминается между запусками.
- Добавлена замена текущего сейва с отдельной резервной копией в `old/backup-…`. Переключатель по умолчанию выключен и сбрасывается при открытии сейва. Запись проверяется повторным чтением; замена отклоняется, если резервную копию создать нельзя или исходный файл изменился на диске.

#### 0.1.4 — исправление ресурсов киберрук

- Исправлены устаревшие имена внешности Gorilla Arms, Mantis Blades и Monowire; восстановлены 16 элементальных групп для четырёх типов киберрук, включая Projectile Launcher.
- Добавлены подбор проверенных мужских/женских вариантов и кнопка «Исправить киберруки» для существующих сейвов.
- Пресеты версии 2 сохраняют отдельные косметические настройки имплантов (`CyberwareCustomizationSystem`) и поддерживают перенос между сейвами. Старые пресеты версии 1 остаются совместимыми; инвентарь, установленные импланты, характеристики и квесты не заменяются содержимым пресета.

#### 0.1.5 — исправление повторного исчезновения рук

- По исходному мужскому сейву обновлена структура: 34 группы рук, отдельные слоты FPP/TPP и 5 связей переключения камер. Ремонт переводит старые объединённые группы в эту структуру, устраняя причину повторного исчезновения рук при обновлении внешности имплантов в игре.
- Ремонт сохраняет выбранные цвета и косметику, лицо, тело, голос и экипировку. Пользователь подтвердил в игре работу мужских рук после обновления внешности и восстановление женской внешности с руками из пресета.

#### 0.1.6 — страница автора

- Добавлена кнопка «О нас / About us» и отдельная страница, доступная без загрузки сейва: короткое описание разработчика, nozhove.com и GitHub.
- Добавлены музыкальные проекты Čerdyn, Kasapski Nozhove, Gut Splinters и W1NTERH8 со ссылками на YouTube Music. Страница переведена на русский и английский.

#### 0.1.7 — пресеты и очки развития (27 октября 2026)

- Сохранение и загрузка пресетов выполняются только кнопками. Смена тела, загрузка пресета и ремонт киберрук больше не требуют предварительного сохранения текущего пресета.
- Исправлена ошибка «Unsupported cosmetic value type gameItemID; preset capture cancelled»: вложенный gameItemID в косметических настройках имплантов теперь сохраняется и переносится между сейвами с корректным сопоставлением имён полей.
- Исправлено добавление очков способностей и атрибутов в новых сейвах до первого повышения уровня или траты очков. Если поле unspent и его имя отсутствуют, редактор создаёт их при изменении; наличие spent не требуется.
- Добавлен регрессионный тест для отсутствующих полей очков; расширена проверка переноса косметики с gameItemID.

Для версии 0.1.7 обновлены Windows x64 и универсальный macOS-пакет, версии приложений и контрольные суммы. Прошли 28 тестов, проверка форматирования и Clippy. Новые исправления не проверялись в игре; запуск Windows-сборки пока не проверен.

### Возможности

| Раздел | Что реализовано |
| --- | --- |
| Персонаж | Пять атрибутов, уровень, репутация, пять навыков, очки развития и биочипа |
| Инвентарь | Контейнеры, деньги, количества существующих стеков, квестовые флаги предметов, поиск |
| Добавление ресурсов | Деньги, компоненты пяти уровней, четыре вида боеприпасов |
| Внешность | Мужское/женское тело и голос, расширенный редактор appearance definitions и morph targets |
| Гараж | Добавление/удаление доступного для вызова транспорта, сохранение неизвестных записей |
| Квестовые флаги | Поиск, редактирование, добавление по имени, пресеты доступности концовки/романов, JSON-экспорт |
| Диагностика | Версии, число узлов, размеры, повторная проверка записи |

Словарь содержит 40 548 технических названий предметов. Неизвестные записи
сохраняются и показываются как `TDBID ...`. Предметы не переименовываются в
локализованные названия игры. Каталог транспорта взят из CyberCAT; неизвестные
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
операция отклоняется, если для него нельзя проверить соответствие.

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

Download version **0.1.7** — no Rust installation or build step required:

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

### Changelog: 0.1.2 → 0.1.7

Changes made on **October 26–27, 2026**. Version 0.1.2 is the comparison baseline.

#### 0.1.2 — baseline

- Independent body and voice controls, Russian/English UI with English selected by default, undo/redo and export to a separate save copy.

#### 0.1.3 — presets and save handling

- Added `.ncpreset` files capturing body, voice and the entire appearance block: resources, morphs, arm groups, camera mappings and tags. Before changing body or loading another preset, the current appearance must be saved; cancelling the dialog cancels the change.
- The last successfully opened save folder is remembered between launches.
- Added optional replacement of the current save with a separate backup in `old/backup-…`. The switch is off by default and resets when a save is opened. Output is verified by reading it back; replacement is refused if backup creation fails or the source file has changed on disk.

#### 0.1.4 — cyberarm resource fixes

- Fixed obsolete Gorilla Arms, Mantis Blades and Monowire appearance names; restored 16 elemental groups for all four cyberarm types, including Projectile Launcher.
- Added verified male/female appearance mapping and a Repair cyberarm appearance button for existing saves.
- Version 2 presets capture the separate cyberware cosmetic settings (`CyberwareCustomizationSystem`) and support transfer between saves. Version 1 presets remain compatible; presets do not replace inventory, equipped cyberware, stats or quests.

#### 0.1.5 — arms disappearing again after a refresh

- Updated the male layout against a natively male save: 34 arm groups, separate FPP/TPP slots and 5 camera mappings. Repair upgrades legacy combined groups, addressing arms disappearing again after an in-game cyberware appearance refresh.
- Repair retains selected colors and cosmetics, face, body, voice and equipment. The user confirmed male arms work after refreshing their appearance and a female preset restores both appearance and arms in game.

#### 0.1.6 — author page

- Added an About us button and a separate page available before loading a save, with a short developer biography, nozhove.com and GitHub links.
- Added YouTube Music links for Čerdyn, Kasapski Nozhove, Gut Splinters and W1NTERH8. The page supports Russian and English.

#### 0.1.7 — presets and development points (October 27, 2026)

- Presets are saved and loaded only through their buttons. Body changes, preset loading and cyberarm repair no longer require saving the current preset first.
- Fixed “Unsupported cosmetic value type gameItemID; preset capture cancelled”: nested gameItemID values in cyberware cosmetics are now captured and transferred between saves with field names rebound correctly.
- Fixed adding perk and attribute points to fresh saves before the first level-up or point expenditure. Missing unspent fields and name-pool entries are created when edited; spent is not required.
- Added a regression test for omitted point fields and extended cosmetic transfer coverage for gameItemID.

Version 0.1.7 updates the Windows x64 and universal macOS packages, application versions and checksums. All 28 tests, formatting checks and Clippy passed. The new fixes have not been verified in game; the Windows build has not yet been run on Windows.

### Features

| Section | Implemented functionality |
| --- | --- |
| Player | Five attributes, level, Street Cred, five skills, development and Relic points |
| Inventory | Containers, money, existing stack quantities, item quest flags and search |
| Add resources | Money, five tiers of crafting components and four ammunition types |
| Appearance | Male/female body and voice controls, advanced editing of appearance definitions and morph targets |
| Garage | Add or remove callable vehicles while preserving unknown entries |
| Quest facts | Search, edit, add by name, ending/romance availability presets and JSON export |
| Diagnostics | Version information, node counts, sizes and write/read-back verification |

The dictionary contains 40,548 technical item names. Unknown records are preserved
and displayed as `TDBID ...`. Items are not renamed to the game's localized display
names. The vehicle catalog comes from CyberCAT; already unlocked vehicles that are
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
settings, which older presets did not capture. Unknown gendered mod resources
without verified counterparts are rejected instead of being guessed.
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
- `src/save.rs` — VASC container, LZ4, nodes, inventory, facts, appearance, garage and export.
- `src/package.rs` — RED packages, player development and optimized 2.5 stats.
- `src/main.rs` — interface and diagnostic CLI.
- `src/i18n.rs` — Russian and English interface translations.
- `tests/real_save.rs` — integration tests using a local `saveExample`.

- [CyberCAT-SimpleGUI](https://github.com/Deweh/CyberCAT-SimpleGUI) · reference project.
- [WolvenKit](https://github.com/WolvenKit/WolvenKit) · save format research.

GPL-3.0-only. CyberCAT dictionaries are used under the MIT license.
See `THIRD_PARTY_NOTICES.md`, `LICENSE` and `assets/CyberCAT-LICENSE`.
