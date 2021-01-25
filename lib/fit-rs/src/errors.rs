// Custom Errors
// see https://users.rust-lang.org/t/solved-wrapper-around-result-that-can-return-both-the-error-and-the-value/19884
// for returning partial success
// cases have been encountered for all listed error types

use crate::messages::message_types::get_messagetype;
use crate::structs::{DataSize, FitData};
use std::fmt;

/// Various errors when parsing MP4 containers
#[derive(Debug)]
pub enum Mp4Error {
    // returned for some/of the functions while parsing
    UnexpectedFileSize(u64),
    UnexpectedAtomSize(u64),
    ErrorRetrieveingFileSize(std::io::Error),
    NoUUID,
}
impl std::error::Error for Mp4Error {} // not required?
impl fmt::Display for Mp4Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mp4Error::UnexpectedFileSize(size) => write!(f, "File has unexpected size {}", size),
            Mp4Error::UnexpectedAtomSize(size) => {
                write!(f, "MP4 atom has unexpected size {}", size)
            }
            Mp4Error::ErrorRetrieveingFileSize(err) => {
                write!(f, "Could not retrieve video size: {}", err)
            }
            Mp4Error::NoUUID => write!(f, "Video contains no UUID"),
        }
    }
}

/// Full FIT-file parse error
/// If Fatal(err) return error only
/// if Partial(err, data) return error and partial data read
#[derive(Debug)]
pub enum FitError {
    Fatal(ParseError),
    Partial(ParseError, FitData),
}
impl std::error::Error for FitError {} // not required?
impl fmt::Display for FitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FitError::Fatal(e) => write!(f, "{}", e),
            FitError::Partial(e, _) => write!(f, "{}, only partial read possible", e),
        }
    }
}

/// Converts ParseError to fatal FitError
impl From<ParseError> for FitError {
    fn from(err: ParseError) -> FitError {
        FitError::Fatal(err)
    }
}

/// Converts std::io::Error to fatal FitError
impl From<std::io::Error> for FitError {
    fn from(err: std::io::Error) -> FitError {
        FitError::Fatal(ParseError::IOError(err))
    }
}

/// Converts std::str::Utf8Error to ParseError
impl From<std::str::Utf8Error> for ParseError {
    fn from(err: std::str::Utf8Error) -> ParseError {
        ParseError::Utf8Error(err)
    }
}

/// Converts std::io::Error to ParseError
impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> ParseError {
        ParseError::IOError(err)
    }
}

/// Converts ParseError to std::io::Error
impl From<ParseError> for std::io::Error {
    fn from(err: ParseError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // for returning ParseErrors in main:s (ok?)
    }
}

/// Converts FitError to std::io::Error
impl From<FitError> for std::io::Error {
    fn from(err: FitError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // for returning ParseErrors in main:s (ok?)
    }
}

/// Converts Mp4Error to std::io::Error
impl From<Mp4Error> for std::io::Error {
    fn from(err: Mp4Error) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // parse mp4 -> uuid contains io err so...
    }
}

// impl From<std::str::Utf8Error> for std::io::Error {
//     fn from(err: std::str::Utf8Error) -> std::io::Error {
//         std::io::Error::new(std::io::ErrorKind::Other, err)
//     }
// }

/// Unimplemented features
#[derive(Debug, Copy, Clone)]
pub enum Feature {
    CompressedTimeStampHeader,
}

/// Various FIT parse errors
// #[derive(Debug, Copy, Clone)]
#[derive(Debug)]
pub enum ParseError {
    DataSizeExceedsFileSize(DataSize), // partial parse possible
    DataSizeDisrepancy(DataSize),      // if no data to return
    DataSizeZero(usize),               // if header specifies data size = 0
    UnexpectedHeaderSize(usize),       // if no data to return
    UnknownDefinition(u8),             // local id (u4) for unknown definition
    UnknownFieldDescription(u8),       // field description not found (developer data definition)
    UnknownBaseType(u8),               // fit base type not found
    UnknownSensorType(u16),            // unused, if sensor other than 164, 165, 208
    UnimplementedFeature(Feature),
    ErrorParsingField(u16, u8), // (Global Fit Message ID, Field Definition Number)
    ErrorAssigningFieldValue(u16, u8), // Global Fit Message ID
    ErrorParsingDataMessage(u16), // Global Fit Message ID
    MultipleDataError(u16),     // Global Fit Message ID, for types supposed to be logged once only
    InvalidArchitecture(u8),
    InvalidLengthForBasetypeCluster((usize, u8, usize)), // cluster length, base_type_numer, base length
    InvalidMessageHeader((u8, usize)), // message header value as u8 + index of FIT data portion
    NoDataForMessageType(u16),
    GenericParseError,              // for places that need re-design etc
    NoData(usize),                  // returns file size, but usually 0 with this error...
    Utf8Error(std::str::Utf8Error), // impl From
    IOError(std::io::Error),        // impl From
}

impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::DataSizeExceedsFileSize(ds) => write!(
                f,
                "Data size exceeds file size. Read {}. Expected {}.",
                ds.read, ds.expected
            ),
            ParseError::DataSizeDisrepancy(ds) => {
                write!(f, "Read {}. Expected {}.", ds.read, ds.expected)
            }
            ParseError::DataSizeZero(size) => {
                write!(f, "Header specifies data size 0. Read {}.", size)
            }
            ParseError::UnexpectedHeaderSize(size) => write!(
                f,
                "Unexpected header size. Read {}. Expected 12 or 14.",
                size
            ),
            ParseError::UnknownDefinition(def) => write!(f, "Unknown definition {}", def),
            ParseError::UnknownBaseType(basetype) => write!(f, "Unknown Base Type {}", basetype),
            ParseError::UnknownSensorType(sens) => write!(f, "Unknown Sensor Type {}", sens), // unused
            ParseError::UnknownFieldDescription(fd) => {
                write!(f, "Unknown Field Description {}", fd)
            }
            ParseError::UnimplementedFeature(ftr) => write!(f, "{:?} not implemented", ftr),
            ParseError::InvalidArchitecture(arc) => write!(f, "Invalid Architecture {}", arc),
            ParseError::InvalidLengthForBasetypeCluster((cllen, bt, btlen)) => write!(f, "Invalid length {} for cluster of FIT base type {} with base length {}", cllen, bt, btlen),
            ParseError::InvalidMessageHeader((hdr, idx)) => {
                write!(f, "Invalid Message Header {} at index {}", hdr, idx)
            }
            ParseError::ErrorParsingDataMessage(glob) => write!(
                f,
                "Error parsing data message {} ({})",
                glob,
                get_messagetype(*glob)
            ),
            ParseError::MultipleDataError(glob) => write!(
                f,
                "Data message {} ({}) can not have multiple entries",
                glob,
                get_messagetype(*glob)
            ),
            ParseError::NoDataForMessageType(glob) => write!(
                f,
                "No data for message type {} ({})",
                glob,
                get_messagetype(*glob)
            ),
            ParseError::GenericParseError => write!(f, "Error parsing FIT-file"),
            ParseError::ErrorParsingField(global, field_def) => {
                write!(f, "Error parsing field {} for global {}", field_def, global)
            }
            ParseError::ErrorAssigningFieldValue(global, field) => {
                write!(f, "Error assigning field {} for global {}", field, global)
            }
            ParseError::NoData(size) => write!(f, "No data in FIT-file with file size {}", size),
            ParseError::Utf8Error(err) => write!(f, "Error parsing bytes to string: {}", err),
            ParseError::IOError(err) => write!(f, "IO error: {}", err),
        }
    }
}
