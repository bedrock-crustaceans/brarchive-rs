use crate::args::{CliArgs, CliSubcommand};
use crate::logger::setup_logger;
use brarchive::SerializeOptions;
use clap::Parser;
use log::{error, info, warn};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Instant;

mod args;
mod logger;

fn main() {
    let args = CliArgs::parse();
    setup_logger(args.log_path);

    match args.command {
        CliSubcommand::Encode {
            path,
            out,
            recursive,
            dedup,
            delete_source,
        } => {
            let start_time = Instant::now();

            if recursive {
                let out_base = out.unwrap_or_else(|| path.clone());
                let archive_root = out_base.join("__brarchive");
                encode_recursive(&path, &path, &archive_root, dedup, delete_source);
                info!(
                    "Successfully encoded recursively in {}!",
                    humantime::format_duration(start_time.elapsed())
                );
            } else {
                let out = out.unwrap_or_else(|| {
                    extract_file_name(&path).unwrap_or(PathBuf::from("brarchive"))
                });
                let out = add_extension_if_missing(out, "brarchive");
                encode_single(&path, &out, dedup, delete_source);
                info!(
                    "Successfully encoded archive in {}!",
                    humantime::format_duration(start_time.elapsed())
                );
            }
        }
        CliSubcommand::List { path, recursive } => {
            if recursive {
                let archive_root = path.join("__brarchive");
                if !archive_root.exists() {
                    error!("No __brarchive/ directory found in \"{}\"", path.display());
                    exit(1);
                }
                list_recursive(&archive_root, &archive_root);
            } else {
                list_single(&path);
            }
        }
        CliSubcommand::Decode {
            path,
            out,
            recursive,
            delete_source,
        } => {
            let start_time = Instant::now();

            if recursive {
                let archive_root = path.join("__brarchive");
                if !archive_root.exists() {
                    error!("No __brarchive/ directory found in \"{}\"", path.display());
                    exit(1);
                }
                let out_base = out.unwrap_or_else(|| path.clone());
                decode_recursive(&archive_root, &archive_root, &out_base, delete_source);
                info!(
                    "Successfully decoded recursively in {}!",
                    humantime::format_duration(start_time.elapsed())
                );
            } else {
                let out = out.unwrap_or_else(|| {
                    extract_file_name(&path).unwrap_or(PathBuf::from("brarchive"))
                });
                decode_single(&path, &out, delete_source);
                info!(
                    "Successfully decoded archive in {}!",
                    humantime::format_duration(start_time.elapsed())
                );
            }
        }
    }
}

fn encode_single(path: &Path, out: &Path, dedup: bool, delete_source: bool) {
    if !path.exists() {
        error!("Input \"{}\" does not exist", path.display());
        exit(1);
    }
    if out.exists() {
        error!("Output \"{}\" already exists", out.display());
        exit(1);
    }

    let entries_map: BTreeMap<String, Vec<u8>> = if path.is_dir() {
        let read_dir = fs::read_dir(path).unwrap_or_else(|err| {
            error!("Failed to read directory \"{}\": {}", path.display(), err);
            exit(1);
        });
        let mut map = BTreeMap::new();
        for entry in read_dir {
            let entry = entry.unwrap_or_else(|err| {
                error!("{}", err);
                exit(1);
            });
            if !entry.path().is_file() {
                continue;
            }
            let content = fs::read(entry.path()).unwrap_or_else(|err| {
                error!("Failed to read \"{}\": {}", entry.path().display(), err);
                exit(1);
            });
            let name = entry
                .path()
                .strip_prefix(path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            map.insert(name, content);
        }
        map
    } else {
        let content = fs::read(path).unwrap_or_else(|err| {
            error!("Failed to read \"{}\": {}", path.display(), err);
            exit(1);
        });
        let name = path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string();
        BTreeMap::from([(name, content)])
    };

    let archive = brarchive::serialize_with(entries_map, SerializeOptions { dedup })
        .unwrap_or_else(|err| {
            error!("Failed to encode: {}", err);
            exit(1);
        });

    fs::write(out, &archive).unwrap_or_else(|err| {
        error!("Failed to write \"{}\": {}", out.display(), err);
        exit(1);
    });

    if delete_source {
        if path.is_dir() {
            fs::remove_dir_all(path).unwrap_or_else(|err| {
                error!("Failed to delete source \"{}\": {}", path.display(), err);
            });
        } else {
            fs::remove_file(path).unwrap_or_else(|err| {
                error!("Failed to delete source \"{}\": {}", path.display(), err);
            });
        }
    }
}

fn encode_recursive(
    source_root: &Path,
    current: &Path,
    archive_root: &Path,
    dedup: bool,
    delete_source: bool,
) {
    let read_dir = fs::read_dir(current).unwrap_or_else(|err| {
        error!("Failed to read \"{}\": {}", current.display(), err);
        exit(1);
    });

    let mut subdirs = Vec::new();
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();

    for entry in read_dir {
        let entry = entry.unwrap_or_else(|err| {
            error!("{}", err);
            exit(1);
        });
        let p = entry.path();
        if p.is_dir() {
            if p.file_name().and_then(OsStr::to_str) != Some("__brarchive") {
                subdirs.push(p);
            }
        } else if p.is_file() {
            let content = match fs::read(&p) {
                Ok(c) => c,
                Err(err) => {
                    warn!("Skipping unreadable file \"{}\": {}", p.display(), err);
                    continue;
                }
            };
            let name = p.file_name().and_then(OsStr::to_str).unwrap().to_string();
            files.insert(name, content);
        }
    }

    if !files.is_empty() {
        let relative = current.strip_prefix(source_root).unwrap_or(Path::new(""));
        let archive_path = if relative == Path::new("") {
            // Root-level files: name archive after the source directory itself
            let stem = source_root
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or("root");
            archive_root.join(stem).with_extension("brarchive")
        } else {
            add_extension_if_missing(archive_root.join(relative), "brarchive")
        };

        if let Some(parent) = archive_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|err| {
                error!("Failed to create directory: {}", err);
                exit(1);
            });
        }

        let archive =
            brarchive::serialize_with(&files, SerializeOptions { dedup }).unwrap_or_else(|err| {
                error!("Failed to encode: {}", err);
                exit(1);
            });

        fs::write(&archive_path, &archive).unwrap_or_else(|err| {
            error!("Failed to write \"{}\": {}", archive_path.display(), err);
            exit(1);
        });

        info!("Encoded \"{}\"", archive_path.display());

        if delete_source {
            for name in files.keys() {
                let file_path = current.join(name);
                fs::remove_file(&file_path).unwrap_or_else(|err| {
                    error!("Failed to delete \"{}\": {}", file_path.display(), err);
                });
            }
        }
    }

    for subdir in subdirs {
        encode_recursive(source_root, &subdir, archive_root, dedup, delete_source);
    }
}

fn decode_single(path: &Path, out: &Path, delete_source: bool) {
    if !path.exists() {
        error!("Input \"{}\" does not exist", path.display());
        exit(1);
    }

    let data = fs::read(path).unwrap_or_else(|err| {
        error!("Failed to read \"{}\": {}", path.display(), err);
        exit(1);
    });

    let archive: BTreeMap<String, Vec<u8>> = brarchive::deserialize(&data).unwrap_or_else(|err| {
        error!("Failed to decode \"{}\": {}", path.display(), err);
        exit(1);
    });

    if out.exists() && out.is_dir() {
        if fs::read_dir(out)
            .map(|mut d| d.next().is_some())
            .unwrap_or(false)
        {
            error!("Output directory \"{}\" is not empty", out.display());
            exit(1);
        }
    } else if !out.exists() {
        fs::create_dir_all(out).unwrap_or_else(|err| {
            error!("Failed to create output directory: {}", err);
            exit(1);
        });
    }

    for (file, contents) in archive {
        let dest = out.join(&file);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|err| {
                error!("Failed to create directories: {}", err);
                exit(1);
            });
        }
        fs::write(&dest, contents).unwrap_or_else(|err| {
            error!("Failed to write \"{}\": {}", dest.display(), err);
        });
        info!("Decoded {:?}", file);
    }

    if delete_source {
        fs::remove_file(path).unwrap_or_else(|err| {
            error!("Failed to delete source \"{}\": {}", path.display(), err);
        });
    }
}

fn decode_recursive(archive_root: &Path, current: &Path, out_root: &Path, delete_source: bool) {
    let read_dir = fs::read_dir(current).unwrap_or_else(|err| {
        error!("Failed to read \"{}\": {}", current.display(), err);
        exit(1);
    });

    for entry in read_dir {
        let entry = entry.unwrap_or_else(|err| {
            error!("{}", err);
            exit(1);
        });
        let p = entry.path();

        if p.is_dir() {
            decode_recursive(archive_root, &p, out_root, delete_source);
        } else if p.is_file() && p.extension().and_then(OsStr::to_str) == Some("brarchive") {
            let relative = p.strip_prefix(archive_root).unwrap_or(&p);
            let out_dir = out_root.join(relative.with_extension(""));
            if out_dir.starts_with(archive_root) {
                error!(
                    "Skipping \"{}\": output path \"{}\" would collide with archive root",
                    p.display(),
                    out_dir.display()
                );
                continue;
            }
            decode_single(&p, &out_dir, delete_source);
        }
    }
}

fn list_single(path: &Path) {
    let data = fs::read(path).unwrap_or_else(|err| {
        error!("Failed to read \"{}\": {}", path.display(), err);
        exit(1);
    });
    let names = brarchive::list(&data).unwrap_or_else(|err| {
        error!("Failed to list \"{}\": {}", path.display(), err);
        exit(1);
    });
    for name in names {
        println!("{}", name);
    }
}

fn list_recursive(archive_root: &Path, current: &Path) {
    let read_dir = fs::read_dir(current).unwrap_or_else(|err| {
        error!("Failed to read \"{}\": {}", current.display(), err);
        exit(1);
    });
    for entry in read_dir {
        let entry = entry.unwrap_or_else(|err| {
            error!("{}", err);
            exit(1);
        });
        let p = entry.path();
        if p.is_dir() {
            list_recursive(archive_root, &p);
        } else if p.is_file() && p.extension().and_then(OsStr::to_str) == Some("brarchive") {
            let relative = p.strip_prefix(archive_root).unwrap_or(&p);
            println!("{}:", relative.display());
            list_single(&p);
        }
    }
}

fn extract_file_name(path: &Path) -> Option<PathBuf> {
    path.file_stem().and_then(OsStr::to_str).map(PathBuf::from)
}

fn add_extension_if_missing(mut path: PathBuf, extension: &str) -> PathBuf {
    if path.extension().is_none() {
        path.set_extension(extension);
    }
    path
}
