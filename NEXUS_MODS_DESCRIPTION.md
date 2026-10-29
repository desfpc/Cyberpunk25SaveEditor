# Night City Save Editor 0.1.9

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
- Сохранение и загрузка полных пресетов внешности .ncpreset только кнопками, без обязательного резервного пресета перед изменениями.
- Исправление невидимых киберрук после смены пола, перенос косметических настроек имплантов.
- Запоминание последней папки сохранения.
- Отмена и повтор изменений, экспорт отдельной копии с проверкой структуры.
- Замена текущего сейва с резервной копией в old — только при включении отдельной галочки.
- Страница «О нас» со ссылками на проекты и музыку автора.

Это первая рабочая версия для формата 2.5 (saveVersion 277 / gameVersion 2500). Проверена на примере macOS 2.5 / Phantom Liberty; пользователь подтвердил в игре смену тела, восстановление пресета и обновление внешности киберрук. Запуск Windows-сборки пока не проверен. Добавление произвольного уникального снаряжения и 3D-просмотр внешности не включены.

Как использовать: закройте игру, откройте sav.dat, внесите изменения и нажмите «Экспорт копии». Поместите новую папку ManualSave рядом с остальными сохранениями и загрузите её в игре.

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
- Save and load complete .ncpreset appearance presets through buttons, without a mandatory preset backup before changes.
- Repair invisible cyberarms after body changes and preserve cyberware cosmetic selections.
- Remember the last save folder.
- Undo/redo and export to a separate copy with structural verification.
- Optional replacement of the current save with a backup in old; off by default.
- About us page with links to the author’s projects and music.

This is an initial working version for the 2.5 format (saveVersion 277 / gameVersion 2500). Checked against a macOS 2.5 / Phantom Liberty example; the user confirmed body changes, preset restoration and cyberarm appearance refresh in game. The Windows build has not yet been run on Windows. Adding arbitrary unique equipment and a 3D appearance preview are not included.

How to use: close the game, open sav.dat, make your changes and select Export copy. Place the new ManualSave folder alongside your other saves and load it in the game.

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

Developed by Sergei Peshalov.
Project GitHub: https://github.com/desfpc/Cyberpunk25SaveEditor
Developer GitHub: https://github.com/desfpc

CyberCAT-SimpleGUI · reference project
https://github.com/Deweh/CyberCAT-SimpleGUI

WolvenKit · save format research
https://github.com/WolvenKit/WolvenKit

GPL-3.0-only. CyberCAT dictionaries are used under the MIT license.
An unofficial project, not affiliated with CD PROJEKT.
