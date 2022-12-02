//! Various errors for parsing, processing and building EAF-files.

use std::fmt;

#[derive(Debug)]
/// Various errors for parsing, processing and building EAF-files.
pub enum EafError {
    /// Number of annotations in ref tier
    /// exceed those in parent tier.
    /// (parent tier id, ref tier id)
    RefTierAlignmentError((String, String)),
    /// Expected referred tier.
    RefTierExpected(String),
    /// Tiers incompatible for e.g. merging
    /// if one is a referred tier and the other is not.
    /// (Tier ID 1, Tier ID 2)
    IncompatibleTiers((String, String)),
    /// Error decoding string as UTF-8.
    Utf8Error(std::str::Utf8Error),
    /// IO error.
    IOError(std::io::Error),
    /// Quick-xml error.
    QuickXMLError(quick_xml::Error),
    /// Quick-xml deserialization error.
    QuickXMLDeError(quick_xml::DeError),
    /// Error parsing integer from string.
    ParseIntError(std::num::ParseIntError),
    /// Error parsing float from string.
    ParseFloatError(std::num::ParseFloatError),
    /// Unexpected tokenized tier.
    TokenizedTier(String),
    /// Invalid tier ID.
    InvalidTierId(String),
    /// Invalid annotation ID.
    InvalidAnnotationId(String),
    /// Invalid time slot ID.
    InvalidTimeSlotId(String),
    /// Missing time slot reference for annotation. 
    MissingTimeslotRef(String),
    /// Missing time slot value for annotation ID (not part of EAF specification).
    MissingTimeslotVal(String),
    /// Error when filtering media, time slots etc on time.
    InvalidTimeSpan((i64, i64)),
    // InvalidTimeSpan((u64, u64)),
    /// Missing tier ID for annotation.
    /// Set if derived.
    MissingTierID(String),
    /// Missing annotation ID.
    MissingAnnotationID(String),
    /// Missing main annotation for ref annotation.
    /// `(ANNOTATION_ID, Option<REF_ANNOTATION>)`
    MissingMainAnnotation((String, Option<String>)),
    /// Annotation ID already exists (e.g. when adding new annotations).
    AnnotationIDExists(String),
    /// Encounterd referred tier, expected main tier.
    RefTier(String),
    /// Encounterd referred annotation, expected main annotation.
    RefAnnotation(String),
    /// Missing file name (when e.g. trying to extract section from media file path)
    MissingFileName(String),
    /// Missing file extension (when e.g. trying to extract section from media file path)
    MissingFileExtension(String),
    /// Invalid path.
    InvalidPath(String),
    /// Value is too small to be used in this context.
    /// E.g. negative time slot values.
    ValueTooSmall(i64),
    /// Value is too large to be used in this context.
    /// E.g. time slot value exceeds media duration.
    ValueTooLarge(i64),
}

impl std::error::Error for EafError {}
impl fmt::Display for EafError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EafError::RefTierAlignmentError((t1, t2)) => write!(f, "Annotations in referred tier '{t2}' exceed those in parent tier '{t1}'"),
            EafError::RefTierExpected(e) => {
                write!(f, "{} is a main tier. Expected dependent tier.", e)
            }
            EafError::IncompatibleTiers((id1, id2)) => write!(f, "The tiers '{id1}' and '{id2}' do not have compatible type."),
            EafError::Utf8Error(err) => write!(f, "Error parsing bytes to string: {}", err),
            EafError::IOError(err) => write!(f, "IO error: {}", err),
            EafError::QuickXMLError(err) => write!(f, "QuickXML error parsing EAF: {}", err),
            EafError::QuickXMLDeError(err) => write!(f, "QuickXML error deserialising EAF: {}", err),
            EafError::ParseIntError(err) => write!(f, "Error parsing string to integer: {}", err),
            EafError::ParseFloatError(err) => write!(f, "Error parsing string to float: {}", err),
            EafError::TokenizedTier(tier_id) => write!(
                f,
                "'{}' is a tokenized tier. Non-tokenized tier expected.",
                tier_id
            ),
            EafError::InvalidTierId(tier_id) => write!(f, "No such tier '{}'", tier_id),
            EafError::InvalidAnnotationId(annotation_id) => write!(f, "No such annotation '{}'", annotation_id),
            EafError::InvalidTimeSlotId(time_slot_id) => write!(f, "No such time slot '{}'", time_slot_id),
            EafError::MissingTimeslotRef(annotation_id) => write!(f, "No time slot reference for annotation/s with ID {}.", annotation_id),
            EafError::MissingTimeslotVal(annotation_val) => write!(f, "No time slot value for annotation/s with ID {}.", annotation_val),
            EafError::InvalidTimeSpan((start, end)) => write!(f, "Invalid time span {}ms-{}ms", start, end),
            EafError::MissingTierID(annotation_id) => write!(f, "Tier ID not set for annotation with ID '{}'", annotation_id),
            EafError::MissingAnnotationID(annotation_id) => write!(f, "No annotation with ID '{}'", annotation_id),
            EafError::MissingMainAnnotation((annotation_id, ref_annotation)) => write!(
                f, "Missing main annotation for ID '{}'. No main annotation with ID '{}'",
                annotation_id,
                ref_annotation.as_deref().unwrap_or_else(|| "NONE")),
            EafError::AnnotationIDExists(annotation_id) => write!(f, "Annotation '{}' already exists", annotation_id),
            EafError::RefTier(tier_id) => write!(f, "Tier '{}' is referred", tier_id),
            EafError::RefAnnotation(annotation_id) => write!(f, "Annotation '{}' is referred", annotation_id),
            EafError::MissingFileName(path) => write!(f, "No file name in path '{}'", path),
            EafError::MissingFileExtension(path) => write!(f, "No file extion in path '{}'", path),
            EafError::InvalidPath(path) => write!(f, "No such file '{}'", path),
            EafError::ValueTooSmall(num) => write!(f, "Value '{}' is too small in this context.", num),
            EafError::ValueTooLarge(num) => write!(f, "Value '{}' is too large in this context.", num),
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

/// Converts fast_xml::Error to EafError
impl From<quick_xml::Error> for EafError {
    fn from(err: quick_xml::Error) -> EafError {
        EafError::QuickXMLError(err)
    }
}

/// Converts quick_xml::DeError to EafError
impl From<quick_xml::DeError> for EafError {
    fn from(err: quick_xml::DeError) -> EafError {
        EafError::QuickXMLDeError(err)
    }
}

/// Converts std::num::ParseIntError to EafError
impl From<std::num::ParseIntError> for EafError {
    fn from(err: std::num::ParseIntError) -> EafError {
        EafError::ParseIntError(err)
    }
}

/// Converts std::num::ParseFloatError to EafError
impl From<std::num::ParseFloatError> for EafError {
    fn from(err: std::num::ParseFloatError) -> EafError {
        EafError::ParseFloatError(err)
    }
}
