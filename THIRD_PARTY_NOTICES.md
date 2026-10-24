# Third-party notices

Night City Save Editor is distributed under GPL-3.0-only; see LICENSE.

The binary format implementation was developed with reference to WolvenKit's
save readers and writers (GPL-3.0), including LZ4 framing, RED packages, facts,
items, appearance and persistency data. The optimized 2.5 stats layout was
investigated against the supplied macOS save.

- WolvenKit: https://github.com/WolvenKit/WolvenKit
- Reference commit: 4e0c851f6b192822a139f7adf101a7609f86a431
- assets/items.json: item record names extracted from WolvenKit.Common/Resources/tweakdbstr.kark.
  Only the record-name catalog is shipped; no Oodle/kraken library is required at runtime.
  The extraction script accepts an already decompressed table.

The fact and vehicle dictionaries are from CyberCAT-SimpleGUI (MIT):

- https://github.com/Deweh/CyberCAT-SimpleGUI
- Reference commit: c1fd00305e7db905a4e7de8cfa360d4f4efee3b2
- assets/facts.json, assets/vehicles.json
- Copyright and permission notice: assets/CyberCAT-LICENSE

Cargo dependency licenses are specified by the corresponding packages and are
recorded by the exact versions in Cargo.lock. Main runtime dependencies include
eframe/egui (MIT OR Apache-2.0), rfd (MIT), lz4_flex (MIT), anyhow
(MIT OR Apache-2.0), serde (MIT OR Apache-2.0), serde_json (MIT OR Apache-2.0),
and crc32fast (MIT OR Apache-2.0). The applications do not contain or require
CyberCAT, WolvenKit, .NET, or a local game installation at runtime.

Cyberpunk 2077 is a trademark of CD PROJEKT. This is an unofficial project.
