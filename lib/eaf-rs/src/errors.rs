use std::fmt;

#[derive(Debug)]
pub enum EafError {
    DependentTierExpected(&'static str),    // passed tier_id
    Utf8Error(std::str::Utf8Error),         // impl From
    IOError(std::io::Error),                // impl From
    QuickXMLError(quick_xml::Error),        // impl From
    ParseIntError(std::num::ParseIntError), // impl From
    TokenizedTier(String),                  // String -> TIER_ID
}

impl std::error::Error for EafError {}
impl fmt::Display for EafError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EafError::DependentTierExpected(e) => {
                write!(f, "{} is a main tier. Dependent tier expected.", e)
            }
            EafError::Utf8Error(err) => write!(f, "Error parsing bytes to string: {}", err),
            EafError::IOError(err) => write!(f, "IO error: {}", err),
            EafError::QuickXMLError(err) => write!(f, "QuickXML error parsing EAF: {}", err),
            EafError::ParseIntError(err) => write!(f, "Error parsing string to integer: {}", err),
            EafError::TokenizedTier(tier_id) => write!(
                f,
                "'{}' is a tokenized tier. Non-tokenized tier expected.",
                tier_id
            ),
        }
    }
}

/// Converts std::str::Utf8Error to EafError
impl From<std::str::Utf8Error> for EafError {
    fn from(err: std::str::Utf8Error) -> EafError {
        EafError::Utf8Error(err)
    }
}

/// Converts EafError to std::io::Error
impl From<EafError> for std::io::Error {
    fn from(err: EafError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // for returning EafErrors in main()
    }
}

/// Converts std::io::Error to EafError
impl From<std::io::Error> for EafError {
    fn from(err: std::io::Error) -> EafError {
        EafError::IOError(err)
    }
}

/// Converts quick_xml::Error to EafError
impl From<quick_xml::Error> for EafError {
    fn from(err: quick_xml::Error) -> EafError {
        EafError::QuickXMLError(err)
    }
}

/// Converts std::num::ParseIntError to EafError
impl From<std::num::ParseIntError> for EafError {
    fn from(err: std::num::ParseIntError) -> EafError {
        EafError::ParseIntError(err)
    }
}
