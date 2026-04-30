use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use clap::Parser;
use log::{error, info};
use crate::args::{CliArgs, CliSubcommand};
use crate::logger::setup_logger;

mod args;
mod logger;

fn main() {
    let args = CliArgs::parse();

    setup_logger(args.log_path);

    match args.command {
        CliSubcommand::Encode { path, out } => {
            let start_time = Instant::now();

            let out = out.unwrap_or(extract_file_name(&path).unwrap_or(PathBuf::from("brarchive")));
            let out = add_extension_if_missing(out, "brarchive");

            info!("Beginning to encode \"{}\" into \"{}\" archive", &path.display(), out.display());

            let exists = fs::exists(&out).unwrap_or_else(|err| {
                error!("Failed to check if output directory exists \"{}\": {}", path.display(), err);
                exit(1);
            });

            if exists && !out.is_dir() {
                error!("Output directory \"{}\" already exists", out.display());
                exit(1);
            }

            if exists && !out.is_file() {
                error!("Output file \"{}\" already exists", out.display());
                exit(1);
            }

            let exists = fs::exists(&path).unwrap_or_else(|err| {
                error!("Failed to check if input directory exists \"{}\": {}", path.display(), err);
                exit(1);
            });

            if !exists {
                error!("Input directory \"{}\" does not exist", path.display());
                exit(1);
            }

            let entries_map = if path.is_dir() {
                let read_dir = fs::read_dir(&path).unwrap_or_else(|err| {
                    error!("Failed to read directory \"{}\": {}", path.display(), &err);
                    exit(1);
                });

                let mut dir_entries = BTreeMap::new();

                for entry in read_dir {
                    let entry = entry.unwrap_or_else(|err| {
                        error!("Failed to read directory entry of \"{}\": {}", path.display(), err);
                        exit(1);
                    });

                    let content = fs::read_to_string(entry.path()).unwrap_or_else(|err| {
                        error!("Failed to read file \"{}\": {}", entry.path().display(), err);
                        exit(1);
                    });

                    let entry_name = entry.path();
                    let entry_name = entry_name.strip_prefix(&path).unwrap_or_else(|err| {
                        error!("Failed to strip prefix from \"{}\": {}", entry.path().display(), err);
                        exit(1);
                    });
                    let entry_name = entry_name.to_str().unwrap_or_else(|| {
                        error!("Failed to convert file path to UTF-8: {}", entry.path().display());
                        exit(1);
                    });

                    dir_entries.insert(entry_name.to_string(), content);
                }

                dir_entries
            } else if path.is_file() {
                let content = fs::read_to_string(&path).unwrap_or_else(|err| {
                    error!("Failed to read file \"{}\": {}", path.display(), &err);
                    exit(1);
                });

                let entry_name = path.file_name().and_then(OsStr::to_str).unwrap_or_else(|| {
                    error!("Failed to convert file path to UTF-8: {}", path.display());
                    exit(1);
                }).to_string();

                BTreeMap::from([(entry_name, content)])
            } else {
                error!("Input Path is neither a file nor a folder \"{}\"", path.display());
                exit(1);
            };

            let archive = brarchive::serialize(entries_map).unwrap_or_else(|err| {
                error!("Failed to encode directory entries \"{}\": {}", err, path.display());
                exit(1);
            });

            fs::write(&out, &archive).unwrap_or_else(|err| {
                error!("Failed to write archive \"{}\": {}", out.display(), err);
            });

            info!("Successfully encoded archive in {}!", humantime::format_duration(start_time.elapsed()));
        }
        CliSubcommand::Decode { path, out } => {
            let start_time = Instant::now();

            let out = out.unwrap_or(extract_file_name(&path).unwrap_or(PathBuf::from("brarchive")));

            info!("Beginning to decode archive \"{}\" into \"{}\"", path.display(), out.display());

            let exists = fs::exists(&path).unwrap_or_else(|err| {
                error!("Failed to check if input file exists \"{}\": {}", path.display(), err);
                exit(1);
            });

            if !exists {
                error!("Input file \"{}\" does not exist", path.display());
                exit(1);
            }

            let data = fs::read(&path).unwrap_or_else(|err| {
                error!("Failed to read archive file \"{}\": {}", path.display(), err);
                exit(1);
            });

            let archive = brarchive::deserialize(&data).unwrap_or_else(|err| {
                error!("Failed to decode archive file \"{}\": {}", path.display(), err);
                exit(1);
            });

            let exists = fs::exists(&out).unwrap_or_else(|err| {
                error!("Failed to check if output directory exists \"{}\": {}", out.display(), err);
                exit(1);
            });

            if exists && !out.is_dir() {
                error!("Output directory \"{}\" already exists and is not a directory", out.display());
                exit(1);
            }

            let directory_contents = fs::read_dir(&out).ok();

            if let Some(contents) = directory_contents {
                if contents.count() > 0 || exists {
                    error!("Output directory \"{}\" already exists and is not empty", out.display());
                    exit(1);
                }
            }

            if !exists {
                fs::create_dir(&out).unwrap_or_else(|err| {
                    error!("Failed to create output directory \"{}\": {}", out.display(), err);
                    exit(1);
                });

                info!("Successfully created output directory \"{}\"", out.display());
            }

            for (file, contents) in archive {
                fs::write(out.join(&file), contents).unwrap_or_else(|err| {
                    error!("Failed to write output file \"{}\": {}", out.display(), err);
                });

                info!("Successfully decoded {:?}", &file);
            }

            info!("Successfully decoded archive in {}!", humantime::format_duration(start_time.elapsed()));
        }
    }
}


fn extract_file_name(path: &PathBuf) -> Option<PathBuf> {
    path.file_stem()
        .and_then(OsStr::to_str) // Convert OsStr to &str
        .map(PathBuf::from) // Convert &str to PathBuf
}

fn add_extension_if_missing(mut path: PathBuf, extension: &str) -> PathBuf {
    if path.extension().is_none() {
        path.set_extension(extension);
    }
    path
}
