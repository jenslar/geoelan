//! Core MP4 struct and methods.
//! 
//! Note on `hdlr` atom and finding "component name"
//! (this crate was developed with the need for parsing GoPro MP4 files, hence the examples below):
//! - The component name is a counted string:
//!     - first byte specifies number of bytes, e.g. "0x0b" = 11, followed by the string.
//!     - For e.g. GoPro the component name for GPMF data "GoPro MET": starts after 8 32-bit fields.
//!     - All GoPro component names end in 0x20 so far: ' ':
//!     - ASCII/Unicode U+0020 (category Zs: Separator, space), so just read as utf-8 read_to_string after counter byte and strip whitespace?
//! 
//! ```rs
//! use mp4iter::Mp4;
//! //! use std::path::Path;
//! 
//! fn main() -> std::io::Result<()> {
//!     let mp4 = Mp4::new(Path::new("VIDEO.MP4"))?;
//!     
//!     // Iterate over atoms. Currently returns `None` on error.
//!     for atom in mp4.into_iter() {
//!         println!("{atom:?}")
//!     }
//!
//!     println!("{:?}", mp4.duration());
//! 
//!     Ok(())
//! }
//! ```

use std::{
    io::{SeekFrom, Cursor, Read, Seek},
    fs::{Metadata, File},
    path::Path, borrow::BorrowMut
};

use binread::{
    BinReaderExt,
    BinResult
};
use time::ext::NumericalDuration;

use crate::{
    errors::Mp4Error,
    atom::Atom,
    fourcc::FourCC,
    Offset,
    Stts,
    Stsz,
    Stco,
    Hdlr,
    Udta, AtomHeader, CONTAINER
};

/// Mp4 file.
pub struct Mp4{
    /// Open MP4 file.
    file: File,
    /// Current byte offset, while parsing.
    /// Must always be the start of an atom.
    pub offset: u64, // is File::seek()/self.seek() enough?
    /// Offset for last branch/container atom
    // branch_offset: Option<u64>,
    pub len: u64
}

impl Iterator for Mp4 {
    type Item = AtomHeader;

    fn next(&mut self) -> Option<Self::Item> {
        let atom_header = self.header().ok()?;
        let _ = self.next().ok()?;

        Some(atom_header)
    }
}

impl Mp4 {
    /// New Mp4 from path. Sets offset to 0.
    /// Offset changes while parsing and must always
    /// be the start of an atom.
    pub fn new(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let len = file.metadata()?.len(); // save as constant once instead of repeated sys calls
        Ok(Self{
            file,
            offset: 0,
            // branch_offset: None,
            len
        })
    }

    /// Returns file size in bytes.
    pub fn len(&self) -> std::io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    /// Reads specified number of bytes at current position,
    /// and returns these as `Cursor<Vec<u8>>`.
    pub fn read(&mut self, len: u64) -> Result<Cursor<Vec<u8>>, Mp4Error> {
        let mut chunk = self.file.borrow_mut().take(len);
        let mut data = Vec::with_capacity(len as usize);
        let read_len = chunk.read_to_end(&mut data)? as u64;

        if read_len != len {
            return Err(Mp4Error::ReadMismatch{got: read_len, expected: len})
        } else {
            Ok(Cursor::new(data))
        }
    }

    /// Reads `len` number of bytes
    /// at specified position/byte offset `pos` from start of MP4,
    /// and returns these as `Cursor<vec<u8>>`.
    pub fn read_at(&mut self, pos: u64, len: u64) -> Result<Cursor<Vec<u8>>, Mp4Error> {
        // TODO 221016 bounds check not working as expected.
        // TODO        bounds error for max360/fusion if used (but not max-heromode),
        // TODO        but parses fine if commented out...
        // TODO        is self.len/size incorrectly set/used?
        // TODO        no error for hero-series if self.check_bounds is used
        // TODO        something in multi-device (Fusion/Max) gpmf structure that causes this?
        // self.check_bounds(len)?;
        self.seek_to(pos)?;
        self.read(len)
    }

    /// Returns current position/offset.
    pub fn pos(&mut self) -> std::io::Result<u64> {
        self.file.seek(SeekFrom::Current(0))
    }

    /// Seeks back or forth relative to current position.
    pub fn seek(&mut self, offset_from_current: i64) -> Result<(), Mp4Error> {
        // self.file.seek(SeekFrom::Current(offset_from_current))
        let pos_seek = self.file.seek(SeekFrom::Current(offset_from_current))?;
        let pos = self.pos()?;
        if pos_seek != pos {
            return Err(Mp4Error::OffsetMismatch{got: pos_seek as u64, expected: pos})
        }
        Ok(())
    }

    /// Seeks from start.
    pub fn seek_to(&mut self, offset_from_start: u64) -> Result<(), Mp4Error> {
        let pos_seek = self.file.seek(SeekFrom::Start(offset_from_start))?;
        let pos = self.pos()?;
        if pos_seek != pos {
            self.offset = offset_from_start;
            return Err(Mp4Error::OffsetMismatch{got: pos_seek as u64, expected: pos})
        }
        Ok(())
    }

    /// Convenience method to check whether atom at current offset is
    /// a container or not.
    pub fn is_container(&mut self) -> Result<bool, Mp4Error> {
        let name = self.name()?;
        Ok(CONTAINER.contains(&name.as_str()))
    }

    /// Check if len to read exceeds file size.
    pub fn check_bounds(&self, len_to_try: u64) -> Result<(), Mp4Error> {
        // TODO 221016 bounds check not working as expected.
        // TODO        bounds error for max360/fusion if used (but not max-heromode),
        // TODO        but parses fine if commented out...
        // TODO        is self.len/size incorrectly set/used?
        // TODO        no error for hero-series if self.check_bounds is used
        // TODO        something in multi-device (Fusion/Max) gpmf structure that causes this?
        let len = self.offset + len_to_try;
        if len > self.len {
            Err(Mp4Error::BoundsError((len, self.offset)))
        } else {
            Ok(())
        }
    }

    /// Derive offset for next top-level atom,
    /// starting from current offset.
    /// Sets `Mp4.offset` to derived offset,
    /// and returns new offset.
    /// 
    /// Note that `next()` currently does not find nested atoms
    /// that do not begin immediately after the header.
    /// Try e.g. [AtomicParsley](https://atomicparsley.sourceforge.net) for this.
    pub fn next(&mut self) -> Result<u64, Mp4Error> {
        let size = self.size()?; // byte 0-3 for each atom

        // TODO need to be able to backtrack if checking container/size when branching
        let iter_size = if self.is_container()? {
            // Go past 8 bytes header of container
            8
        } else {
            size
        };

        self.offset += iter_size;
        // self.seek(size as i64)?;

        Ok(self.offset)
    }

    /// Returns total size of atom at current offset.
    /// Supports 64-bit sizes.
    pub fn size(&mut self) -> Result<u64, Mp4Error> {
        // self.check_bounds(4)?;
        let mut cursor = self.read_at(self.offset, 4)?;
        
        // let mut cursor = self.read(4)?;
        // Get container size, 32 or 64-bit value
        match cursor.read_be::<u32>() {
            // Check against, e.g. 1k Dropbox place holders containing zeros.
            Ok(0) => Err(Mp4Error::UnexpectedAtomSize{len: 0, offset: self.pos()?}),

            // Size of 1 indicates a 64-bit sized atom,
            // actual size retreived from the following 8 bytes
            Ok(1) => self.read(8)?
                .read_be::<u64>()
                .map_err(|e| Mp4Error::BinReadError(e)),

            Ok(l) => Ok(l as u64),

            Err(err) => Err(Mp4Error::BinReadError(err))
        }
    }

    /// Get name of atom (Four CC) at current offset.
    /// Supports Four CC with byte values above 127 (non-standard ASCII)
    /// if the numerical values map to ISO8859-1,
    /// e.g. GoPro uses Four CC `©xyz` in `udta` atom.
    pub fn name(&mut self) -> Result<String, Mp4Error> {
        let mut cursor = self.read_at(self.offset + 4, 4)?;
        let bytes: Vec<u8> = (0..4).into_iter()
            .map(|_| cursor.read_be::<u8>())
            .collect::<BinResult<Vec<u8>>>()?;
        let name: String = bytes.iter()
            .map(|b| *b as char)
            .collect();
        Ok(name)
    }

    /// Return atom header at current offset.
    pub fn header(&mut self) -> Result<AtomHeader, Mp4Error> {
        Ok(AtomHeader {
            size: self.size()?,
            name: FourCC::from_str(&self.name()?),
            offset: self.offset,
        })
    }

    /// Read data for atom at current offset as a single chunk.
    /// Note that e.g. the `mdat` atom may be many GB in size,
    /// and that raw data is read into memory as `Cursor<Vecu8>>`.
    pub fn atom(&mut self) -> Result<Atom, Mp4Error> {
        let header = self.header()?;
        let cursor = self.read_at(self.offset + 8, header.size - 8)?;

        Ok(Atom{
            header,
            cursor
        })
    }

    /// Finds first top-level atom with specified name (FourCC),
    /// then returns and sets `Mp4.offset` to start of that atom.
    /// 
    /// `Mp4::find()` will continue from current offset.
    /// Run `Mp4::reset()` to set start offset to 0.
    pub fn find(&mut self, name: &str) -> Result<Option<u64>, Mp4Error> {
        // TODO does not check self at current offset, only from next and on...
        while let Ok(offset) = self.next() {
            if self.name()? == name {
                // TODO is this correct? self.name moves offset 4 bytes...?
                return Ok(Some(offset))
            }
        }
        Ok(None)
    }

    /// Exctract time to sample values (`stts` atom - one for each `trak`).
    /// 
    /// Path: `/ moov / trak (multiple) / mdia / minf / stbl / stts`
    pub fn stts(&mut self) -> Result<Stts, Mp4Error> {
        if self.find("stts").is_ok() {
            self.atom()?.stts()
        } else {
            Err(Mp4Error::NoSuchAtom("stts".to_owned()))
        }
    }

    /// Exctract sample to size values (`stsz` atom - one for each `trak`).
    /// 
    /// Path: `/ moov / trak (multiple) / mdia / minf / stbl / stsz`
    pub fn stsz(&mut self) -> Result<Stsz, Mp4Error> {
        if self.find("stsz").is_ok() {
            self.atom()?.stsz()
        } else {
            Err(Mp4Error::NoSuchAtom("stsz".to_owned()))
        }
    }

    /// Exctract chunk offset values (`stco` atom - one for each `trak`).
    /// 
    /// Path: `/ moov / trak (multiple) / mdia / minf / stbl / stco`
    pub fn stco(&mut self) -> Result<Stco, Mp4Error> {
        if self.find("stco").is_ok() {
            self.atom()?.stco()
        } else {
            Err(Mp4Error::NoSuchAtom("stco".to_owned()))
        }
    }

    /// Exctract media handler values (`hdlr` atom).
    /// 
    /// Path: `/ moov / trak (multiple) / mdia / hdlr`
    pub fn hdlr(&mut self) -> Result<Hdlr, Mp4Error> {
        if self.find("hdlr").is_ok() {
            self.atom()?.hdlr()
        } else {
            Err(Mp4Error::NoSuchAtom("hdlr".to_owned()))
        }
    }

    /// Extract user data atom (`udta`).
    /// Some vendors embed data such as device info,
    /// unique identifiers (Garmin VIRB UUID),
    /// or even data in vendor specific formats
    /// (GoPro undocumented GPMF data, separate from
    /// the main GPMF telemetry interleaved in the `mdat` atom).
    /// 
    /// Path: `/ moov / udta`
    // pub fn udta(&mut self) -> Result<Atom, Mp4Error> {
    pub fn udta(&mut self) -> Result<Udta, Mp4Error> {
        // Set position to start of file to avoid
        // previous reads to have moved the cursor
        // past the 'udta' atom.
        self.reset()?;
        
        // if let Ok(_) = self.find("udta") {
        if self.find("udta").is_ok() {
            self.atom()?.udta()
        } else {
            Err(Mp4Error::NoSuchAtom("udta".to_owned()))
        }
    }

    /// Returns duration of MP4.
    /// Derived from `mvhd` atom (inside `moov` atom),
    /// which lists duration for whichever track is the longest.
    pub fn duration(&mut self) -> Result<time::Duration, Mp4Error> {
        // ensure search is done from beginning of file
        self.reset()?;
        // Find 'mvhd' atom (inside 'moov' atom)
        if let Some(offset) = self.find("mvhd")? {
            // seek to start of 'mvhd' + until 'time scale' field
            self.seek_to(offset + 20)?;
            
            // Read time scale value and scaled duration, normalises to seconds
            let time_scale = self.read(4)?.read_be::<u32>()?;
            let scaled_duration = self.read(4)?.read_be::<u32>()?;
            // Generate 'time::Duration' from normalised duration
            let duration = (scaled_duration as f64 / time_scale as f64).seconds();
    
            Ok(duration)
        } else {
            Err(Mp4Error::NoSuchAtom("mvhd".to_owned()))
        }
    }

    /// Seek to start of file and set `self.offset = 0`.
    pub fn reset(&mut self) -> Result<(), Mp4Error> {
        self.seek_to(0)?;
        self.offset = 0;
        Ok(())
    }

    /// Returns file `std::fs::Metadata`.
    pub fn meta(&self) -> std::io::Result<Metadata> {
        self.file.metadata()
    }

    /// Extract byte offsets, byte sizes, and time/duration
    /// for track handler with specified `handler_name` ('hdlr' atom inside 'moov/trak' atom). 
    /// E.g. use `handler_name` "GoPro MET" for locating interleaved GoPro GPMF data in MP4.
    pub fn offsets(&mut self, handler_name: &str) -> Result<Vec<Offset>, Mp4Error> {
        // self.reset()?;
        
        while let Ok(hdlr) = self.hdlr() {
            if &hdlr.component_name == handler_name {
                let stts = self.stts()?;
                let stsz = self.stsz()?;
                let stco = self.stco()?;

                // Assert equal size of all contained Vec:s to allow iter in parallel
                assert_eq!(stco.len(), stsz.len(), "'stco' and 'stsz' atoms differ in data size");
                assert_eq!(stts.len(), stsz.len(), "'stts' and 'stsz' atoms differ in data size");

                let offsets: Vec<Offset> = stts.iter()
                    .zip(stsz.iter())
                    .zip(stco.iter())
                    .map(|((duration, size), position)| Offset::new(*position, *size, *duration))
                    .collect();

                return Ok(offsets)
            }
        }

        Err(Mp4Error::NoOffsets)
    }
}