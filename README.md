# Night City Save Editor

[Русский](#русский) · [English](#english)

## Русский

Нативное приложение на **Rust + egui** с тёмным интерфейсом на русском и английском языках для сохранений
**Cyberpunk 2077 2.5**. Поддерживаемая пара версий внутри `sav.dat`:
`saveVersion = 277`, `gameVersion = 2500`.

Репозиторий проекта: [desfpc/Cyberpunk25SaveEditor](https://github.com/desfpc/Cyberpunk25SaveEditor).

### Запуск

Готовая версия **0.1.2** — установка Rust и сборка не нужны:

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

Изменение характеристики синхронизирует `PlayerDevelopmentData` и соответствующий
forced modifier игрока в **новом StatsSystem 2.5** (`valuesOptimized`,
`sharedBufferSlices`, `sharedDataBuffer`). Если модификатор отсутствует,
неоднозначен или разделяется с другой сущностью, операция отклоняется целиком.
Неизвестные узлы, поля и байты остаются в сохранении. При изменении размера
узла пересчитываются смещения и размеры родительских узлов. При добавлении
предмета обновляются ID узлов, ссылки и счётчики инвентаря.

Смена типа тела сбрасывает внешность к базовому пресету выбранного типа и
синхронизирует пол в `PlayerPuppetPS`. Выбранный голос сохраняется. Переключатель
голоса меняет его независимо от тела. Оба действия поддерживают отмену.
Экспорт создаёт отдельную копию, оригинал не изменяется.

### Текущие границы

Это первая рабочая версия, а не полная замена всех функций CyberCAT.

- Проверено предоставленное сохранение **macOS 2.5 / Phantom Liberty**;
  загрузка изменённой копии в самой игре пока не проверена.
- macOS GUI проверен вручную. Windows EXE собран кросс-компиляцией;
  запуск на Windows пока не проверен. CI также поддерживает нативную MSVC-сборку.
- Релизы не подписаны сертификатами Apple/Microsoft и не нотарифицированы.
- Внешность редактируется по внутренним именам игры, без 3D-просмотра и
  гарантии существования введённого ресурса. Смена жизненного пути,
  импорт пресетов и автоматический подбор связанных ресурсов не реализованы.
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

Download version **0.1.2** — no Rust installation or build step required:

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

Changing body type resets appearance to the selected body's default preset and
synchronizes gender in `PlayerPuppetPS`, preserving the selected voice. Voice
can be changed independently. Both edits support undo; the original save remains
unchanged when exporting a new copy.

### Current limitations

This is an initial working version and does not replace every CyberCAT feature.

- Checked against the supplied **macOS 2.5 / Phantom Liberty** save. Loading an
  edited copy in the game has not yet been verified.
- The macOS GUI was checked manually. The Windows EXE was cross-compiled but has
  not yet been run on Windows. CI also supports a native MSVC build.
- Builds are not signed with Apple/Microsoft developer certificates or notarized.
- Appearance editing uses internal game names, without a 3D preview or validation
  that the entered resource exists. Life path changes, preset
  import and automatic selection of related resources are not implemented.
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
