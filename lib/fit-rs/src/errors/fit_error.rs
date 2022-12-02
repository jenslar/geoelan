//! Various FIT related parse errors.

use std::{fmt, string::FromUtf8Error};

#[derive(Debug)]
pub enum FitError {
    /// Converted `BinResult` error.
    BinReadError(binread::Error),
    /// Converted `time::Error` error.
    TimeError(time::Error),
    /// Converted `Utf8Error`.
    Utf8Error(std::str::Utf8Error),
    /// Converted `FromUtf8Error`.
    FromUtf8Error(FromUtf8Error),
    /// Converted `std::io::Error`.
    IOError(std::io::Error),
    /// Converted `mp4iter::errors::Mp4Error`.
    Mp4Error(mp4iter::errors::Mp4Error),
    /// Filesizes of e.g. 0 sized place holders.
    UnexpectedFileSize(u64),
    /// Filesizes of e.g. 0 sized place holders.
    ReadMismatch{got: u64, expected: u64},
    /// MP4 0 sized atoms,
    /// e.g. 1k Dropbox place holders.
    UnexpectedAtomSize(u64),
    /// Architecture error. Specifies endianess,
    /// and must be either 0 (Little),
    /// or 1 (Big).
    InvalidArchitecture{arch: u8, pos: u64},
    /// Unexpected FIT header size.
    /// Valid sizes are 12 or 14 bytes.
    UnexpectedHeaderSize(usize),
    /// Unknown local id (u4).
    /// Matches definitions with data messages.
    UnknownDefinition{local: u8, offset: u64},
    /// Unknown FIT base type number.
    UnknownBaseType(u8),
    /// Unknown developer field description (FIT global ID 206)
    UnknownFieldDescription{field_number: u8, developer_data_index: u8},
    /// FIT global ID mismatch
    UnexpectedMessageType{expected: u16, got: u16},
    /// Error parsing data message field.
    ErrorParsingField{global: u16, field_def_no: u8},
    /// Error assigning data message field.
    ErrorAssigningField{global: u16, field_def_no: u8},
    /// Error parsing data message field.
    ErrorParsingMessage(u16),
    /// Invalid VIRB MP4 file, i.e. it no UUID embeded.
    InvalidVirbMp4,
    /// Invalid VIRB MP4 file, i.e. it no UUID embeded.
    MissingVideo,
    NoSuchSession,
    NoData,
}

impl std::error::Error for FitError {}
impl fmt::Display for FitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FitError::BinReadError(err) => write!(f, "{err}"),
            FitError::TimeError(err) => write!(f, "{err}"),
            FitError::Utf8Error(err) => write!(f, "{err}"),
            FitError::FromUtf8Error(err) => write!(f, "{err}"),
            FitError::IOError(err) => write!(f, "{err}"),
            FitError::Mp4Error(err) => write!(f, "{err}"),
            FitError::UnexpectedFileSize(size) => write!(f, "Unexpected file size of {size} bytes."),
            FitError::ReadMismatch{got, expected} => write!(f, "Read {got} bytes, expected {expected} bytes."),
            FitError::UnexpectedAtomSize(size) => write!(f, "Unexpected MP4 atom size of {size} bytes."),
            FitError::InvalidArchitecture{arch, pos} => write!(f, "Invalid architecture: {arch} at position {pos}. Must be 0 (Little Endian), or 1 (Big Endian)."),
            FitError::UnexpectedHeaderSize(size) => write!(f,
                "Unexpected header size {size}. Valid sizes are 12 or 14 bytes."),
            FitError::UnknownDefinition{local, offset} => write!(f,
                "Unknown local definition ID {local} at offset {offset}."),
            FitError::UnknownBaseType(id) => write!(f,
                "Unknown base type ID {id}."),
            FitError::UnknownFieldDescription{field_number, developer_data_index} => write!(f,
                "Unknown field description with field number {field_number}, developer data index {developer_data_index}."),
            FitError::UnexpectedMessageType{expected, got} => write!(f,
                "Global ID mismatch: Exptected {expected}, got {got}."),
            FitError::ErrorParsingField{global, field_def_no} => write!(f,
                "Failed to parse field with FIT global ID {global}, field definition number {field_def_no}."),
            FitError::ErrorAssigningField{global, field_def_no} => write!(f,
                "Failed to assign field with FIT global ID {global}, field definition number {field_def_no}."),
            FitError::ErrorParsingMessage(global) => write!(f,
                "Failed to parse message with FIT global ID {global}."),
            FitError::InvalidVirbMp4 => write!(f,
                "Not a valid VIRB MP4-file."),
            FitError::MissingVideo => write!(f,
                "No video file in session."),
            FitError::NoSuchSession => write!(f,
                "Failed to retrieve VIRB session."),
            FitError::NoData => write!(f,
                "Failed to extract data."),
        }
    }
}

/// Converts std::io::Error to FitError
impl From<std::io::Error> for FitError {
    fn from(err: std::io::Error) -> FitError {
        FitError::IOError(err)
    }
}

/// Converts FitError to std::io::Error
impl From<FitError> for std::io::Error {
    fn from(err: FitError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // for returning FitParseErrors in main:s (ok?)
    }
}

/// Converts std::str::Utf8Error to FitError
impl From<std::str::Utf8Error> for FitError {
    fn from(err: std::str::Utf8Error) -> FitError {
        FitError::Utf8Error(err)
    }
}

/// Converts std::string::FromUtf8Error to FitError
impl From<std::string::FromUtf8Error> for FitError {
    fn from(err: std::string::FromUtf8Error) -> FitError {
        FitError::FromUtf8Error(err)
    }
}

/// Converts `binread::Error` to `FitError`
impl From<binread::Error> for FitError {
    fn from(err: binread::Error) -> FitError {
        FitError::BinReadError(err)
    }
}

/// Converts time::Error to FitError
impl From<time::Error> for FitError {
    fn from(err: time::Error) -> FitError {
        FitError::TimeError(err)
    }
}

/// Converts binread::Error to FitError
impl From<time::error::ComponentRange> for FitError {
    fn from(err: time::error::ComponentRange) -> FitError {
        FitError::TimeError(err.into())
    }
}

/// Converts binread::Error to FitError
impl From<mp4iter::errors::Mp4Error> for FitError {
    fn from(err: mp4iter::errors::Mp4Error) -> FitError {
        FitError::Mp4Error(err)
    }
}
