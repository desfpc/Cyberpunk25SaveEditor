#!/usr/bin/env python3
"""Extract record names from an already decompressed WolvenKit TweakDB string table.
Usage: python3 scripts/extract_item_catalog.py /path/to/uncompressed/table
No Oodle library is needed in the shipped application.
"""
import struct,sys,json,pathlib
b=pathlib.Path(sys.argv[1]).read_bytes()
magic,version,records,flats,queries=struct.unpack_from('<IIIII',b)
assert magic==0x0BB1DB57 and version==1
pos=20
names=[]
for _ in range(records):
 v=b[pos];pos+=1;negative=v&128;n=v&63;shift=6;more=v&64
 while more:
  v=b[pos];pos+=1;n|=(v&127)<<shift;shift+=7;more=v&128
 size=n if negative else n*2
 value=b[pos:pos+size].decode('utf8' if negative else 'utf-16-le');pos+=size
 if value.startswith('Items.'):names.append(value)
pathlib.Path('assets/items.json').write_text(json.dumps(sorted(set(names)),indent=2)+'\n')
print(f'{len(names)} item record names extracted')
