//! Various MP4-related errors.

use std::fmt;

/// Various GPMF related read/parse errors.
#[derive(Debug)]
pub enum Mp4Error {
    /// Converted `BinResult` error.
    BinReadError(binread::Error),
    /// Converted `Utf8Error`.
    Utf8Error(std::string::FromUtf8Error),
    /// IO error
    IOError(std::io::Error),
    /// Filesizes of e.g. 0 sized place holders.
    ReadMismatch{got: u64, expected: u64},
    /// Failed to locate offsets for interleaved data
    /// from `stts`, `stsz`, and `stco` atoms in MP4.
    /// For e.g. GoPro MP4-files.
    NoOffsets,
    /// Seek mismatch.
    OffsetMismatch{got: u64, expected: u64},
    /// Atom mismatch.
    AtomMismatch{got: String, expected: String},
    /// MP4 0 sized atoms,
    /// e.g. 1k Dropbox place holders.
    UnexpectedAtomSize{len: u64, offset: u64},
    /// No such atom.
    NoSuchAtom(String),
    /// MP4 ouf of bounds.
    BoundsError((u64, u64)),
    /// Filesizes of e.g. 0 sized place holders.
    UnexpectedFileSize(u64),
    /// Unknown base type when parsing `Values`.
    UnknownBaseType(u8),
    /// Missing type definition for Complex type (`63`/`?`)
    MissingComplexType,
    /// Exceeded recurse depth when parsing GPMF into `Stream`s
    RecurseDepthExceeded((usize, usize)),
    /// Invalid FourCC. For detecting `&[0, 0, 0, 0]`.
    /// E.g. GoPro `udta` atom contains
    /// mainly undocumented GPMF data and is padded with
    /// zeros.
    InvalidFourCC,
}

impl std::error::Error for Mp4Error {} // not required?

impl fmt::Display for Mp4Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mp4Error::BinReadError(err) => write!(f, "{err}"),
            Mp4Error::Utf8Error(err) => write!(f, "{err}"),
            Mp4Error::IOError(err) => write!(f, "IO error: {}", err),
            Mp4Error::ReadMismatch{got, expected} => write!(f, "Read {got} bytes, expected {expected} bytes."),
            Mp4Error::OffsetMismatch{got, expected} => write!(f, "Moved {got} bytes, expected to move {expected} bytes"),
            Mp4Error::AtomMismatch{got, expected} => write!(f, "Atom mismatch. Expected '{expected}', got '{got}'"),
            Mp4Error::UnexpectedAtomSize{len, offset} => write!(f, "Unexpected MP4 atom size of {len} bytes @ offset {offset}."),
            Mp4Error::NoSuchAtom(name) => write!(f, "No such atom {name}."),
            Mp4Error::BoundsError((got, max)) => write!(f, "Bounds error: tried to read file at {got} with max {max}."),
            Mp4Error::UnexpectedFileSize(size) => write!(f, "Unexpected file size of {size} bytes."),
            Mp4Error::UnknownBaseType(bt) => write!(f, "Unknown base type {}/'{}'", bt, *bt as char),
            Mp4Error::MissingComplexType => write!(f, "Missing type definitions for complex type '?'"),
            Mp4Error::RecurseDepthExceeded((depth, max)) => write!(f, "Recurse depth {depth} exceeds max recurse depth {max}"),
            Mp4Error::InvalidFourCC => write!(f, "Invalid FourCC"),
            Mp4Error::NoOffsets => write!(f, "Failed to locate offsets ('stco'), chunk size ('stsz'), and time span ('stts') for interleaved data."),
        }
    }
}

/// Converts std::io::Error to Mp4Error
impl From<std::io::Error> for Mp4Error {
    fn from(err: std::io::Error) -> Self {
        Mp4Error::IOError(err)
    }
}

/// Converts std::string::FromUtf8Error to Mp4Error
/// (`&str` reqiures `std::str::Utf8Error`)
impl From<std::string::FromUtf8Error> for Mp4Error {
    fn from(err: std::string::FromUtf8Error) -> Mp4Error {
        Mp4Error::Utf8Error(err)
    }
}

/// Converts Mp4Error to std::io::Error
impl From<Mp4Error> for std::io::Error {
    fn from(err: Mp4Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err)
    }
}

/// Converts binread::Error to FitError
impl From<binread::Error> for Mp4Error {
    fn from(err: binread::Error) -> Mp4Error {
        Mp4Error::BinReadError(err)
    }
}