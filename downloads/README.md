# Night City Save Editor 0.1.8

## Русский

Готовые пакеты для скачивания без сборки:

- [Windows x64](NightCity-Windows-x64.zip) — распакуйте ZIP и запустите `nightcity-save-editor.exe`.
- [macOS Apple Silicon / Intel](NightCity-macOS-universal.zip) — распакуйте ZIP и откройте `Night City Save Editor.app`.

По умолчанию выбран English; русский доступен в переключателе языка.
В разделе «Внешность» доступны независимые переключатели тела и голоса; смена тела
сбрасывает внешность к базовому пресету выбранного типа и поддерживает отмену. Поддерживается Cyberpunk 2077 2.5.
Добавлены полные пресеты персонажа `.ncpreset`, запоминание последней папки и
замена сейва с резервной копией в `old` (по умолчанию выключена).
В 0.1.5 исправлены мужские FPP/TPP-группы и маршруты камер киберрук: ремонт
обновляет старую структуру для повторного обновления внешности в игре. Пресеты теперь включают отдельные косметические настройки имплантов.
В 0.1.6 добавлена страница «О нас»: проекты, GitHub и музыка автора.
В 0.1.7 убраны обязательные диалоги сохранения пресета, исправлены захват косметики
с gameItemID и добавление очков развития в новые сейвы без поля unspent.
В 0.1.8 автоматически исправляется исчезнувший пункт лицевых киберимплантов при экспорте и работе с пресетами, включая старые файлы. Дополнительных действий не требуется.
Если косметику имплантов нельзя перенести на другое тело, редактор предлагает отмену или смену тела со сбросом косметики; экипировка сохраняется, доступен Undo.
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
0.1.5 repairs male cyberarm FPP/TPP groups, slot names and camera routes, including
legacy saves whose arms disappeared again after an in-game appearance refresh.
Presets now include the separate cyberware cosmetic settings.
0.1.6 adds an About us page with the author’s projects, GitHub profile and music.
0.1.7 removes mandatory preset save dialogs and fixes gameItemID cosmetic capture
and adding development points to fresh saves without an unspent field.
0.1.8 automatically repairs the missing facial-cyberware control on export and during preset capture/loading, including older presets. No extra steps are required.
If cyberware cosmetics cannot transfer to the other body, a warning offers cancellation or a body change with default cosmetics. Equipment is retained and Undo is available.
SHA-256 checksums are provided in [SHA256SUMS.txt](SHA256SUMS.txt).

Packages are not signed with Apple/Microsoft developer certificates or notarized.
The macOS GUI was checked manually; the Windows build and in-game loading of edited
saves have not yet been verified. See the [project README](../README.md) for full details.
