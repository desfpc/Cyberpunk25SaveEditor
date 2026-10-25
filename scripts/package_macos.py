#!/usr/bin/env python3
"""Package a compiled binary without third-party Python modules."""
import argparse, pathlib, plistlib, shutil
p=argparse.ArgumentParser()
p.add_argument('--binary', default='target/release/nightcity-save-editor')
p.add_argument('--out', default='dist/macos/Night City Save Editor.app')
a=p.parse_args()
app=pathlib.Path(a.out)
(app/'Contents/MacOS').mkdir(parents=True,exist_ok=True)
shutil.copy2(a.binary,app/'Contents/MacOS/nightcity-save-editor')
(app/'Contents/MacOS/nightcity-save-editor').chmod(0o755)
(app/'Contents/Resources').mkdir(exist_ok=True)
with (app/'Contents/Info.plist').open('wb') as f:
 plistlib.dump(dict(CFBundleName='Night City Save Editor',CFBundleDisplayName='Night City Save Editor',CFBundleIdentifier='dev.nightcity.saveeditor',CFBundleExecutable='nightcity-save-editor',CFBundlePackageType='APPL',CFBundleShortVersionString='0.1.2',CFBundleVersion='3',LSMinimumSystemVersion='11.0',NSHighResolutionCapable=True),f)
for name in ['LICENSE','README.md','THIRD_PARTY_NOTICES.md']:
 if pathlib.Path(name).exists():shutil.copy2(name,app/'Contents/Resources'/name)
shutil.copy2('assets/CyberCAT-LICENSE',app/'Contents/Resources/CyberCAT-LICENSE')
if pathlib.Path('dist/THIRD_PARTY_LICENSES.txt').exists():shutil.copy2('dist/THIRD_PARTY_LICENSES.txt',app/'Contents/Resources/THIRD_PARTY_LICENSES.txt')
print(app.resolve())
