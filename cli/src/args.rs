use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliSubcommand,
    #[arg(short, long)]
    pub log_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CliSubcommand {
    #[command(alias = "compress", about = "Encode a folder or file into .brarchive format")]
    Encode {
        path: PathBuf,
        out: Option<PathBuf>,
        /// Walk subdirectories, producing one .brarchive per directory under __brarchive/
        #[arg(short, long)]
        recursive: bool,
        /// Skip entries whose content is identical to an already-written entry
        #[arg(short, long)]
        dedup: bool,
        /// Delete source files after successful encode
        #[arg(long)]
        delete_source: bool,
    },
    #[command(alias = "decompress", about = "Decode a .brarchive file or directory of archives")]
    Decode {
        path: PathBuf,
        out: Option<PathBuf>,
        /// Given a directory, find and decode all .brarchive files within it
        #[arg(short, long)]
        recursive: bool,
        /// Delete source archive(s) after successful decode
        #[arg(long)]
        delete_source: bool,
    },
}
