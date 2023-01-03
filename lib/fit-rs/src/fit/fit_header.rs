//! FIT file level header.

use std::io::Cursor;

use binread::{BinRead, BinReaderExt};

use crate::errors::FitError;

/// FIT file level header.
#[derive(Debug, Copy, Clone, Default, BinRead)]
pub struct FitHeader {
    /// Byte 0: size of header
    pub headersize: u8,
    /// Byte 1: Protocol
    pub protocol: u8,
    /// Bytes 2-3: Profile, Little Endian
    pub profile: u16,
    /// Bytes 4-7: Size of FIT data succeeding header, Little Endian
    pub datasize: u32,
    /// Bytes 8-11: Ascii for .FIT
    pub dotfit: [char; 4],
    /// Bytes 12, 13: CRC, optional. CRC check not implemented.
    #[br(default)]
    pub crc: Option<u16>,
}

impl FitHeader {
    /// Read FIT header with optional CRC value.
    /// Valid sizes are 12 or 14 bytes.
    pub fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, FitError> {
        let mut hdr: FitHeader = cursor.read_ne()?;

        if hdr.headersize == 14 {
            hdr.crc = Some(cursor.read_ne::<u16>()?);
        }

        Ok(hdr)
    }

    /// Derive data size, including for corrupt or truncated FIT-files.
    /// `len` is size of FitFile in bytes.
    /// Returns `FitHeader.datasize` if no issues exist.
    /// Encountered issues so far are:
    /// - FIT-files with reported data size 0, despite logged data
    /// - FIT-files with reported data size that exceeds file size
    pub fn data_size(&self, len: usize) -> usize {
        if self.datasize == 0 || self.datasize as usize > len {
            let crc_len = if self.crc.is_some() { 2 } else { 0 };
            let size = len - self.headersize as usize - crc_len;
            size
        } else {
            self.datasize as usize
        }
    }
}
