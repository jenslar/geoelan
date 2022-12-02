use std::ops::Range;
use time::Duration;
// use std::time::Duration;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::Fit;
use crate::{FitError, fit::DataMessage};

/// VIRB only.
/// Parsed `camera_event` data message, global id 161.
/// Contains UUID for the corresponding video file (present in both MP4 and GLV).
#[derive(Debug)]
pub struct CameraEvent {
    /// Field definition number 253: seconds
    pub timestamp: u32,
    /// Field definition number 0: milliseconds
    pub timestamp_ms: u16,
    /// Field definition number 2: camera_file_uuid
    pub camera_file_uuid: String,
    /// Field definition number 1: camera_event_type
    pub camera_event_type: u8,
    /// Field definition number 3: camera_orientation
    pub camera_orientation: u8,
    pub(crate) index: usize
}

impl CameraEvent {
    pub fn new(data_message: &DataMessage) -> Result<CameraEvent, FitError> {
        let global_id = 161_u16;

        if data_message.global != global_id {
            return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        }

        let mut timestamp: Option<u32> = None;
        let mut timestamp_ms: Option<u16> = None;
        let mut camera_event_type: Option<u8> = None;
        let mut camera_file_uuid: Option<String> = None;
        let mut camera_orientation: Option<u8> = None;

        for field in data_message.fields.iter() {
            match field.field_def_no() {
                253 => timestamp = field.data.as_ref().into(),
                0 => timestamp_ms = field.data.as_ref().into(),
                1 => camera_event_type = field.data.as_ref().into(),
                2 => camera_file_uuid = field.data.as_ref().into(),
                3 => camera_orientation = field.data.as_ref().into(),
                _ => (),
            }
        }

        Ok(CameraEvent {
            timestamp: timestamp
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 253})?,
            timestamp_ms: timestamp_ms
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 0})?,
            camera_file_uuid: camera_file_uuid
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 2})?,
            camera_event_type: camera_event_type
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            camera_orientation: camera_orientation
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 3})?,
            index: data_message.index
        })
    }

    /// VIRB only.
    /// Transforms all `DataMessage`s that correspond to camera_event/161 into a more accessible form.
    /// Error handling is for determining if a required field could not be assigned,
    /// i.e. was not present in input data.
    pub fn from_fit(
        fit: &Fit,
        range: Option<&Range<usize>>, // slice indeces for session
    ) -> Result<Vec<Self>, FitError> {
        let global = 161_u16;
    
        let range = range.cloned().unwrap_or(0 .. fit.len());
    
        fit.records[range].par_iter()
            .filter(|rec| rec.global == global)
            .map(Self::new)
            .collect()
    }

    pub fn to_duration(&self) -> Duration {
        Duration::seconds(self.timestamp as i64)
        + Duration::milliseconds(self.timestamp_ms as i64)
    }
}

// /// VIRB only.
// /// Transforms all `DataMessage`s that correspond to camera_event/161 into a more accessible form.
// /// Error handling is for determining if a required field could not be assigned,
// /// i.e. was not present in input data.
// pub fn into_cameraevent(
//     fitfile: &Fit,
//     range: Option<&Range<usize>>, // slice indeces for session
// ) -> Result<Vec<CameraEvent>, FitError> {
//     let global = 161_u16;

//     let range = range.cloned().unwrap_or(0 .. fitfile.len());

//     fitfile.records[range].par_iter()
//         .filter(|rec| rec.global == global)
//         .map(CameraEvent::new)
//         .collect()
// }

// pub fn session_indeces(cam: &[CameraEvent], uuid: &str) -> Option<Range<usize>> {
//     let mut start = None;
//     let mut end = None;

//     for c in cam.iter() {
//         // Find start of session
//         if c.camera_file_uuid == uuid && c.camera_event_type == 0 {
//             start = Some(c.index);
//         // Find end of session
//         } else if start.is_some() && c.camera_event_type == 2 {
//             end = Some(c.index + 1); // +1 since used as index with exclusive upper limit
//             break // break early or risk re-assigning 'end' with UUID from other session
//         }
//     }
    
//     if let (Some(s), Some(e)) = (start, end) {
//         Some(s..e)
//     } else {
//         None
//     }
// }
