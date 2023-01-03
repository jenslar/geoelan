//! Timestamp correlation message (global ID 162). Logged by (at least) Garmin VIRB cameras
//! at satellite sync (?) to allow for shifting the relative timeline to an absolute one.
//! 
//! Other devices may not need or log this value.

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::fit::DataMessage;
use crate::{
    FitError,
    Fit
};

/// VIRB only.
/// Parsed `timestamp_correlation` data message, global id 162.
/// Important: presumably logged at satellite sync (?),
/// but does NOT always precede the first gps_metadata (160) message
#[derive(Debug, Copy, Clone)]
pub struct TimestampCorrelation {
    pub timestamp: u32,    // seconds
    pub timestamp_ms: u16, // milliseconds
    pub system_timestamp: u32,
    pub system_timestamp_ms: u16,
    pub(crate) index: usize
}

impl TimestampCorrelation {
    /// Returns a `TimestampCorrelation`/162 from a restructured `DataMessage`.
    pub fn new(data_message: &DataMessage) -> Result<Self, FitError> {
        let global_id = 162_u16;

        if data_message.global != global_id {
            return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        }

        let mut timestamp: Option<u32> = None;
        let mut system_timestamp: Option<u32> = None;
        let mut timestamp_ms: Option<u16> = None;
        let mut system_timestamp_ms: Option<u16> = None;

        for field in data_message.fields.iter() {
            match field.field_def_no() {
                // UTC seconds at time of logging timestamp_correlation
                253 => timestamp = field.data.as_ref().into(),
                // seconds since start of fit file
                1 => system_timestamp = field.data.as_ref().into(),
                // UTC fractional/milliseconds at time of logging timestamp_correlation
                4 => timestamp_ms = field.data.as_ref().into(),
                // milliseconds since start of fit file
                5 => system_timestamp_ms = field.data.as_ref().into(),
                _ => (),
            }
        }

        Ok(Self {
            timestamp: timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 253})?,
            timestamp_ms: timestamp_ms
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 4})?,
            system_timestamp: system_timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            system_timestamp_ms: system_timestamp_ms
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 5})?,
            index: data_message.index
        })
    }

    /// VIRB only.
    /// 
    /// Parses `timestamp_correlation/162` and returns these in a more accessible form.
    /// Only a single `timestamp_correlation` should be logged in each FIT-file.
    /// Error handling is for determining if a required field could not be assigned
    /// , i.e. was not present in input data.
    pub fn from_fit(
        fit: &Fit,
    ) -> Result<Self, FitError> {
        let global = 162_u16;
        
        fit.records.par_iter()
            .find_first(|rec| rec.global == global) // only one correlation value per fit file
            .map(Self::new) // how to forward error for single item, a la '.collect::<Result<_>>()?'
            .ok_or_else(|| FitError::ErrorParsingMessage(global))?
    }
}
