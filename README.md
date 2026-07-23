# brarchive-rs

[![Crates.io Version](https://img.shields.io/crates/v/brarchive)](https://crates.io/crates/brarchive)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/brarchive)](https://crates.io/crates/brarchive)
[![Crates.io MSRV (version)](https://img.shields.io/crates/msrv/brarchive/0.3.0)](https://crates.io/crates/brarchive)
[![Crates.io License](https://img.shields.io/crates/l/brarchive)](https://github.com/theaddonn/brarchive-rs/blob/main/LICENSE)

Library and CLI for the Bedrock Archive (`.brarchive`) format, the bundling format Mojang uses
to pack the files inside Minecraft Bedrock Edition resource and behavior packs. Entries are
usually JSON, but newer packs also embed compiled binary data, and brarchive-rs handles both.

See [FORMAT.md](FORMAT.md) for the binary format specification.

## Library Usage

```rust
// List entry names without reading any content
let names: Vec<String> = brarchive::list(&bytes)?;

// Deserialize into anything that implements FromIterator<(String, Vec<u8>)>.
// Content comes back as raw bytes, since an archive can hold binary entries.
let map: std::collections::BTreeMap<String, Vec<u8>> = brarchive::deserialize(&bytes)?;
let vec: Vec<(String, Vec<u8>)>                       = brarchive::deserialize(&bytes)?;

// Values that hold text can be turned back into a String when you need one
let text = String::from_utf8(map["entity.json"].clone())?;

// Serialize any iterable of key/value pairs. Keys are entry names; values are
// bytes, so &str, String, Vec<u8>, and &[u8] all work.
brarchive::serialize([("entity.json", r#"{"id":"zombie"}"#)])?;
brarchive::serialize(&btree_map)?;

// Serialize with options, for example deduplicating identical content
brarchive::serialize_with(&map, brarchive::SerializeOptions { dedup: true })?;
```

## CLI Usage

Install the CLI from crates.io:

```shell
cargo install brarchive-cli
```

Prebuilt binaries for Linux, macOS, and Windows are attached to every
[GitHub release](https://github.com/theaddonn/brarchive-rs/releases) as well.

There are three subcommands: `encode`, `decode`, and `list`. Each one takes
either a single `.brarchive` file or, with `--recursive`, a whole pack whose
archives live under `__brarchive/`.

### Encoding

Bundle a folder into one archive:

```shell
brarchive-cli encode path/to/dir output.brarchive
```

Walk a pack and mirror its directory tree into `__brarchive/`, writing one
archive per folder:

```shell
brarchive-cli encode path/to/pack --recursive
```

Add `--dedup` to store identical file contents only once, and `--delete-source`
to remove the originals once the archive is written:

```shell
brarchive-cli encode path/to/dir output.brarchive --dedup --delete-source
```

### Decoding

Extract a single archive into a folder:

```shell
brarchive-cli decode output.brarchive path/to/out/
```

Extract every archive under a pack's `__brarchive/` folder in one go:

```shell
brarchive-cli decode path/to/pack --recursive
```

Mojang ships the JSON inside these archives minified, and by default the CLI
writes each entry back exactly as stored. Pass `--pretty` if you would rather
get readable JSON with a 2-space indent. Entries that are not JSON, such as the
compiled binary MCB files Bedrock now embeds, are always written untouched so
nothing gets corrupted:

```shell
brarchive-cli decode output.brarchive path/to/out/ --pretty
```

`--delete-source` works here too and removes the archive after a successful
decode.

### Listing

Print the entry names in an archive without extracting anything:

```shell
brarchive-cli list output.brarchive
```

Or list the contents of every archive in a pack:

```shell
brarchive-cli list path/to/pack --recursive
```
