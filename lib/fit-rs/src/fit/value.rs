use std::io::Cursor;

use binread::{BinRead, BinReaderExt};

use crate::errors::FitError;

use super::DefinitionField;

/// FIT data types.
/// See FIT SDK for specifics.
// #[derive(Debug, Clone, BinRead)]
#[derive(Debug, Clone)]
pub enum Value {
    /// Enum is single-byte value according to FIT SDK,
    /// but it still appears as an array (Vec<u8>,
    /// rather than single u8) occasionally
    /// in some FIT-files (firmware bug?)
    /// causing offset issues if not fully read.
    Enum(Vec<u8>), // 0
    Sint8(Vec<i8>), // 1
    Uint8(Vec<u8>), // 2
    Sint16(Vec<i16>), // 3
    Uint16(Vec<u16>), // 4
    Sint32(Vec<i32>), // 5
    Uint32(Vec<u32>), // 6
    // /// String as binread::strings::NullString
    // /// Null terminated string that consumes,
    // /// but does not include, null byte
    // binread NullString does not work for FIT...
    // perhaps since there
    // may be more than one 0? i.e. if
    // terminated at first 0 the remaining
    // 0-padding is not consumed?
    // #[br(align_before = 0xA)]
    // String(NullString), // 7,
    String(String), // 7,
    Float32(Vec<f32>), // 8
    Float64(Vec<f64>), // 9
    Uint8z(Vec<u8>), // 10 Z = ?
    Uint16z(Vec<u16>), // 11 Z = ?
    Uint32z(Vec<u32>), // 12 Z = ?
    Byte(Vec<u8>), // 13
    Sint64(Vec<i64>), // 14
    Uint64(Vec<u64>), // 15
    Uint64z(Vec<u64>), // 16 Z = ?
}

impl AsRef<Value> for Value {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Into<Option<u8>> for &Value {
    fn into(self) -> Option<u8> {
        match self {
            Value::Uint8(n)
            | Value::Uint8z(n)
            | Value::Byte(n)
            | Value::Enum(n) => n.first().cloned(),
            _ => None,
        }
    }
}

impl Into<Option<i8>> for &Value {
    fn into(self) -> Option<i8> {
        match self {
            Value::Sint8(n) => n.first().cloned(),
            _ => None,
        }
    }
}

impl Into<Option<i16>> for &Value {
    fn into(self) -> Option<i16> {
        match self {
            Value::Sint16(n) => n.first().cloned(),
            _ => None,
        }
    }
}

impl Into<Option<Vec<i16>>> for &Value {
    fn into(self) -> Option<Vec<i16>> {
        match self {
            Value::Sint16(n) => Some(n.to_owned()),
            _ => None,
        }
    }
}

impl Into<Option<u16>> for &Value {
    fn into(self) -> Option<u16> {
        match self {
            Value::Uint16(n) 
            | Value::Uint16z(n) => n.first().cloned(),
            _ => None,
        }
    }
}

impl Into<Option<Vec<u16>>> for &Value {
    fn into(self) -> Option<Vec<u16>> {
        match self {
            Value::Uint16(n) 
            | Value::Uint16z(n) => Some(n.to_owned()),
            _ => None,
        }
    }
}

impl Into<Option<i32>> for &Value {
    fn into(self) -> Option<i32> {
        match self {
            Value::Sint32(n) => n.first().cloned(),
            _ => None,
        }
    }
}

impl Into<Option<Vec<i32>>> for &Value {
    fn into(self) -> Option<Vec<i32>> {
        match self {
            Value::Sint32(n) => Some(n.to_owned()),
            _ => None,
        }
    }
}

impl Into<Option<u32>> for &Value {
    fn into(self) -> Option<u32> {
        match self {
            Value::Uint16(n) 
            | Value::Uint16z(n) => n.first().cloned().map(u32::from),
            Value::Uint32(n)
            | Value::Uint32z(n) => n.first().cloned(),
            _ => None,
        }
    }
}

impl Into<Option<Vec<u32>>> for &Value {
    fn into(self) -> Option<Vec<u32>> {
        match self {
            Value::Uint16(n) 
            | Value::Uint16z(n) => Some(n.iter().map(|v| u32::from(*v)).collect()),
            Value::Uint32(n)
            | Value::Uint32z(n) => Some(n.to_owned()),
            _ => None,
        }
    }
}

impl Into<Option<String>> for &Value {
    fn into(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.to_owned()),
            _ => None,
        }
    }
}

impl Value {
    /// Reads FIT values from `std::io::Cursor`
    /// with endianess derived via `arch`
    /// (`0` = Little Endian, `1` = Big Endian)
    /// into `Vec<T>`.
    fn read<T: Sized + BinRead>(
        cursor: &mut Cursor<Vec<u8>>,
        arch: u8,
        repeats: u8
    ) -> Result<Vec<T>, FitError> {
        match arch {
            // Little Endian
            0 => (0..repeats).into_iter()
                    .map(|_| cursor.read_le::<T>()
                        .map_err(|err| FitError::BinReadError(err)))
                    .collect(),
            // Big Endian
            1 => (0..repeats).into_iter()
                    .map(|_| cursor.read_be::<T>()
                        .map_err(|err| FitError::BinReadError(err)))
                    .collect(),
            // Invalid architecture value
            _ => Err(FitError::InvalidArchitecture{arch, pos: cursor.position()})
        }
    }

    /// Read and convert byte slice to checked or lossy UTF-8 string.
    /// While `lossy` is optional, it's still `true` by default
    /// due to some FIT-files containing corrupt strings.
    fn from_utf8(
        cursor: &mut Cursor<Vec<u8>>,
        arch: u8,
        repeats: u8,
        lossy: bool
    ) -> Result<String, FitError> {
        let bytes = Self::read::<u8>(cursor, arch, repeats)?;
        if lossy {
            Ok(String::from_utf8_lossy(&bytes).replace(char::from(0), "").to_string())
        } else {
            String::from_utf8(bytes).map_err(|e| e.into())
        }
    }

    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        field_def: &DefinitionField,
        architecture: u8
    ) -> Result<Self, FitError> {

        // if architecture > 1 {
        //     // arch must be 0 (little endian) or 1 (bigendian)
        //     return Err(FitError::InvalidArchitecture{arch: architecture, pos: cursor.position()})
        // }

        let base_len = field_def.base_type.base_len()?;
        // TODO add alignment check? e.g. if size % len != 0 -> Err()
        let repeats = field_def.size / base_len;

        match field_def.base_type.number() {
            // Changed Value::Enum from reading into single u8,
            // to reading into Vec<u8>, since some devices
            // OCCASIONALLY define Enum with total length > 1...?
            0 => Ok(Self::Enum(Self::read::<u8>(cursor, architecture, repeats)?)),
            1 => Ok(Self::Sint8(Self::read::<i8>(cursor, architecture, repeats)?)),
            2 => Ok(Self::Uint8(Self::read::<u8>(cursor, architecture, repeats)?)),
            3 => Ok(Self::Sint16(Self::read::<i16>(cursor, architecture, repeats)?)),
            4 => Ok(Self::Uint16(Self::read::<u16>(cursor, architecture, repeats)?)),
            5 => Ok(Self::Sint32(Self::read::<i32>(cursor, architecture, repeats)?)),
            6 => Ok(Self::Uint32(Self::read::<u32>(cursor, architecture, repeats)?)),
            // Parsing bytes as lossy utf8 due to corrupt (?) strings in some fit files.
            7 => Ok(Self::String(Self::from_utf8(cursor, architecture, repeats, true)?)),
            8 => Ok(Self::Float32(Self::read::<f32>(cursor, architecture, repeats)?)),
            9 => Ok(Self::Float64(Self::read::<f64>(cursor, architecture, repeats)?)),
            10 => Ok(Self::Uint8z(Self::read::<u8>(cursor, architecture, repeats)?)),
            11 => Ok(Self::Uint16z(Self::read::<u16>(cursor, architecture, repeats)?)),
            12 => Ok(Self::Uint32z(Self::read::<u32>(cursor, architecture, repeats)?)),
            13 => Ok(Self::Byte(Self::read::<u8>(cursor, architecture, repeats)?)),
            14 => Ok(Self::Sint64(Self::read::<i64>(cursor, architecture, repeats)?)),
            15 => Ok(Self::Uint64(Self::read::<u64>(cursor, architecture, repeats)?)),
            16 => Ok(Self::Uint64z(Self::read::<u64>(cursor, architecture, repeats)?)),
            b => Err(FitError::UnknownBaseType(b))
        }
    }
}
