# brarchive-rs

[![Crates.io Version](https://img.shields.io/crates/v/brarchive)](https://crates.io/crates/brarchive)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/brarchive)](https://crates.io/crates/brarchive)
[![Crates.io License](https://img.shields.io/crates/l/brarchive)](https://github.com/theaddonn/brarchive/blob/main/LICENSE)

Library for Bedrock Archives in Rust

So Mojang decided we don't have enough archive formats already and now invented their own for some reason, the `.brarchive` format.

It is basically nothing more than a simple uncompressed text archive format to bundle multiple files into one.
Which helps improve compression, and hopefully in the future allows for chunk based deduplication

This library implements the format and includes a CLI to encode and decode directories/archives.

## Library Usage Examples

One can easily decode/deserialize archives using the library:

```rust
fn main() {
    let bytes = include_bytes!("your_archive.brarchive");
    
    let archive = brarchive::deserialize(&bytes).unwrap();

    println!("{:#?}", archive);
}
```

One can also easily encode/serialize archives using the library:

```rust
use std::collections::BTreeMap;

fn main() {
    // You can use any data-structure such as HashMap, BTreeMap, Vec
    // Anything that implements IntoIterator<Item = (String, String)>
    let archive = BTreeMap::from([
        ("entry_name".to_string(), "entry_content".to_string())
    ]);

    let bytes = brarchive::serialize(&archive).unwrap();

    println!("{:?}", bytes);
}
```

## CLI Usage Examples

The integrated [brarchive-cli](https://crates.io/crates/brarchive-cli) allows you
to easily encode and decode archives with the command line. 

How to encode a folder:

```shell
brarchive-cli encode "path/to/input/folder" "path/to/output/file"
```

How to decode an archive:

```shell
brarchive-cli decode "path/to/input/file" "path/to/output/folder
```

You can also get help via the help command:

```shell
brarchive-cli help
```
