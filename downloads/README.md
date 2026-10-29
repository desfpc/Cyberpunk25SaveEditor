# Night City Save Editor 0.1.9

## Русский

Готовые пакеты для скачивания без сборки:

- [Windows x64](NightCity-Windows-x64.zip) — распакуйте ZIP и запустите `nightcity-save-editor.exe`.
- [macOS Apple Silicon / Intel](NightCity-macOS-universal.zip) — распакуйте ZIP и откройте `Night City Save Editor.app`.

По умолчанию выбран English; русский доступен в переключателе языка.
В разделе «Внешность» доступны независимые переключатели тела и голоса; смена тела
сбрасывает внешность к базовому пресету выбранного типа и поддерживает отмену. Поддерживается Cyberpunk 2077 2.5.
Добавлены полные пресеты персонажа `.ncpreset`, запоминание последней папки и
замена сейва с резервной копией в `old` (по умолчанию выключена).
Последние изменения (от новых версий к старым):

- **0.1.9:** английские названия предметов с ID, поиск по названию/ID, категории и совместные фильтры культовых и квестовых предметов. Уровни персонажа, репутации и навыков можно повышать выше стандартных лимитов с соответствующим игровым модом; доступен ввод с клавиатуры двойным щелчком.
- **0.1.8:** автоматически исправляется исчезнувший пункт лицевых киберимплантов. Если косметику нельзя перенести на другое тело, можно отменить смену тела или продолжить со стандартной косметикой, сохранив экипировку.
- **0.1.7:** убраны обязательные диалоги сохранения пресета; исправлены сохранение косметики имплантов в пресетах и добавление очков развития в новых сейвах.
- **0.1.6:** добавлена страница «О нас» со ссылками на проекты, GitHub и музыку автора.
- **0.1.5:** исправлено повторное исчезновение мужских киберрук после обновления внешности имплантов в игре.

Контрольные суммы SHA-256 находятся в [SHA256SUMS.txt](SHA256SUMS.txt).

Пакеты не подписаны сертификатами Apple/Microsoft и не нотарифицированы.
macOS GUI проверен вручную; запуск Windows-сборки и загрузка изменённого сохранения
в игре пока не проверены. Полные возможности и ограничения — в [README проекта](../README.md).

## English

Ready-to-run packages; no compilation required:

- [Windows x64](NightCity-Windows-x64.zip) — extract the ZIP and run `nightcity-save-editor.exe`.
- [macOS Apple Silicon / Intel](NightCity-macOS-universal.zip) — extract the ZIP and open `Night City Save Editor.app`.

English is selected by default; Russian is available in the language selector.
Appearance includes independent body and voice controls. Changing body type resets
appearance to its default preset and can be undone. Cyberpunk 2077 2.5 is supported.
Includes complete `.ncpreset` character presets, the remembered last save folder,
and optional save replacement with backups in `old` (off by default).
Recent changes (newest first):

- **0.1.9:** English item names with IDs, name/ID search, categories and combined iconic/quest filters. Character level, Street Cred and skills can exceed standard limits with a suitable game mod; double-click a number to enter a level with the keyboard.
- **0.1.8:** automatic repair of the missing facial-cyberware option. If cosmetics cannot transfer to another body, you can cancel or continue with default cosmetics while retaining equipment.
- **0.1.7:** removed mandatory preset save dialogs; fixed cyberware cosmetic capture in presets and adding development points to fresh saves.
- **0.1.6:** added an About us page with links to the author’s projects, GitHub and music.
- **0.1.5:** fixed male cyberarms disappearing again after refreshing cyberware appearance in game.

SHA-256 checksums are provided in [SHA256SUMS.txt](SHA256SUMS.txt).

Packages are not signed with Apple/Microsoft developer certificates or notarized.
The macOS GUI was checked manually; the Windows build and in-game loading of edited
saves have not yet been verified. See the [project README](../README.md) for full details.
