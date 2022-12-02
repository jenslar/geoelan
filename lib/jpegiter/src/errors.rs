use std::fmt;

/// Various GPMF related read/parse errors.
#[derive(Debug)]
pub enum JpegError {
    /// Converted `BinResult` error.
    BinReadError(binread::Error),
    /// Converted `Utf8Error`.
    Utf8Error(std::string::FromUtf8Error),
    /// IO error
    IOError(std::io::Error),
    /// Filesizes of e.g. 0 sized place holders.
    ReadMismatch{got: u64, expected: u64},
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
    /// Failed to read JPEG tag at offset
    TagReadError(u64),
}

impl std::error::Error for JpegError {} // not required?

impl fmt::Display for JpegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JpegError::BinReadError(err) => write!(f, "{err}"),
            JpegError::Utf8Error(err) => write!(f, "{err}"),
            JpegError::IOError(err) => write!(f, "IO error: {}", err),
            JpegError::ReadMismatch{got, expected} => write!(f, "Read {got} bytes, expected {expected} bytes."),
            JpegError::OffsetMismatch{got, expected} => write!(f, "Moved {got} bytes, expected to move {expected} bytes"),
            JpegError::AtomMismatch{got, expected} => write!(f, "Atom mismatch. Expected '{expected}', got '{got}'"),
            JpegError::UnexpectedAtomSize{len, offset} => write!(f, "Unexpected MP4 atom size of {len} bytes @ offset {offset}."),
            JpegError::NoSuchAtom(name) => write!(f, "No such atom {name}."),
            JpegError::BoundsError((got, max)) => write!(f, "Bounds error: tried to read file at {got} with max {max}."),
            JpegError::UnexpectedFileSize(size) => write!(f, "Unexpected file size of {size} bytes."),
            JpegError::UnknownBaseType(bt) => write!(f, "Unknown base type {}/'{}'", bt, *bt as char),
            JpegError::MissingComplexType => write!(f, "Missing type definitions for complex type '?'"),
            JpegError::RecurseDepthExceeded((depth, max)) => write!(f, "Recurse depth {depth} exceeds max recurse depth {max}"),
            JpegError::InvalidFourCC => write!(f, "Invalid FourCC"),
            JpegError::TagReadError(offset) => write!(f, "Failed to read tag at offset {}", offset),
        }
    }
}

/// Converts std::io::Error to JpegError
impl From<std::io::Error> for JpegError {
    fn from(err: std::io::Error) -> Self {
        JpegError::IOError(err)
    }
}

/// Converts std::string::FromUtf8Error to JpegError
/// (`&str` reqiures `std::str::Utf8Error`)
impl From<std::string::FromUtf8Error> for JpegError {
    fn from(err: std::string::FromUtf8Error) -> JpegError {
        JpegError::Utf8Error(err)
    }
}

/// Converts JpegError to std::io::Error
impl From<JpegError> for std::io::Error {
    fn from(err: JpegError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err)
    }
}

/// Converts binread::Error to FitError
impl From<binread::Error> for JpegError {
    fn from(err: binread::Error) -> JpegError {
        JpegError::BinReadError(err)
    }
}