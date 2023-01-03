//! FIT message types as described in [FIT SDK](https://developer.garmin.com/fit/overview/) Profile.xslx.

use std::collections::HashMap;

use super::profile::get_messagetype;

/// FIT Message Type, from Messages sheet in Profile.xlsx
#[derive(Debug, Clone)]
pub struct FitMessageType {
    /// mesg_num in Profile.xlsx "Types" sheet
    pub global_id: u16,
    /// first column in Profile.xlsx "Messages" sheet
    pub name: String,
    /// Key: field_def_no, Value: Fields as specified in Profile.xlsx "Messages" sheet
    pub fields: HashMap<u8, FitFieldType>,
}

impl FitMessageType {
    pub fn get(global_id: u16) -> Self {
        get_messagetype(global_id)
    }
}

/// FIT Message Field Type as specified in Profile.xlsx "Messages" sheet
/// Used for getting field name and transforming raw values.
/// Note that not some fields may not have scale/offset,
/// but still require further processing, e.g. latitude/longitude
/// in gps_metadata/160.
#[derive(Debug, Clone)]
pub struct FitFieldType {
    pub field_def_no: u8, // numerical, some are unfortunately strings in Profile.xlsx
    pub name: String,
    pub scale: Option<u32>,  // numerical
    pub offset: Option<i32>, // numerical
    pub units: Option<String>,
}

impl FitFieldType {
    /// Called by `profile::get_messagetype()`
    pub fn new(ft: (u8, String, Option<u32>, Option<i32>, Option<String>)) -> Self {
        FitFieldType {
            field_def_no: ft.0,
            name: ft.1,
            scale: ft.2,
            offset: ft.3,
            units: ft.4,
        }
    }
}
