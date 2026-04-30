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
    #[command(alias = "compress", about = "Encode a given folder or single file into the bedrock-archive format")]
    Encode {
        path: PathBuf,
        out: Option<PathBuf>,
    },
    #[command(alias = "decompress", about = "Decode a given file from the bedrock-archive format into a folder or single file")]
    Decode {
        path: PathBuf,
        out: Option<PathBuf>,
    },
}
