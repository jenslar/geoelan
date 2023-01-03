//! Partial representation of record (global ID 20) as logged by the Garmin VIRB.
//! Other devices may or may not have the fields covered, resulting in possible errors.

use std::ops::Range;
use time::Duration;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    fit::DataMessage,
    FitError,
    Fit
};

use super::gps_metadata::FitPoint;

/// Limited representation of `record`/20
/// that only covers position. For devices
/// that do not log to `gps_metadata`.
/// The non-optional fields are found in
/// FIT-files from both Garmin VIRB Ultra 30,
/// and (most?) watches.
#[derive(Debug, Clone, Copy)]
pub struct Record {
    pub timestamp: u32, // field 253, timestamp, SCL: 1 OFF: 0 UNIT:s UINT32([6326])
    pub latitude: i32, // field 0, position_lat, SCL: 1 OFF: 0 UNIT:semicircles SINT32([66924028])
    pub longitude: i32, // field 1, position_long, SCL: 1 OFF: 0 UNIT:semicircles SINT32([1210417996])
    pub distance: u32, // field 5, distance, SCL: 100 OFF: 0 UNIT:m UINT32([411482])
    pub speed: u32, // field 73, enhanced_speed, SCL: 1000 OFF: 0 UNIT:m s UINT32([569])
    pub altitude: Option<u32>, // field 78, enhanced_altitude, SCL: 5 OFF: 500 UNIT:m UINT32([5723])
    pub gps_accuracy: Option<u8>, // field 31, gps_accuracy, no SCL or OFF, UNIT:M UINT8([2])
    pub(crate) index: usize
}

impl Record {
    /// Returns a limited `TimestampCorrelation`/20
    /// from a restructured `DataMessage`.
    /// 
    /// Covers only GPS fields in FIT `record`/20 as logged by Garmin VIRB.
    pub fn new(data_message: &DataMessage) -> Result<Self, FitError> {
        let global_id = 20_u16; // record

        if data_message.global != global_id {
            return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        }

        let mut timestamp: Option<u32> = None;
        let mut latitude: Option<i32> = None;
        let mut longitude: Option<i32> = None;
        let mut altitude: Option<u32> = None;
        let mut speed: Option<u32> = None;
        let mut distance: Option<u32> = None;
        let mut gps_accuracy: Option<u8> = None;

        for field in data_message.fields.iter() {

            match field.field_def_no() {
                253 => timestamp = field.data.as_ref().into(),
                0 => latitude = field.data.as_ref().into(), // 0
                1 => longitude = field.data.as_ref().into(), // 1
                // altitude
                2 => {
                    // While a u16, field no 2/altitude
                    // has the same scale/value magnitude as 78/enhanced altitude
                    let altitude_u16: Option<u16> = field.data.as_ref().into();
                    altitude = altitude_u16.map(|v| v as u32);
                }, // 2 or 78
                // enhanced_altitude
                78 => altitude = field.data.as_ref().into(), // 2 or 78
                // speed
                6 => {
                    // While a u16, field no 6/speed
                    // has the same scale/value magnitude as 73/enhanced speed
                    let speed_u16: Option<u16> = field.data.as_ref().into();
                    speed = speed_u16.map(|v| v as u32);
                },
                // enhanced_speed
                73 => speed = field.data.as_ref().into(), // 6 or 73
                5 => distance = field.data.as_ref().into(),
                31 => gps_accuracy = field.data.as_ref().into(),
                _ => (), // ignore other fields for now
            }
        }

        Ok(Self {
            timestamp: timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 253})?,
            latitude: latitude
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 0})?,
            longitude: longitude
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            altitude,
            speed: speed
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 6})?, // or 73...
            distance: distance
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 5})?,
            gps_accuracy,
            index: data_message.index
        })
    }

    /// Parses limited set of record/20 fields representing location metrics
    /// and returns these in a more accessible form.
    /// 
    /// To only return valid records set `no_fail` to `true`.
    /// 
    /// If `no_fail` is set, errors are discarded and only records
    /// where required fields could be assigned are returned.
    /// 
    /// Error handling is for determining whether a required field
    /// could be assigned or not, i.e. whether the value was present in the input data.
    /// For e.g. watches that have not yet sync:ed with a GPS satellite,
    /// this function will return an error, since required fields can not yet be assigned.
    /// Set `no_fail = true` if this is unwanted.
    pub fn from_fit(
        fit: &Fit,
        range: Option<&Range<usize>>, // slice indeces for session
        no_fail: bool
    ) -> Result<Vec<Self>, FitError> {
        let global = 20_u16;

        let range = range.cloned().unwrap_or(0 .. fit.len());

        if no_fail {
            Ok(fit.records[range].par_iter()
                .filter_map(|rec| if rec.global == global {
                    Self::new(rec).ok()
                } else {
                    None
                })
                .collect())
        } else {
            fit.records[range].par_iter()
                .filter(|rec| rec.global == global)
                .map(Self::new)
                .collect()
        }
    }

    /// Convert `Record` geo-location to decimal degrees.
    pub fn to_point(&self) -> FitPoint {
        let semi2deg = 180.0 / 2.0_f64.powi(31);
        FitPoint {
            latitude: (self.latitude as f64) * semi2deg,
            longitude: (self.longitude as f64) * semi2deg,
            altitude: self.altitude.map(|alt| (alt as f64 / 5.0) - 500.0).unwrap_or(0.0),
            speed2d: self.speed as f64 / 1000.0,
            speed3d: 0.0,
            heading: 0.0, // not supported for record/20. self.heading as f64 / 100.0, // scale 100
            time: Duration::seconds(self.timestamp as i64),
            // time: chrono::Duration::seconds(self.timestamp as i64),
            text: None,
        }
    }
}

// /// Parses limited set of record/20 fields representing location metrics
// /// and returns these in a more accessible form.
// /// Error handling is for determining whether a required field
// /// could be assigned or not, i.e. whether the value was present in the input data.
// /// For e.g. watches that have not yet sync:ed with a GPS satellite,
// /// this function will return an error, since required fields can not yet be assigned.
// /// To only return valid records set `no_fail` to `true`.
// /// If `no_fail` is set, errors are discarded and only records
// /// where required fields could be assigned are returned.
// pub fn into_record(
//     fitfile: &Fit,
//     range: Option<&Range<usize>>, // slice indeces for session
//     no_fail: bool
// ) -> Result<Vec<Record>, FitError> {
//     let global = 20_u16;

//     let range = range.cloned().unwrap_or(0 .. fitfile.len());

//     if no_fail {
//         Ok(fitfile.records[range].par_iter()
//             .filter_map(|rec| if rec.global == global {
//                 Record::new(rec).ok()
//             } else {
//                 None
//             })
//             .collect())
//     } else {
//         fitfile.records[range].par_iter()
//             .filter(|rec| rec.global == global)
//             .map(Record::new)
//             .collect()
//     }
// }

