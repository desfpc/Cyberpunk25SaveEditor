# Night City Save Editor 0.1.6

[Русский](#русский) · [English](#english)

## Русский

Night City Save Editor — Cyberpunk 2077 2.5

Нативный редактор сохранений на Rust с тёмным интерфейсом и переключением Русский / English. Доступны версии для Windows x64 и macOS (Apple Silicon / Intel).

### Возможности

- Изменение атрибутов, уровня, репутации, навыков и очков развития V.
- Редактирование денег, количества предметов и квестовых флагов предметов.
- Добавление денег, компонентов и боеприпасов.
- Настройка доступного для вызова транспорта.
- Поиск, изменение и добавление квестовых флагов, JSON-экспорт.
- Выбор мужского/женского тела и голоса; смена тела сбрасывает внешность к базовому пресету.
- Расширенное редактирование записей внешности по внутренним именам игры.
- Сохранение и загрузка полных пресетов внешности .ncpreset; резервная копия пресета перед сменой тела.
- Исправление невидимых киберрук после смены пола, перенос косметических настроек имплантов.
- Запоминание последней папки сохранения.
- Отмена и повтор изменений, экспорт отдельной копии с проверкой структуры.
- Замена текущего сейва с резервной копией в old — только при включении отдельной галочки.
- Страница «О нас» со ссылками на проекты и музыку автора.

Это первая рабочая версия для формата 2.5 (saveVersion 277 / gameVersion 2500). Проверена на примере macOS 2.5 / Phantom Liberty; пользователь подтвердил в игре смену тела, восстановление пресета и обновление внешности киберрук. Запуск Windows-сборки пока не проверен. Добавление произвольного уникального снаряжения и 3D-просмотр внешности не включены.

Как использовать: закройте игру, откройте sav.dat, внесите изменения и нажмите «Экспорт копии». Поместите новую папку ManualSave рядом с остальными сохранениями и загрузите её в игре.

### История изменений 0.1.2 → 0.1.6

Изменения за **26 октября 2026 года**. Версия 0.1.2 — исходная точка сравнения.

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

Для релизов обновлены Windows x64 и универсальные macOS-пакеты, версии приложений и контрольные суммы. Для 0.1.6 прошли 27 тестов и Clippy; запуск Windows-сборки пока не проверен.

Разработал Sergei Peshalov.
GitHub проекта: https://github.com/desfpc/Cyberpunk25SaveEditor
GitHub разработчика: https://github.com/desfpc

CyberCAT-SimpleGUI · исходный ориентир
https://github.com/Deweh/CyberCAT-SimpleGUI

WolvenKit · исследование формата
https://github.com/WolvenKit/WolvenKit

GPL-3.0-only. Словари CyberCAT используются по MIT.
Неофициальный проект, не связанный с CD PROJEKT.


## English

Night City Save Editor — Cyberpunk 2077 2.5

A native Rust save editor with a dark interface and instant Russian / English switching. Available for Windows x64 and macOS (Apple Silicon / Intel).

### Features

- Edit V's attributes, level, Street Cred, skills and development points.
- Change money, item quantities and item quest flags.
- Add money, crafting components and ammunition.
- Manage vehicles available in the vehicle call list.
- Search, edit and add quest facts; export facts as JSON.
- Select male/female body and voice; changing body type resets appearance to its default preset.
- Advanced appearance editing using internal game names.
- Save and load complete .ncpreset appearance presets; preset backup before changing body type.
- Repair invisible cyberarms after body changes and preserve cyberware cosmetic selections.
- Remember the last save folder.
- Undo/redo and export to a separate copy with structural verification.
- Optional replacement of the current save with a backup in old; off by default.
- About us page with links to the author’s projects and music.

This is an initial working version for the 2.5 format (saveVersion 277 / gameVersion 2500). Checked against a macOS 2.5 / Phantom Liberty example; the user confirmed body changes, preset restoration and cyberarm appearance refresh in game. The Windows build has not yet been run on Windows. Adding arbitrary unique equipment and a 3D appearance preview are not included.

How to use: close the game, open sav.dat, make your changes and select Export copy. Place the new ManualSave folder alongside your other saves and load it in the game.

### Changelog: 0.1.2 → 0.1.6

Changes made on **October 26, 2026**. Version 0.1.2 is the comparison baseline.

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

Windows x64 and universal macOS packages, application versions and checksums were updated for these releases. Version 0.1.6 passed 27 tests and Clippy; the Windows build has not yet been run on Windows.

Developed by Sergei Peshalov.
Project GitHub: https://github.com/desfpc/Cyberpunk25SaveEditor
Developer GitHub: https://github.com/desfpc

CyberCAT-SimpleGUI · reference project
https://github.com/Deweh/CyberCAT-SimpleGUI

WolvenKit · save format research
https://github.com/WolvenKit/WolvenKit

GPL-3.0-only. CyberCAT dictionaries are used under the MIT license.
An unofficial project, not affiliated with CD PROJEKT.
