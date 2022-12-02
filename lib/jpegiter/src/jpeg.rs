use std::{io::{SeekFrom, Cursor, Read, Seek}, fs::{Metadata, File}, path::{Path, PathBuf}, borrow::BorrowMut};

use binread::{BinReaderExt, BinResult, BinRead};

use crate::{JpegError, tag::JpegTag, Segment};

/// JPEG file.
pub struct Jpeg {
    // TODO probably better to just read as single Cursor<> instead of multiple file reads
    /// Path.
    path: PathBuf,
    /// File data.
    cursor: Cursor<Vec<u8>>,
}

impl Jpeg {
    /// Create new `Jpeg` struct from path.
    pub fn new(path: &Path) -> std::io::Result<Self> {
        Ok(Self{
            path: path.to_owned(),
            cursor: Cursor::new(std::fs::read(path)?),
        })
    }

    /// Returns file size.
    pub fn len(&self) -> std::io::Result<u64> {
        Ok(self.path.metadata()?.len())
    }

    /// Returns current position/offset.
    pub fn pos(&mut self) -> std::io::Result<u64> {
        self.cursor.seek(SeekFrom::Current(0))
    }

    /// Find next segment.
    pub fn next(&mut self) -> Result<Segment, JpegError> {
        let mut tag = JpegTag::from(self.read_as::<u16>()?);
        
        // Move past start of image
        if tag == JpegTag::SOI {
            tag = JpegTag::from(self.read_as::<u16>()?)
        // Return end of file is End of Image tag found (0xFFD9)
        } else if tag == JpegTag::EOI {
            return Ok(Segment::eof())
        }

        // Size of segment includes the size value itself (2 bytes), but not the tag (2 bytes).
        let size = self.read_as::<u16>()? - 2;
        // Return data slice as Cursor<Vec<u8>>
        let data = self.read(size as u64)?;

        Ok(Segment{
            tag,
            data
        })
    }

    /// Seeks back or forth relative to current position.
    pub fn seek(&mut self, offset_from_current: i64) -> Result<u64, JpegError> {
        let pos_seek = self.cursor.seek(SeekFrom::Current(offset_from_current))?;
        let pos = self.pos()?;
        if pos_seek != pos {
            return Err(JpegError::OffsetMismatch{got: pos_seek as u64, expected: pos})
        }
        Ok(pos_seek)
    }
    
    /// Seeks from start.
    pub fn seek_to(&mut self, offset_from_start: u64) -> Result<u64, JpegError> {
        let pos_seek = self.cursor.seek(SeekFrom::Start(offset_from_start))?;
        let pos = self.pos()?;
        if pos_seek != pos {
            return Err(JpegError::OffsetMismatch{got: pos_seek as u64, expected: pos})
        }
        Ok(pos_seek)
    }

    /// Reads specified number of bytes at current position,
    /// and returns these as `Cursor<Vec<u8>>`.
    pub fn read(&mut self, len: u64) -> Result<Cursor<Vec<u8>>, JpegError> {
        let mut chunk = self.cursor.borrow_mut().take(len);
        let mut data = Vec::with_capacity(len as usize);
        let read_len = chunk.read_to_end(&mut data)? as u64;

        if read_len != len {
            Err(JpegError::ReadMismatch{got: read_len, expected: len})
        } else {
            Ok(Cursor::new(data))
        }
    }

    /// Reads `len` number of bytes
    /// at specified position/byte offset `pos` from start of MP4,
    /// and returns these as `Cursor<vec<u8>>`.
    pub fn read_at(&mut self, pos: u64, len: u64) -> Result<Cursor<Vec<u8>>, JpegError> {
        // TODO bounds check?
        self.seek_to(pos)?;
        self.read(len)
    }

    pub fn read_as<T: Sized + BinRead>(&mut self) -> Result<T, JpegError> {
        self.cursor.read_be::<T>().map_err(|err| err.into())
    }

    /// Returns total size of segment at current offset.
    pub fn size(&mut self) -> Result<u16, JpegError> {
        self.cursor.read_be::<u16>().map_err(|err| err.into())
    }

    pub fn find(
        &mut self,
        tag: &JpegTag
    ) -> Result<Option<Segment>, JpegError> {
        while let Ok(segment) = self.next() {
            if &segment.tag == tag {
                return Ok(Some(segment))
            }
        }
        Ok(None)
    }

    pub fn exif(&mut self) -> Result<Option<Segment>, JpegError> {
        self.find(&JpegTag::APP1)
    } 
}