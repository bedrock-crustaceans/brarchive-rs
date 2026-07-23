# brarchive Format Specification

## Overview

The `.brarchive` format is a simple uncompressed binary archive developed by Mojang for bundling
UTF-8 text files in Minecraft Bedrock Edition resource and behavior packs. It has no compression
of its own; the value comes from bundling many small files into one, which improves external
compression ratios.

## Known Usage Patterns

**Full archive:** Entries contain the actual file content (JSON, etc.).

**Manifest stub:** Entries have `content_len = 0` and `content_offset = 0`. Only the file names
are stored. Used by Mojang as a registry to declare which files belong to a category without
embedding content.

## Binary Layout- Version 1

All multi-byte integers are little-endian.

### Header (16 bytes)

| Offset | Size | Type   | Description                  |
| ------ | ---- | ------ | ---------------------------- |
| 0      | 8    | u64 LE | Magic: `0x267052A0B125277D`  |
| 8      | 4    | u32 LE | Entry count                  |
| 12     | 4    | u32 LE | Format version (must be `1`) |

### Entry Descriptor (256 bytes each, repeated `entry_count` times)

| Offset | Size | Type   | Description                                           |
| ------ | ---- | ------ | ----------------------------------------------------- |
| 0      | 1    | u8     | Name length in bytes (0–247)                          |
| 1      | 247  | bytes  | Entry name, UTF-8, zero-padded to 247 bytes           |
| 248    | 4    | u32 LE | Content offset (relative to start of content section) |
| 252    | 4    | u32 LE | Content length in bytes                               |

### Content Section

Raw UTF-8 bytes packed sequentially. `content_offset` for each entry is relative to the
first byte after all descriptors, i.e.:

```
content_base = 16 + (entry_count × 256)
entry_data   = bytes[content_base + content_offset .. content_base + content_offset + content_len]
```

Two entries may share the same `content_offset` and `content_len`- this is valid and is used
for content deduplication.

## Constraints

- Entry name: 0–247 UTF-8 bytes. Names are not required to be unique by the format, but
  deserializers building maps will keep only the last value for duplicate names.
- Content: arbitrary bytes. Historically UTF-8 text (JSON), but recent Mojang packs
  also embed binary entries — notably compiled `MCB` blobs (magic `7F 4D 43 42`,
  i.e. `\x7F` + `"MCB"`) for particles, cameras, trades, and shapes. Deserializers
  must not assume UTF-8.
- `content_len = 0` is valid (manifest stubs).

## Version History

| Version | Description     |
| ------- | --------------- |
| 1       | Initial version |

## Pack Integration Notes

- Archives are placed under `__brarchive/` inside a pack root, mirroring the directory tree.
  Example: files from `RP/entity/` are archived to `RP/__brarchive/entity.brarchive`.
- `manifest.json` may contain `header.pack_optimization_version` to signal archive usage.
- Directories excluded from archiving in known Mojang tooling:
  `font`, `loot_tables`, `materials`, `scripts`, `sounds`, `subpacks`, `texts`, `textures`.
- The engine appears to prefer the archive when present and falls back to the directory otherwise.
