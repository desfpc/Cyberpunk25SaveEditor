#!/usr/bin/env python3
"""Build the read-only inventory dictionary from local Cyberpunk 2077 2.5 data.

Pass base-game then expansion tweakdb files, and already decompressed English
onscreens.json CR2W resources. Archive extraction/decompression is external;
neither game resources nor a decompression library are shipped with the app.
Only item names, record IDs, item types and iconic flags are exported.
"""
import argparse
import json
import pathlib
import struct
import zlib


def fnv(value):
    result = 14695981039346656037
    for byte in value.encode():
        result = ((result ^ byte) * 1099511628211) & ((1 << 64) - 1)
    return result


def tdb(value):
    return zlib.crc32(value.encode()) | len(value.encode()) << 32


def flat_id(record, prop):
    suffix = ('.' + prop).encode()
    return zlib.crc32(suffix, record & 0xFFFFFFFF) | (
        ((record >> 32) & 255) + len(suffix)
    ) << 32


class Reader:
    def __init__(self, data, pos=0):
        self.data, self.pos = data, pos

    def unpack(self, fmt):
        result = struct.unpack_from('<' + fmt, self.data, self.pos)
        self.pos += struct.calcsize('<' + fmt)
        return result[0] if len(result) == 1 else result

    def vlq(self):
        byte = self.unpack('B')
        negative, value, shift, more = byte & 128, byte & 63, 6, byte & 64
        while more:
            byte = self.unpack('B')
            value |= (byte & 127) << shift
            shift += 7
            more = byte & 128
        return -value if negative else value

    def read(self, kind):
        if kind.startswith('array:'):
            return [self.read(kind[6:]) for _ in range(self.vlq())]
        if kind in ('CName', 'String'):
            count = self.vlq()
            size = abs(count) if count <= 0 else count * 2
            result = self.data[self.pos:self.pos + size].decode(
                'utf-8' if count <= 0 else 'utf-16-le')
            self.pos += size
            return result
        return self.unpack({'TweakDBID': 'Q', 'gamedataLocKeyWrapper': 'Q', 'Float': 'f', 'Int32': 'i',
                            'Bool': '?', 'Uint8': 'B'}[kind])


def localization(path):
    data = path.read_bytes()
    assert data[:4] == b'CR2W'
    tables = [struct.unpack_from('<III', data, 40 + 12 * i) for i in range(10)]
    names = []
    for i in range(tables[1][1]):
        pos = tables[0][0] + struct.unpack_from('<I', data, tables[1][0] + 8 * i)[0]
        names.append(data[pos:data.index(0, pos)].decode())

    def fields(pos):
        assert data[pos] == 0
        pos += 1
        values = {}
        while True:
            name = struct.unpack_from('<H', data, pos)[0]
            pos += 2
            if not name:
                return values, pos
            kind, size = struct.unpack_from('<HI', data, pos)
            pos += 6
            values[names[name]] = (names[kind], pos, size - 4)
            pos += size - 4

    result = {}
    for i in range(tables[4][1]):
        cls, _, _, _, pos, _, _ = struct.unpack_from(
            '<HHIIIII', data, tables[4][0] + 24 * i)
        if names[cls] != 'localizationPersistenceOnScreenEntries':
            continue
        _, pos, _ = fields(pos)[0]['entries']
        count = struct.unpack_from('<I', data, pos)[0]
        pos += 4
        for _ in range(count):
            entry, pos = fields(pos)
            key = struct.unpack_from('<Q', data, entry['primaryKey'][1])[0]
            text = Reader(data, entry['femaleVariant'][1]).read('String') if 'femaleVariant' in entry else ''
            if not text and 'maleVariant' in entry:
                text = Reader(data, entry['maleVariant'][1]).read('String')
            result[key] = text
            if 'secondaryKey' in entry:
                secondary = Reader(data, entry['secondaryKey'][1]).read('String')
                if secondary:
                    result[fnv(secondary)] = text
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--tweakdb', nargs='+', type=pathlib.Path, required=True)
    parser.add_argument('--onscreens', nargs='+', type=pathlib.Path, required=True)
    parser.add_argument('--records', type=pathlib.Path, default=pathlib.Path('assets/items.json'))
    parser.add_argument('--out', type=pathlib.Path, default=pathlib.Path('assets/item_metadata.json'))
    args = parser.parse_args()
    records = {tdb(name): name for name in json.loads(args.records.read_text())}
    record_types = {}
    for path in args.tweakdb:
        data = path.read_bytes()
        start = struct.unpack_from('<I', data, 20)[0]
        for i in range(struct.unpack_from('<I', data, start)[0]):
            record, record_type = struct.unpack_from('<QI', data, start + 4 + 12 * i)
            record_types[record] = record_type
            records.setdefault(record, f'TDBID {record:010X}')
    props = ('displayName', 'itemType', 'tags', 'itemCategory')
    wanted = {flat_id(record, prop) for record in records for prop in props}
    types = {fnv(kind): kind for kind in ('TweakDBID', 'CName', 'String', 'Float',
             'Int32', 'Bool', 'Uint8', 'gamedataLocKeyWrapper', 'array:TweakDBID', 'array:CName', 'array:Float')}
    # itemType records expose an enum name via their .name flat. Keep CNames too.
    flats = {}
    for path in args.tweakdb:
        data = path.read_bytes()
        assert struct.unpack_from('<II', data) == (0x0BB1DB47, 8)
        start = struct.unpack_from('<I', data, 16)[0]
        count = struct.unpack_from('<I', data, start)[0]
        for i in range(count):
            kind_hash, _, _, pos = struct.unpack_from('<QIII', data, start + 4 + 20 * i)
            if kind_hash not in types:
                continue
            kind = types[kind_hash]
            reader = Reader(data, pos)
            values = [reader.read(kind) for _ in range(reader.unpack('I'))]
            for _ in range(reader.unpack('I')):
                key, index = reader.unpack('QI')
                if key in wanted or kind == 'CName':
                    flats[key] = values[index]
    strings = {}
    for path in args.onscreens:
        strings.update(localization(path))
    result = {}
    for record, name in sorted(records.items(), key=lambda pair: pair[1]):
        type_id = flats.get(flat_id(record, 'itemType'))
        category_id = flats.get(flat_id(record, 'itemCategory'))
        if not isinstance(type_id, int) or not type_id:
            continue  # Service records and modifier definitions are not inventory items.
        kind = flats.get(flat_id(type_id, 'name'), '') if type_id else ''
        category = flats.get(flat_id(category_id, 'name'), '') if isinstance(category_id, int) and category_id else ''
        display = flats.get(flat_id(record, 'displayName'), '')
        if isinstance(display, str):
            key = int(display[7:]) if display.startswith('LocKey#') else fnv(display)
            display = strings.get(key, '')
        else:
            display = strings.get(display, '')
        tags = flats.get(flat_id(record, 'tags'), [])
        if not isinstance(tags, list):
            tags = []
        # A handful of readable records use whole messages as displayName.
        # They are not item titles and do not belong in this dictionary.
        if len(display) > 160 or '\\n' in display or '\n' in display:
            display = ''
        result[name] = {'name': display.strip(), 'type': kind,
                        'iconic': 'IconicWeapon' in tags or 'IconicItem' in tags,
                        'category': category, 'recipe': 'Recipe' in tags}
    inventory_types = {record_types[tdb(name)] for name in result
                       if name.startswith('Items.') and tdb(name) in record_types}
    result = {name: meta for name, meta in result.items()
              if record_types.get(int(name[6:], 16) if name.startswith('TDBID ') else tdb(name)) in inventory_types}
    args.out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(f'{len(result)} inventory records, {sum(bool(v["name"]) for v in result.values())} English names')


if __name__ == '__main__':
    main()
