use crate::error::BrArchiveError;
use crate::versions::v1;
use crate::versions::{ENTRY_NAME_LEN_MAX, MAGIC, VERSIONS};
use byteorder::{LittleEndian, WriteBytesExt};
use std::collections::BTreeMap;
use std::io::{Cursor, Write};

pub mod error;
pub(crate) mod versions;

pub fn serialize(
    data: impl IntoIterator<Item = (String, String)>,
) -> Result<Vec<u8>, BrArchiveError> {
    let data = data.into_iter().collect::<Vec<_>>();
    let mut buf = Vec::new();
    buf.write_u64::<LittleEndian>(MAGIC)?;
    buf.write_u32::<LittleEndian>(data.len() as u32)?;
    buf.write_u32::<LittleEndian>(*VERSIONS.last().unwrap())?;
    let mut current_offset: u32 = 0;
    let mut entries: Vec<(u32, u32)> = Vec::with_capacity(data.len());
    for (_, content) in &data {
        let len = content.as_bytes().len() as u32;
        entries.push((current_offset, len));
        current_offset += len;
    }
    for ((name, _), (offset, len)) in data.iter().zip(entries.iter()) {
        let name_bytes = name.as_bytes();
        if name_bytes.len() > ENTRY_NAME_LEN_MAX {
            return Err(BrArchiveError::EntryNameTooLong(name_bytes.len()));
        }
        buf.write_u8(name_bytes.len() as u8)?;
        let mut name_buf = [0u8; ENTRY_NAME_LEN_MAX];
        name_buf[..name_bytes.len()].copy_from_slice(name_bytes);
        buf.write_all(&name_buf)?;
        buf.write_u32::<LittleEndian>(*offset)?;
        buf.write_u32::<LittleEndian>(*len)?;
    }
    for (_, content) in &data {
        buf.write_all(content.as_bytes())?;
    }
    Ok(buf)
}

pub fn deserialize(data: &[u8]) -> Result<BTreeMap<String, String>, BrArchiveError> {
    let mut buf = Cursor::new(data);
    let header = v1::read_header(&mut buf)?;
    let mut entry_descriptors = Vec::with_capacity(header.entries as usize);
    for _ in 0..header.entries {
        entry_descriptors.push(v1::read_entry_descriptor(&mut buf)?);
    }
    let mut map = BTreeMap::new();
    for entry in entry_descriptors {
        let contents = v1::read_entry_contents(&mut buf, &entry)?;
        map.insert(entry.name.to_string(), contents);
    }
    Ok(map)
}
