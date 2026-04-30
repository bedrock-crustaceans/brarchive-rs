use std::str::Utf8Error;
use crate::{MAGIC, ENTRY_NAME_LEN_MAX};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrArchiveError {
    #[error("Magic Sequence Mismatch (expected {MAGIC}, got {0})")]
    MagicMismatch(u64),
    #[error("Unsupported Archive Version {0}")]
    UnsupportedVersion(u32),
    #[error("Entry Name too long! Got {0} bytes, maximum {ENTRY_NAME_LEN_MAX}")]
    EntryNameTooLong(usize),
    #[error(transparent)]
    Utf8Error(#[from] Utf8Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl PartialEq for BrArchiveError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MagicMismatch(a), Self::MagicMismatch(b)) => a == b,
            (Self::UnsupportedVersion(a), Self::UnsupportedVersion(b)) => a == b,
            (Self::Utf8Error(a), Self::Utf8Error(b)) => a == b,
            (Self::EntryNameTooLong(a), Self::EntryNameTooLong(b)) => a == b,
            (Self::IOError(_), Self::IOError(_)) => false,
            _ => false,
        }
    }
}
