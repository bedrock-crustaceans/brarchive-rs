# brarchive-rs

[![Crates.io Version](https://img.shields.io/crates/v/brarchive)](https://crates.io/crates/brarchive)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/brarchive)](https://crates.io/crates/brarchive)
[![Crates.io MSRV (version)](https://img.shields.io/crates/msrv/brarchive/0.2.1)](https://crates.io/crates/brarchive)
[![Crates.io License](https://img.shields.io/crates/l/brarchive)](https://github.com/theaddonn/brarchive-rs/blob/main/LICENSE)

Library and CLI for the Bedrock Archive (`.brarchive`) format- Mojang's bundling format for
UTF-8 text files in Minecraft Bedrock Edition resource and behavior packs.

See [FORMAT.md](FORMAT.md) for the binary format specification.

## Library Usage

```rust
// List entry names without reading content
let names: Vec<String> = brarchive::list(&bytes)?;

// Deserialize- collect into any type that implements FromIterator<(String, String)>
let map: std::collections::BTreeMap<_, _> = brarchive::deserialize(&bytes)?;
let vec: Vec<(String, String)>            = brarchive::deserialize(&bytes)?;

// Serialize- accepts any iterable of string-like pairs
brarchive::serialize([("entity.json", r#"{"id":"zombie"}"#)])?;
brarchive::serialize(&btree_map)?;

// Serialize with options (e.g. dedup identical content)
brarchive::serialize_with(&map, brarchive::SerializeOptions { dedup: true })?;
```

## CLI Usage

```shell
# Encode a directory into a single archive
brarchive-cli encode path/to/dir output.brarchive

# Encode recursively (mirrors directory tree under __brarchive/)
brarchive-cli encode path/to/pack --recursive

# Encode with content deduplication
brarchive-cli encode path/to/dir output.brarchive --dedup

# Delete source files after encoding
brarchive-cli encode path/to/dir output.brarchive --delete-source

# Decode a single archive
brarchive-cli decode output.brarchive path/to/out/

# Decode all archives under __brarchive/ recursively
brarchive-cli decode path/to/pack --recursive

# Delete archive after decoding
brarchive-cli decode output.brarchive path/to/out/ --delete-source

# List entry names in an archive
brarchive-cli list output.brarchive

# List entries in all archives under __brarchive/ recursively
brarchive-cli list path/to/pack --recursive
```
