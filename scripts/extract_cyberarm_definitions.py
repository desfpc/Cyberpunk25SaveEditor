#!/usr/bin/env python3
"""Extract real appearance names from already decompressed 2.5 cyberarm CR2W files.

Input files must be named <decimal resource hash>.cr2w. Use an archive extractor
against your own game installation; no game archive or decompressor is distributed.
Unlike a string search, this reads only appearanceAppearanceDefinition.name fields
and excludes similarly named dismemberment configurations.
"""
import argparse
import json
from pathlib import Path
import struct


def definitions(path):
    data = path.read_bytes()
    if data[:4] != b"CR2W":
        raise ValueError(f"Not a CR2W resource: {path}")
    tables = [struct.unpack_from("<III", data, 40 + 12 * i) for i in range(10)]
    strings = tables[0][0]
    start, count, _ = tables[1]
    names = []
    for i in range(count):
        offset = strings + struct.unpack_from("<I", data, start + 8 * i)[0]
        names.append(data[offset:data.index(0, offset)].decode())
    start, count, _ = tables[4]
    result = []
    for i in range(count):
        cls, _, _, _, offset, _, _ = struct.unpack_from("<HHIIIII", data, start + 24 * i)
        if names[cls] != "appearanceAppearanceDefinition":
            continue
        position = offset + 1
        while True:
            name = struct.unpack_from("<H", data, position)[0]
            position += 2
            if name == 0:
                break
            _, length = struct.unpack_from("<HI", data, position)
            position += 6
            if length < 4:
                raise ValueError("Invalid property size")
            if names[name] == "name":
                result.append(names[struct.unpack_from("<H", data, position)[0]])
            position += length - 4
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--out", type=Path, default=Path("assets/cyberarm_definitions_25.json"))
    args = parser.parse_args()
    resources = set()
    for gender in ("male", "female"):
        preset = json.loads(Path(f"assets/appearance_{gender}.json").read_text())["Preset"]
        resources.update(str(a["resource"]["DepotPath"]) for g in preset["armsGroups"] for a in g["customization"])
    catalog = {resource: definitions(args.directory / f"{resource}.cr2w") for resource in sorted(resources)}
    if any(not values for values in catalog.values()):
        raise ValueError("A cyberarm resource has no appearance definitions")
    args.out.write_text(json.dumps(catalog, indent=2) + "\n")
    print(f"Wrote {len(catalog)} resources to {args.out}")


if __name__ == "__main__":
    main()
