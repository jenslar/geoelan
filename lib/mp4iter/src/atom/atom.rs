use std::io::{Cursor, Read, Seek, SeekFrom};

use binread::{BinReaderExt, BinRead, BinResult};

use crate::{errors::Mp4Error, fourcc::FourCC};

use super::{Hdlr, Stco, Stsz, Stts, Udta, UdtaField};


// /// Values extracted from GoPro MP4 to determine offset, size, duration for each DEVC chunk:
// /// - `position` in bytes, from `stco` atom
// /// - `size` in bytes, from `stsz` atom
// /// - `duration` in time units, from `stts` atom, milliseconds so far (needs to be verified)
// #[derive(Debug, Clone)]
// pub struct Offset {
//     /// Byte offset derived from `stco` atom
//     /// under GoPro MET
//     pub position: u32,
//     /// Byte size derived from `stsz` atom
//     /// under GoPro MET
//     pub size: u32,
//     /// Sample duration in milliseconds (?)
//     /// derived from `stts` atom under GoPro MET
//     pub duration: u32, // from stts atom in GoPro MET
// }

// // If the atom is a "container",
// // it's nested and contains more atoms,
// // within its specified, total size.
// const CONTAINERS: [&'static str; 10] = [
//     "moov", // offset tables, timing, metadata, telemetry
//     "trak", // moov -> trak
//     "tref", // moov -> trak -> tref
//     "edts", // moov -> trak -> edts
//     "mdia", // moov -> trak -> mdia
//     "minf", // moov -> trak -> mdia -> minf
//     "dinf", // moov -> trak -> mdia -> minf -> dinf
//     "dref", // moov -> trak -> mdia -> minf -> dinf -> dref
//     "stbl", // moov -> trak -> mdia -> minf -> stbl, contains timing (stts), offsets (stco)
//     "stsd", // moov -> trak -> mdia -> minf -> stbl -> stsd
// ];

/// MP4 atom.
pub struct Atom {
    /// Total size in bytes including 8 byte "header"
    /// (4 bytes size, 4 bytes Four CC)
    pub size: u64,
    /// Four CC
    pub name: FourCC,
    /// Byte offset in MP4.
    pub offset: u64,
    /// Raw data load, excluding 8 byte header (size + name).
    pub cursor: Cursor<Vec<u8>>
}

impl Atom {
    // pub fn new(cursor: &mut Cursor<Vec<u8>>) {

    // }

    pub fn iter(&mut self) {}

    /// Read single Big Endian value.
    pub fn read<T: Sized + BinRead>(&mut self) -> BinResult<T> {
        self.cursor.read_be::<T>()
    }

    /// Read multiple Big Endian values of the same primal type.
    pub fn iter_read<T: Sized + BinRead>(&mut self, repeats: usize) -> BinResult<Vec<T>> {
        (0..repeats).into_iter()
            .map(|_| self.read::<T>())
            .collect()
    }

    /// Read cursor to string.
    pub fn read_to_string(&mut self) -> std::io::Result<String> {
        // get the byte len (NOT UTF-8 len).
        let len = self.cursor.get_ref().len();
        let mut string = String::with_capacity(len);
        self.cursor.read_to_string(&mut string)?;
        Ok(string)
    }

    /// Seek to next atom if nested.
    pub fn next(&mut self) -> Result<u64, Mp4Error> {
        let size = self.read::<u32>()?;
        self.cursor.seek(SeekFrom::Current(size as i64 - 4))
            .map_err(|e| Mp4Error::IOError(e))
    }

    pub fn pos(&self) -> u64 {
        self.cursor.position()
    }

    /// Set atom cursor position to start of cursor.
    pub fn reset(&mut self) {
        self.cursor.set_position(0)
    }

    /// Get name of atom (Four CC) at current offset.
    /// Supports Four CC with byte values above 127 (non-standard ASCII)
    /// if the numerical values map to ISO8859-1,
    /// e.g. GoPro uses Four CC `©xyz` in `udta` atom.
    pub fn name(&mut self, pos: u64) -> Result<String, Mp4Error> {
        // let mut cursor = self.read_at(self.offset + 4, 4)?;
        let bytes: Vec<u8> = (pos..pos+4).into_iter()
            .map(|_| self.cursor.read_be::<u8>())
            .collect::<BinResult<Vec<u8>>>()?;
        let name: String = bytes.iter()
            .map(|b| *b as char)
            .collect();
        Ok(name)
    }

    /// Ensures user specified name (Four CC),
    /// matches that of current `Atom`.
    fn match_name(&self, name: &FourCC) -> Result<(), Mp4Error> {
        if &self.name != name {
            Err(Mp4Error::AtomMismatch{
                got: self.name.to_str().to_owned(),
                expected: name.to_str().to_owned()
            })
        } else {
            Ok(())
        }
    }

    /// Parse `Atom` into `Stts` (time-to sample) if `Atom.name` is `stts`,
    pub fn stts(&mut self) -> Result<Stts, Mp4Error> {
        self.match_name(&FourCC::Stts)?;

        // Seek past version (1 byte) + flags (3 bytes)
        self.cursor.seek(SeekFrom::Current(4))?;
        let no_of_entries = self.read::<u32>()?;

        let mut time_to_sample_table: Vec<u32> = Vec::new();
        for _ in 0..no_of_entries {
            let sample_count = self.read::<u32>()?;
            let sample_duration = self.read::<u32>()?;
            time_to_sample_table.extend(vec![sample_duration; sample_count as usize])
            // time_to_sample_table.append(&mut vec![sample_duration; sample_count as usize])
        }

        Ok(Stts::new(time_to_sample_table))
    }

    /// Parse `Atom` into `Stsz` (sample to size in bytes) if `Atom.name` is `stsz`,
    pub fn stsz(&mut self) -> Result<Stsz, Mp4Error> {
        self.match_name(&FourCC::Stsz)?;

        // Seek past version (1 byte) + flags (3 bytes)
        self.cursor.seek(SeekFrom::Current(4))?;

        let sample_size = self.read::<u32>()?;
        let no_of_entries = self.read::<u32>()?;

        let sizes = match sample_size {
            0 => {
                (0..no_of_entries).into_iter()
                    .map(|_| self.read::<u32>())
                    .collect()
            }
            // Is below really correct? If all samples have the same size
            // is no_of_entries still representative?
            _ => Ok(vec![sample_size; no_of_entries as usize]),
        };

        Ok(Stsz::new(sizes?))
    }

    /// Parse `Atom` into `Stco` (sample to size in bytes) if `Atom.name` is `stco`,
    pub fn stco(&mut self) -> Result<Stco, Mp4Error> {
        self.match_name(&FourCC::Stco)?;

        // Seek past version (1 byte) + flags (3 bytes)
        self.cursor.seek(SeekFrom::Current(4))?;

        // let sample_size = self.read::<u32>()?;
        let no_of_entries = self.read::<u32>()?;

        let offsets: BinResult<Vec<u32>> = (0..no_of_entries).into_iter()
            .map(|_| self.read::<u32>())
            .collect();

        Ok(Stco::new(offsets?))
    }

    /// Parse `Atom` into `Hdlr` if `Atom.name` is `hdlr`,
    pub fn hdlr(&mut self) -> Result<Hdlr, Mp4Error> {
        self.match_name(&&FourCC::Hdlr)?;

        // Seek past version (1 byte) + flags (3 bytes)
        self.cursor.seek(SeekFrom::Current(4))?;

        let component_type = self.cursor.read_be::<u32>()?;
        let component_sub_type = self.cursor.read_be::<u32>()?;
        let component_manufacturer = self.cursor.read_be::<u32>()?;
        let component_flags = self.cursor.read_be::<u32>()?;
        let component_flags_mask = self.cursor.read_be::<u32>()?;
        let component_name_size = self.cursor.read_be::<u8>()?;
        let mut component_name = String::with_capacity(component_name_size as usize);
        let read_bytes = self.cursor.read_to_string(&mut component_name)?;
        if read_bytes != component_name_size as usize {
            return Err(Mp4Error::ReadMismatch{got: read_bytes as u64, expected: component_name_size as u64})
        }

        Ok(Hdlr{
            component_type,
            component_sub_type,
            component_manufacturer,
            component_flags,
            component_flags_mask,
            component_name: component_name.trim().to_owned()
        })
    }

    pub fn udta(&mut self) -> Result<Udta, Mp4Error> {
        self.match_name(&&FourCC::Udta)?;

        let mut fields: Vec<UdtaField> = Vec::new();

        // Atom::size includes 8 byte header
        // TODO remove header bytes from cursor instead? Cleaner, consistent
        while self.cursor.position() < self.size - 8 {
            let size = self.read::<u32>()?;
            let name = FourCC::from_str(&self.name(self.cursor.position())?);

            let mut buf: Vec<u8> = vec![0; size as usize - 8];
            self.cursor.read_exact(&mut buf)?;

            let field = UdtaField {
                name,
                size,
                data: Cursor::new(buf)
            };

            fields.push(field)
        }

        Ok(Udta{fields})
    }
}
