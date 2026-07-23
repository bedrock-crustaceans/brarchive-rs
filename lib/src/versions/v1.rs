use crate::error::BrArchiveError;
use crate::versions::{EntryDescriptor, Header, ENTRY_NAME_LEN_MAX, MAGIC, VERSIONS};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek};

pub(crate) fn read_header(buf: &mut Cursor<&[u8]>) -> Result<Header, BrArchiveError> {
    let magic = buf.read_u64::<LittleEndian>()?;
    if magic != MAGIC {
        return Err(BrArchiveError::MagicMismatch(magic));
    }
    let entries = buf.read_u32::<LittleEndian>()?;
    let version = buf.read_u32::<LittleEndian>()?;
    if !VERSIONS.contains(&version) {
        return Err(BrArchiveError::UnsupportedVersion(version));
    }
    Ok(Header { entries, version })
}

pub(crate) fn read_entry_descriptor<'a>(
    buf: &mut Cursor<&'a [u8]>,
) -> Result<EntryDescriptor<'a>, BrArchiveError> {
    let name_len = buf.read_u8()?;
    if name_len as usize > ENTRY_NAME_LEN_MAX {
        return Err(BrArchiveError::EntryNameTooLong(name_len as usize));
    }
    let current_pos = buf.position() as usize;
    let name = std::str::from_utf8(&buf.get_ref()[current_pos..current_pos + name_len as usize])?;
    buf.set_position((current_pos + ENTRY_NAME_LEN_MAX) as u64);
    let contents_offset = buf.read_u32::<LittleEndian>()?;
    let contents_len = buf.read_u32::<LittleEndian>()?;
    Ok(EntryDescriptor {
        name,
        contents_offset,
        contents_len,
    })
}

pub(crate) fn read_entry_contents(
    buf: &mut Cursor<&[u8]>,
    entry: &EntryDescriptor,
) -> Result<Vec<u8>, BrArchiveError> {
    let start_offset = buf.stream_position()?;
    buf.set_position(start_offset + entry.contents_offset as u64);
    let mut contents = vec![0u8; entry.contents_len as usize];
    buf.read_exact(&mut contents)?;
    buf.set_position(start_offset);
    Ok(contents)
}
