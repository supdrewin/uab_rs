use modular_bitfield::prelude::*;

use crate::BinaryReader;

#[derive(Debug, Specifier)]
#[bits = 6]
pub enum CompressionType {
    None,
    Lzma,
    Lz4,
    Lz4HC,
}

#[bitfield]
#[repr(u32)]
#[derive(Debug)]
pub struct ArchiveFlags {
    compression_type: CompressionType,
    blocks_and_directory_info_combined: bool,
    blocks_info_at_the_end: bool,
    old_web_plugin_compatibility: bool,
    block_info_need_padding_at_start: bool,
    unity_cnencryption: bool,
    _reserved0: B1,
    unity_cnencryption_new: bool,
    _reserved1: B19,
}

#[derive(Debug)]
pub struct Header {
    signature: String,
    version: u32,
    unity_version: String,
    unity_revision: String,
    size: i64,
    compressed_blocks_info_size: u32,
    uncompressed_blocks_info_size: u32,
    archive_flags: ArchiveFlags,
}

#[derive(Debug)]
pub struct AssetBundle {
    header: Header,
}

impl AssetBundle {
    pub fn new(mut reader: BinaryReader) -> Self {
        Self {
            header: Header {
                signature: reader.read_cstr(),
                version: reader.read_u32(),
                unity_version: reader.read_cstr(),
                unity_revision: reader.read_cstr(),
                size: reader.read_i64(),
                compressed_blocks_info_size: reader.read_u32(),
                uncompressed_blocks_info_size: reader.read_u32(),
                archive_flags: reader.read_u32().into(),
            },
        }
    }
}
