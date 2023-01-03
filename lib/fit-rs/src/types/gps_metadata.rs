//! FIT `gps_metadata` message (global ID 160). Only VIRB logs all described fields.
//! Other devices may return an error if `gps_metadata` exists.

use std::ops::Range;
use time::Duration;

use crate::{
    FitError,
    Fit,
    fit::DataMessage,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// VIRB only (message type may exist on other devices, but not all fields).
/// Parsed `gps_metadata` data message, global id 160.
/// 10Hz GPS log for Garmin VIRB Ultra 30.
/// Note: Garmin VIRB Ultra 30 logs additional data not documented in the FIT SDK, ignored here.
///       Run `geoelan check -f FITFILE -g 160 --verbose` to view these
#[derive(Debug)]
pub struct GpsMetadata {
    /// Field definition number 253, seconds
    pub timestamp: u32,
    /// Field definition number 0, milliseconds
    pub timestamp_ms: u16,
    /// Field definition number 1
    pub latitude: i32,
    /// Field definition number 2
    pub longitude: i32,
    /// Field definition number 3
    pub altitude: u32,
    /// Field definition number 4
    pub speed: u32,
    /// Field definition number 5
    pub heading: u16,
    pub utc_timestamp: u32, // id:6
    pub velocity: Vec<i16>, // id:7 Vec::with_capacity(3), x, y, z velocity values, was [i16;3]
                            // pub unknown: [u16;5] // id:8-12 not in Profile.xlsx, exists in definition message
    pub(crate) index: usize
}

impl GpsMetadata {
    /// Create new `GpsMetadata` struct from a `DataMessage`.
    pub fn new(data_message: &DataMessage) -> Result<Self, FitError> {
        let global_id = 160_u16; // gps_metadata

        if data_message.global != global_id {
            return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        }

        let mut timestamp: Option<u32> = None;
        let mut timestamp_ms: Option<u16> = None;
        let mut latitude: Option<i32> = None;
        let mut longitude: Option<i32> = None;
        let mut altitude: Option<u32> = None;
        let mut speed: Option<u32> = None;
        let mut heading: Option<u16> = None;
        let mut utc_timestamp: Option<u32> = None;
        let mut velocity: Option<Vec<i16>> = None; // [i16;3]

        for field in data_message.fields.iter() {
            match field.field_def_no() {
                253 => timestamp = field.data.as_ref().into(),
                0 => timestamp_ms = field.data.as_ref().into(),
                1 => latitude = field.data.as_ref().into(),
                2 => longitude = field.data.as_ref().into(),
                3 => altitude = field.data.as_ref().into(),
                4 => speed = field.data.as_ref().into(),
                5 => heading = field.data.as_ref().into(),
                6 => utc_timestamp = field.data.as_ref().into(),
                7 => velocity = field.data.as_ref().into(),
                // ignore undocumented id:s,
                // found 8, 9, 10, 11, 12 for VIRB Ultra 30 so far
                _ => (),
            }
        }

        Ok(Self {
            timestamp: timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 253})?,
            utc_timestamp: utc_timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 6})?,
            timestamp_ms: timestamp_ms
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 0})?,
            latitude: latitude
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            longitude: longitude
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 2})?,
            altitude: altitude
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 3})?,
            speed: speed
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 4})?,
            velocity: velocity
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 7})?,
            heading: heading
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 5})?,
            index: data_message.index
        })
    }

    /// VIRB only, partially in some other devices (these will probably return an error).
    /// Parses all gps_metadata/160 and returns these in a more accessible form.
    /// Error handling is for determining whether a required field 
    /// was present in the input data.
    /// 
    /// If FIT contains no gps_metadata/160,
    /// an empty vec will be returned with no errors
    pub fn from_fit(
        fit: &Fit,
        range: Option<&Range<usize>>, // slice indeces for session
    ) -> Result<Vec<Self>, FitError> {
        let global = 160_u16;

        let range = range.cloned().unwrap_or(0 .. fit.len());

        fit.records[range].par_iter()
            .filter(|rec| rec.global == global)
            .map(Self::new)
            .collect()
    }

    /// Convert gps_metadata basetype values to decimal degrees etc
    pub fn to_point(&self) -> FitPoint {
        let semi2deg = 180.0 / 2.0_f64.powi(31);
        FitPoint {
            latitude: (self.latitude as f64) * semi2deg,
            longitude: (self.longitude as f64) * semi2deg,
            altitude: (self.altitude as f64 / 5.0) - 500.0,
            speed2d: self.speed as f64 / 1000.0,
            speed3d: (
                self.velocity[0].pow(2) as f64 +
                self.velocity[1].pow(2) as f64 +
                self.velocity[2].pow(2) as f64
            ).sqrt() / 100.0,
            heading: self.heading as f64 / 100.0, // scale 100
            time: {
                Duration::seconds(self.timestamp as i64)
                    + Duration::milliseconds(self.timestamp_ms as i64)
            },
            text: None,
        }
    }

    /// Returns `[latitude, longitude, altitude]` as decimal degrees and meters.
    pub fn to_decimal(&self) -> [f64; 3] {
        let semi2deg = 180.0 / 2.0_f64.powi(31);
        [
            (self.latitude as f64) * semi2deg,
            (self.longitude as f64) * semi2deg,
            (self.altitude as f64 / 5.0) - 500.0 // FIT SDK: scale 5, offset 500
        ]
    }
}

/// For converting gps_metadata/161 to decimal values.
#[derive(Debug, Clone)]
pub struct FitPoint {
    pub latitude: f64,  // id:1
    pub longitude: f64, // id:2
    pub altitude: f64,  // id:3 f32?
    pub speed2d: f64,     // id:4 f32?
    pub speed3d: f64,     // id:4
    pub heading: f64, // id:5 f32?
    /// relative timestamp
    pub time: Duration,
    pub text: Option<String>,
}
