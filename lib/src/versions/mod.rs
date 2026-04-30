pub(crate) mod v1;

pub(crate) const MAGIC: u64 = 0x267052A0B125277D;
pub(crate) const VERSIONS: [u32; 1] = [1];
pub(crate) const ENTRY_NAME_LEN_MAX: usize = 247;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Header {
    pub entries: u32,
    pub version: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct EntryDescriptor<'a> {
    pub name: &'a str,
    pub contents_offset: u32,
    pub contents_len: u32,
}
