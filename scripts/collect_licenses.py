#!/usr/bin/env python3
"""Collect dependency license notices for binary distribution (all locked targets)."""
import argparse,json,pathlib,subprocess
p=argparse.ArgumentParser();p.add_argument('--cargo',default='cargo');p.add_argument('--out',default='dist/THIRD_PARTY_LICENSES.txt');a=p.parse_args()
m=json.loads(subprocess.check_output([a.cargo,'metadata','--locked','--offline','--format-version','1']))
parts=['Dependency license notices. Includes dependencies for all supported targets.\n']
for pkg in sorted(m['packages'],key=lambda x:x['name']):
 if not pkg.get('source'):continue
 root=pathlib.Path(pkg['manifest_path']).parent
 parts.append('\n'+'='*72+'\n'+pkg['name']+' '+pkg['version']+'\nLicense: '+str(pkg.get('license'))+'\nSource: '+str(pkg.get('repository') or pkg.get('source'))+'\n')
 files={f for f in root.iterdir() if f.is_file() and any(s in f.name.lower() for s in ['license','licence','copying','notice','copyright'])}
 if pkg.get('license_file'):files.add(root/pkg['license_file'])
 for f in sorted(files):
  if f.is_file():parts.append('\n--- '+f.name+' ---\n'+f.read_text(errors='replace')+'\n')
out=pathlib.Path(a.out);out.parent.mkdir(parents=True,exist_ok=True);out.write_text(''.join(parts));print(out)
