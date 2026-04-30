use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use crate::{EntryDescriptor, Header, ENTRY_NAME_LEN_MAX, MAGIC, VERSIONS};
use crate::error::BrArchiveError;

pub fn write_header(buf: &mut Vec<u8>, header: &Header) -> Result<(), BrArchiveError>  {
    buf.write_u64::<LittleEndian>(MAGIC)?;
    buf.write_u32::<LittleEndian>(header.entries)?;

    if !VERSIONS.contains(&header.version) {
        return Err(BrArchiveError::UnsupportedVersion(header.version));
    }

    buf.write_u32::<LittleEndian>(header.version)?;

    Ok(())
}

pub fn write_entry_descriptor(buf: &mut Vec<u8>, entry: &EntryDescriptor) -> Result<(), BrArchiveError>  {
    let name = entry.name.as_bytes();

    if name.len() > ENTRY_NAME_LEN_MAX {
        return Err(BrArchiveError::EntryNameTooLong(name.len()))
    }

    let mut name_buf =  [0; ENTRY_NAME_LEN_MAX];
    name_buf[..name.len()].copy_from_slice(name);

    buf.write_u8(name.len() as u8)?;
    buf.write_all(&name_buf)?;

    buf.write_u32::<LittleEndian>(entry.contents_offset)?;
    buf.write_u32::<LittleEndian>(entry.contents_len)?;

    Ok(())
}

pub fn write_entry_contents(buf: &mut Vec<u8>, entry_data: &str) -> Result<(), BrArchiveError>  {
    buf.write_all(entry_data.as_bytes())?;
    Ok(())
}
