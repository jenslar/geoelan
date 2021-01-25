//! Various structs for parsed FIT data. Fields are named according to FIT SDK
#![allow(dead_code)]
use chrono::prelude::*;
use std::fs::File;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    errors::FitError,
    errors::{self, ParseError},
    parse_fit, process,
};

/// Currently not used for anything, relates to the optional u16 crc
/// Directly translated - possibly incorrectly - from FIT SDK documentation
fn fit_crc16(mut crc: u16, byte: u8) -> u16 {
    let crc_table: [u16; 16] = [
        0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800,
        0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
    ];
    // compute checksum of lower four bits of byte
    let tmp = crc_table[crc as usize & 0xF];
    crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ crc_table[byte as usize & 0xF];
    // now compute checksum of upper four bits of byte tmp = crc_table[crc & 0xF];
    crc = (crc >> 4) & 0x0FFF;
    crc = crc ^ tmp ^ crc_table[(byte >> 4) as usize & 0xF];

    crc
}

/// FIT Base Type
/// The data types that may occur in a FIT-file, see FIT SDK
#[derive(Debug, Clone)]
pub enum BaseType {
    STRING(String), // borrow issues
    BYTE(Vec<u8>),
    ENUM(Vec<u8>),
    UINT8(Vec<u8>),
    UINT8Z(Vec<u8>), // Z = ?
    SINT8(Vec<i8>),
    UINT16(Vec<u16>),
    UINT16Z(Vec<u16>), // Z = ?
    SINT16(Vec<i16>),
    UINT32(Vec<u32>),
    UINT32Z(Vec<u32>), // Z = ?
    SINT32(Vec<i32>),
    FLOAT32(Vec<f32>),
    UINT64(Vec<u64>),
    UINT64Z(Vec<u64>), // Z = ?
    SINT64(Vec<i64>),
    FLOAT64(Vec<f64>),
}

/// Unpack BaseTypes.
/// As a pre-caution every type has its own unack fn,
/// even when they are of the same primal type.
impl BaseType {
    pub fn get_enum(&self, global: u16, field_def: u8) -> Result<Vec<u8>, ParseError> {
        match self {
            BaseType::ENUM(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u8(&self, global: u16, field_def: u8) -> Result<Vec<u8>, ParseError> {
        match self {
            BaseType::UINT8(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u8z(&self, global: u16, field_def: u8) -> Result<Vec<u8>, ParseError> {
        match self {
            BaseType::UINT8Z(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_byte(&self, global: u16, field_def: u8) -> Result<Vec<u8>, ParseError> {
        match self {
            BaseType::BYTE(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_i8(&self, global: u16, field_def: u8) -> Result<Vec<i8>, ParseError> {
        match self {
            BaseType::SINT8(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_string(self, global: u16, field_def: u8) -> Result<String, ParseError> {
        // no borrow + deref for string... fix or ok?
        match self {
            BaseType::STRING(val) => Ok(val),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_i16(&self, global: u16, field_def: u8) -> Result<Vec<i16>, ParseError> {
        match self {
            BaseType::SINT16(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u16(&self, global: u16, field_def: u8) -> Result<Vec<u16>, ParseError> {
        match self {
            BaseType::UINT16(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u16z(&self, global: u16, field_def: u8) -> Result<Vec<u16>, ParseError> {
        match self {
            BaseType::UINT16Z(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_i32(&self, global: u16, field_def: u8) -> Result<Vec<i32>, ParseError> {
        match self {
            BaseType::SINT32(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u32(&self, global: u16, field_def: u8) -> Result<Vec<u32>, ParseError> {
        match self {
            BaseType::UINT32(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u32z(&self, global: u16, field_def: u8) -> Result<Vec<u32>, ParseError> {
        match self {
            BaseType::UINT32Z(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_f32(&self, global: u16, field_def: u8) -> Result<Vec<f32>, ParseError> {
        match self {
            BaseType::FLOAT32(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_i64(&self, global: u16, field_def: u8) -> Result<Vec<i64>, ParseError> {
        match self {
            BaseType::SINT64(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u64(&self, global: u16, field_def: u8) -> Result<Vec<u64>, ParseError> {
        match self {
            BaseType::UINT64(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_u64z(&self, global: u16, field_def: u8) -> Result<Vec<u64>, ParseError> {
        match self {
            BaseType::UINT64Z(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
    pub fn get_f64(&self, global: u16, field_def: u8) -> Result<Vec<f64>, ParseError> {
        match self {
            BaseType::FLOAT64(val) => Ok(val.to_owned()),
            _ => Err(ParseError::ErrorParsingField(global, field_def)),
        }
    }
}

/// Expected and actually read size in bytes, for error handling
#[derive(Debug, Copy, Clone)]
pub struct DataSize {
    pub expected: usize,
    pub read: usize,
}

/// The available sensor types as specified in FIT SDK
#[derive(Debug, Copy, Clone)]
pub enum ThreeDSensorType {
    Gyroscope,     // id: 164
    Accelerometer, // id: 165
    Magnetometer,  // id: 208
}

/// Used to specify transformation values for raw FIT-values to common variants
/// e.g. semicircle to centigrades for coordinates
// #[derive(Debug, Copy, Clone)]
// pub struct Transform {
//     message_type: u16, // Message Name, mesg_num, u16
//     field_definition_number: u8,// Field Def u8 -> Field Name String
//     scale: u32, // Scale
//     offset: Option<u32>, // Offset -> unwrap_or(1)
//     // Units Field Type    Array    Components
// }

// pub struct VirbSession {
//     uuid: Vec<String>
// }

/// FitFile struct
#[derive(Debug)]
pub struct FitFile {
    pub path: PathBuf,
}

impl FitFile {
    /// Creates new FitFile struct
    pub fn new(path: &PathBuf) -> FitFile {
        FitFile {
            path: path.to_owned(),
        }
    }
    /// Parse the FIT-file returning all messages in a FitData struct.
    /// For VIRB data it's also possible to filter a specific session via UUID.
    pub fn parse(
        &self,
        global_id: &Option<u16>,
        uuid: &Option<String>,
    ) -> Result<FitData, crate::errors::FitError> {
        parse_fit(&self.path, global_id, uuid, false, false, false)
    }

    /// Parse the FIT-file returning all messages in a FitData struct.
    /// "Debug" version of FitFile.parse().
    /// Will print data as it is being parsed. Filtering on UUID not possible.
    /// `unchecked_string` = true means BaseType::STRING will be parsed with std::str::from_utf8_unchecked()
    pub fn debug(&self, unchecked_string: bool) -> Result<FitData, crate::errors::FitError> {
        parse_fit(&self.path, &None, &None, false, true, unchecked_string)
    }

    /// FIT-file size
    pub fn len(&self) -> std::io::Result<u64> {
        Ok(File::open(&self.path)?.metadata()?.len())
    }

    // The following also exist as impl FitData.
    // FitFile method returns Result<>
    // FitData method returns Option<>

    /// VIRB only.
    /// Returns all unique uuids for VIRB action camera FIT-files.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no camera_event/161 can be parsed.
    pub fn uuid(&self, uuid_start: &Option<String>, force_partial_on_error: bool) -> Result<Vec<String>, errors::FitError> {
        let data = match force_partial_on_error {
            true => {
                match self.parse(&Some(161_u16), uuid_start) {
                    Ok(d) => d,
                    Err(e) => {
                        match e {
                            FitError::Fatal(_) => return Err(e),
                            FitError::Partial(_, d) => d // want partial data
                        }
                    }
                }
            },
            false => self.parse(&Some(161_u16), uuid_start)?
        };
        let cam = process::parse_cameraevent(&data)?;
        let mut uuids: Vec<String> = Vec::new();
        for evt in cam.into_iter() {
            uuids.push(evt.camera_file_uuid);
        }
        uuids.dedup();
        Ok(uuids)
    }

    /// VIRB only.
    /// Returns unique uuids for VIRB action camera FIT-files,
    /// grouped into sessions.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no camera_event/161 can be parsed.
    pub fn sessions(&self, force_partial_on_error: bool) -> Result<Vec<Vec<String>>, errors::FitError> {
        let data = match force_partial_on_error {
            true => {
                match self.parse(&Some(161_u16), &None) {
                    Ok(d) => d,
                    Err(e) => {
                        match e {
                            FitError::Fatal(_) => return Err(e),
                            FitError::Partial(_, d) => d // want partial data
                        }
                    }
                }
            },
            false => self.parse(&Some(161_u16), &None)?
        };
        let cam = process::parse_cameraevent(&data)?;
        let mut sessions: Vec<Vec<String>> = Vec::new();
        let mut session: Vec<String> = Vec::new();
        for evt in cam.into_iter() {
            if evt.camera_event_type == 6 {
                // also last in session, succeeds 2
                continue;
            }
            session.push(evt.camera_file_uuid);
            if evt.camera_event_type == 2 {
                session.dedup(); // enough to keep only unique items, since uuids logged in order
                sessions.push(session.to_owned());
                session.clear();
            }
        }
        Ok(sessions)
    }

    /// VIRB only. (?)
    /// Return the absolute timestamp for the start of the FIT-file
    /// Only valid if timestamp_correlation is logged.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no timestamp_correlation/162 can be parsed.
    pub fn t0(&self, offset: i64, force_partial_on_error: bool) -> std::result::Result<DateTime<Utc>, FitError> {
        let data = match force_partial_on_error {
            true => {
                match self.parse(&Some(162_u16), &None) {
                    Ok(d) => d,
                    Err(e) => {
                        match e {
                            FitError::Fatal(_) => return Err(e),
                            FitError::Partial(_, d) => d // want partial data
                        }
                    }
                }
            },
            false => self.parse(&Some(162_u16), &None)?
        };
        let tc = process::parse_timestampcorrelation(&data)?; // ...then fail here if no 162

        Ok(Utc.ymd(1989, 12, 31).and_hms_milli(0, 0, 0, 0)
        + chrono::Duration::hours(offset) // not encoded as proper timezone in output
        + chrono::Duration::seconds(
            tc.timestamp as i64 - tc.system_timestamp as i64)
        + chrono::Duration::milliseconds(
            tc.timestamp_ms as i64 - tc.system_timestamp_ms as i64))
    }

    /// VIRB only.
    /// Returns formatted gps_metadata/160.
    /// Some devices may have gps_metadata, but not necessarily all fields
    /// present in VIRB data.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no gps_metadata/160 can be parsed.
    pub fn gps(&self, uuid: &Option<String>, force_partial_on_error: bool) -> Result<Vec<GpsMetadata>, FitError> {
        let data = match force_partial_on_error {
            true => {
                match self.parse(&Some(160_u16), uuid) {
                    Ok(d) => d,
                    Err(e) => {
                        match e {
                            FitError::Fatal(_) => return Err(e),
                            FitError::Partial(_, d) => d // want partial data
                        }
                    }
                }
            },
            false => self.parse(&Some(160_u16), uuid)?
        };
        process::parse_gpsmetadata(&data)
    }

    /// VIRB only.
    /// Returns formatted camera_event/161.
    /// Some devices may have gps_metadata, but not necessarily all fields
    /// present in VIRB data.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no gps_metadata/160 can be parsed.
    pub fn cam(&self, uuid: &Option<String>, force_partial_on_error: bool) -> Result<Vec<CameraEvent>, FitError> {
        let data = match force_partial_on_error {
            true => {
                match self.parse(&Some(161_u16), uuid) {
                    Ok(d) => d,
                    Err(e) => {
                        match e {
                            FitError::Fatal(_) => return Err(e),
                            FitError::Partial(_, d) => d // want partial data
                        }
                    }
                }
            },
            false => self.parse(&Some(161_u16), uuid)?
        };
        process::parse_cameraevent(&data)
    }
}

#[derive(Debug, Clone)]
pub struct FitData {
    pub header: FitHeader,
    pub records: Vec<DataMessage>, // all records ordered as logged
                                   // pub crc: Option<u16> // check crc not implemented
}

impl FitData {
    /// Get specific message type by specifying FIT Global ID for full parses.
    /// Also possible to use FitFile.parse(GLOBAL_ID, UUID)
    /// See Profile.xslx in FIT SDK for message type descriptions and global id.
    pub fn filter(&self, global_id: u16) -> Vec<DataMessage> {
        self.records
            .clone() // better way?
            .into_iter()
            .filter(|m| m.global == global_id)
            .collect::<Vec<DataMessage>>()
    }

    /// Sort records according to message type into HashMap.
    /// Key is numerical FIT global id.
    pub fn group(&self) -> HashMap<u16, Vec<DataMessage>> {
        let mut sorted_records: HashMap<u16, Vec<DataMessage>> = HashMap::new();
        for msg in self.records.clone().into_iter() {
            // possible not to clone()?
            sorted_records
                .entry(msg.global)
                .or_insert(Vec::new())
                .push(msg);
        }
        sorted_records
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns total number of records
    pub fn len(&self) -> usize {
        self.records.len()
    }

    // The following also exist as impl FitFile:
    // - FitFile method returns Result<>
    // - FitData method returns Option<>

    /// VIRB only.
    /// Returns all unique uuids for VIRB action camera FIT-files
    /// Cannot filter on UUID and returns Option<T> instead of Result<T>,
    /// compared to FitFile method.
    pub fn uuid(&self) -> Option<Vec<String>> {
        let cam = process::parse_cameraevent(&self).ok()?;
        let mut uuids: Vec<String> = Vec::new();
        for evt in cam.into_iter() {
            uuids.push(evt.camera_file_uuid);
        }
        uuids.dedup();
        Some(uuids)
    }

    /// VIRB only.
    /// Returns unique uuids for VIRB action camera FIT-files,
    /// grouped into sessions
    /// Cannot filter on UUID and returns Option<T> instead of Result<T>,
    /// compared to FitFile method.
    pub fn sessions(&self) -> Option<Vec<Vec<String>>> {
        let cam = process::parse_cameraevent(&self).ok()?;
        let mut sessions: Vec<Vec<String>> = Vec::new();
        let mut session: Vec<String> = Vec::new();
        for evt in cam.into_iter() {
            if evt.camera_event_type == 6 {
                // also last in session, succeeds 2
                continue;
            }
            session.push(evt.camera_file_uuid);
            if evt.camera_event_type == 2 {
                session.dedup(); // enough to keep only unique items, since uuids logged in order
                sessions.push(session.to_owned());
                session.clear();
            }
        }
        Some(sessions)
    }

    /// VIRB only. (?)
    /// Return the absolute timestamp for the start of the FIT-file
    /// Only valid if timestamp_correlation is logged.
    /// Cannot filter on UUID and returns Option<T> instead of Result<T>,
    /// compared to FitFile method.
    pub fn t0(&self, offset: i64) -> Option<DateTime<Utc>> {
        let tc = process::parse_timestampcorrelation(&self).ok()?;

        Some(
            Utc.ymd(1989, 12, 31).and_hms_milli(0, 0, 0, 0)
        + chrono::Duration::hours(offset) // not encoded as proper timezone in output
        + chrono::Duration::seconds(
            tc.timestamp as i64 - tc.system_timestamp as i64)
        + chrono::Duration::milliseconds(
            tc.timestamp_ms as i64 - tc.system_timestamp_ms as i64),
        )
    }

    /// VIRB only.
    /// Returns formatted gps_metadata/160.
    /// Some devices may have gps_metadata, but not necessarily all fields
    /// present in VIRB data.
    /// Cannot filter on UUID and returns Option<T> instead of Result<T>,
    /// compared to FitFile method.
    pub fn gps(&self, _uuid: &Option<String>) -> Option<Vec<GpsMetadata>> {
        process::parse_gpsmetadata(&self).ok()
    }

    /// VIRB only.
    /// Returns formatted gps_metadata/160.
    /// Some devices may have gps_metadata, but not necessarily all fields
    /// present in VIRB data.
    /// Cannot filter on UUID and returns Option<T> instead of Result<T>,
    /// compared to FitFile method.
    pub fn cam(&self) -> Option<Vec<CameraEvent>> {
        process::parse_cameraevent(&self).ok()
    }
}

/// FIT-file header, 12 or 14 bytes
#[derive(Debug, Copy, Clone)]
pub struct FitHeader {
    /// Byte 0: size of header
    pub headersize: u8,
    /// Byte 1: Protocol
    pub protocol: u8,
    /// Bytes 2-3: Profile, Little Endian
    pub profile: u16,
    /// Bytes 4-7: Size of FIT data succeeding header, Little Endian
    pub datasize: u32,
    /// Bytes 8-11: Ascii for .FIT
    pub dotfit: [char; 4],
    /// Bytes 12, 13: CRC, optional. CRC check not yet implemented.
    /// If present in the header, the final two bytes (u16) of the file contain the CRC.
    pub crc: Option<u16>,
}

/// FIT definition message
/// Specifies the structure of data structure it precedes, can be overwritten
/// DefintionField is used for both predefined data in Profile.xslx and developer data
#[derive(Debug)]
pub struct DefinitionField {
    /// Byte 0: Defined in the Global FIT profile for the specified FIT message
    pub field_definition_number: u8,
    /// byte 1, Size (in bytes) of the specified FIT message’s field
    pub size: u8,
    /// byte 2, Base type of the specified FIT message’s field
    pub base_type: u8,
    /// Field description
    /// See Profile.xsls for predefined data or field_description message (global 206) for developer data
    pub field_name: String,
    /// Currently only for developer data defintions
    /// Extracted from global id 206/field description
    pub units: Option<String>,
    pub scale: Option<u8>,
    pub offset: Option<i8>,
}

/// FIT definition field for developer data
#[derive(Debug, Copy, Clone)]
pub struct DeveloperField {
    /// Byte 0: Maps to the field_definition_number of a field_description Message
    pub field_number: u8,
    /// Byte 1: Size (in bytes) of the specified FIT message’s field
    pub size: u8,
    /// Byte 2: Maps to the developer_data_index of a developer_data_id in a field_description data message (global 206)
    pub developer_data_index: u8,
}

/// Field Description Message, global id 206
/// Describes the structure for custom data
#[derive(Debug, Clone)]
pub struct FieldDescriptionMessage {
    // Required? Specified in SDK
    pub developer_data_index: u8, // id: 0, uint8 1 byte, Index of the developer that this message maps to
    pub field_definition_number: u8, // id: 1, uint8 1 byte Field Number that maps to this message
    pub fit_base_type_id: u8,     // id: 2 Base type of the field
    pub field_name: String,       // id: 3, 64 bytes (up to? 0-padded?)
    // Optional? Not in Wahoo Rival Fit
    // pub units: String,            // id: 8, 16 bytes (up to?), Units associated with the field
    pub units: Option<String>, // id: 8, 16 bytes (up to?), Units associated with the field
    // Optional?
    pub array: Option<u8>,             // id: 4  uint8
    pub components: Option<String>,    // id: 5  string
    pub scale: Option<u8>,             // id: 6  uint8
    pub offset: Option<i8>,            // id: 7  sint8
    pub bits: Option<String>,          // id: 9  string
    pub accumulate: Option<String>,    // id: 10  string
    pub fit_base_unit_id: Option<u16>, // id: 13  fit_base_unit (weight only?) 0,1,2
    pub native_mesg_num: Option<u16>,  // id: 14  mesg_num (aka global id)
    pub native_field_num: Option<u8>,  // id: 15, Equivalent native field number
}

/// FIT definition message, describes the structure
/// for the corresponding FIT message type stored in DataMessage.
/// A size (u8) of 255 bytes means it is invalid.
#[derive(Debug)]
pub struct DefinitionMessage {
    pub header: u8,                              // byte 0
    pub reserved: u8,                            // byte 1
    pub architecture: u8,                        // byte 2: 0=LE, 1=BE
    pub global: u16,                             // bytes 3-4
    pub definition_fields: Vec<DefinitionField>, // definition fields (3 bytes/each)
    pub developer_fields: Vec<DefinitionField>,  // developer data, optional
    pub data_message_length: usize, // CHANGE FROM usize TO u32? total message length incl dev
}

/// FIT data field in a FIT DataMessage
#[derive(Debug, Clone)]
pub struct DataField {
    pub field_definition_number: u8,
    pub description: String, // remove? or Option for dev data? only retreive when needed?
    pub units: Option<String>, // currently only for dev data
    pub data: BaseType,      // 201127 now BaseType(Vec<T>), instead of Vec<Basetype>
}
/// FIT data message
#[derive(Debug, Clone)]
pub struct DataMessage {
    pub header: u8, // 1 byte header
    pub global: u16,
    pub description: String,
    pub fields: Vec<DataField>,     // "normal" fit data
    pub dev_fields: Vec<DataField>, // developer data "converted" to DataField via field_description/206
}

/// Parsed `timestamp_correlation` data message, global id 162
/// Important: presumably loggaed at satellite sync,
/// but does NOT always precede the first gps_metadata (160) message
#[derive(Debug, Copy, Clone)]
pub struct TimestampCorrelation {
    pub timestamp: u32,    // seconds
    pub timestamp_ms: u16, // milliseconds
    pub system_timestamp: u32,
    pub system_timestamp_ms: u16,
}

/// Parsed `camera_event` data message, global id 161
/// Contains UUID for the corresponding clip
#[derive(Debug)]
pub struct CameraEvent {
    pub timestamp: u32,           // 253: seconds
    pub timestamp_ms: u16,        // 0: milliseconds
    pub camera_file_uuid: String, // 2: camera_file_uuid
    pub camera_event_type: u8,    // 1: camera_event_type
    pub camera_orientation: u8,   // 3: camera_orientation
}

/// Parsed `gps_metadata` data message, global id 160
/// 10Hz GPS log for Garmin VIRB Ultra 30
/// Note: Garmin VIRB Ultra 30 logs additional data not documented in the FIT SDK, ignored here.
///       Run `geoelan check -f FITFILE -g 160 --verbose` to view these
#[derive(Debug)]
pub struct GpsMetadata {
    pub timestamp: u32,     // id:253, seconds
    pub timestamp_ms: u16,  // id:0, milliseconds
    pub latitude: i32,      // id:1
    pub longitude: i32,     // id:2
    pub altitude: u32,      // id:3
    pub speed: u32,         // id:4
    pub heading: u16,       // id:5
    pub utc_timestamp: u32, // id:6
    pub velocity: Vec<i16>, // id:7 Vec::with_capacity(3), x, y, z velocity values, was [i16;3]
                            // pub unknown: [u16;5] // id:8-12 not in Profile.xlsx, exists in definition message
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub time: crate::Duration,
    pub latitude: f64,  // id:1
    pub longitude: f64, // id:2
    pub altitude: f32,  // id:3
    pub speed: f32,     // id:4
    pub heading: f32,   // id:5
}

impl GpsMetadata {
    /// Convert gps_metadata basetype values to decimal degrees etc
    pub fn point(&self) -> Point {
        Point {
            time: {
                chrono::Duration::seconds(self.timestamp as i64)
                    + chrono::Duration::milliseconds(self.timestamp_ms as i64)
            },
            latitude: (self.latitude as f64) * (180.0 / 2.0_f64.powi(31)),
            longitude: (self.longitude as f64) * (180.0 / 2.0_f64.powi(31)),
            altitude: (self.altitude as f32 / 5.0) - 500.0,
            speed: self.speed as f32 / 1000.0,
            heading: self.heading as f32 / 100.0, // scale 100
        }
    }
}

/// Parsed `record` data message, global id 20
/// Partial spatial data is logged here, less frequently than gps_metadata.
// Based on fields in FIT SDK + Wahoo Elemnt Bolt
#[derive(Debug)]
pub struct Record {
    pub timestamp: u32,           // id:253, seconds
    pub timestamp_ms: u16,        // id:0, milliseconds
    pub latitude: i32,            // id:0
    pub longitude: i32,           // id:1
    pub altitude: u16,            // id:2
    pub distance: u32,            // id: 5
    pub speed: u16,               // id:6
    pub grade: i16,               // id: 9, in wahoo bolt
    pub velocity: Vec<i16>, // used Vec::with_capacity(3) id:7 x, y, z velocity values, use vector sum [NOTE: WAS u16]
    pub temperature: Option<i8>, // id: 13, in wahoo bolt
    pub gps_accuracy: Option<u8>, // id: 31, in wahoo bolt fit
}

/// Parsed 3D sensor data message
/// gyrometer_data, global id = 164
/// accelerometer_data, global id = 165
/// magnetometer_data, global id = 208
#[derive(Debug)]
pub struct ThreeDSensorData {
    pub timestamp: u32,               // id:253, seconds
    pub timestamp_ms: u16,            // id:0, milliseconds
    pub sample_time_offset: Vec<u16>, // id:1
    pub x: Vec<u16>,                  // id:2 gyro_x, acc_x, mag_x
    pub y: Vec<u16>,                  // id:3 gyro_y, acc_y, mag_y
    pub z: Vec<u16>,                  // id:4 gyro_z, acc_z, mag_z
    // values below calculated post-extraction via corresponding three_d_sensor_calibration messages
    pub calibrated_x: Vec<f64>, // id:5 calibrated_gyro_x, calibrated_acc_x, calibrated_mag_x
    pub calibrated_y: Vec<f64>, // id:6 calibrated_gyro_y, calibrated_acc_y, calibrated_mag_y
    pub calibrated_z: Vec<f64>, // id:7 calibrated_gyro_z, calibrated_acc_z, calibrated_mag_z
}

/// Parsed `three_d_sensor_calibration` message, global id 167
/// Contains calibration values for global id 164, 165, 208
/// Accelerometer/165 = sensor type 0
/// Ayrometer/164 = sensor type 1
/// Magnetometer/208 = sensor type 2 (compass)
#[derive(Debug)]
pub struct ThreeDSensorCalibration {
    pub timestamp: u32,               // id:253, seconds
    pub sensor_type: u8,              // id:0, enum
    pub calibration_factor: u32,      // id:1
    pub calibration_divisor: u32,     // id:2
    pub level_shift: u32,             // id:3
    pub offset_cal: Vec<i32>,         // using Vec::with_capacity(3) id:4 [3]
    pub orientation_matrix: Vec<i32>, // using Vec::with_capacity(9)  id:5 3x3 matrix [9]
}

// Metadata listing session uuid:s, FIT-file, start, end etc, TIL: doc comment for outcommented code causes syntax error
// #[derive(Debug)]
// pub struct FitMetadata {
//     pub uuid: Vec<String>,
//     pub sha256: String, // FIT file sha256 hash, hex string, or use Vec<u8>?
//     pub fitfile: String, // FIT filename/basename
//     pub fitsize: u64, // FIT filesize
//     pub start: String, // absolute timestamp as string: "%Y-%m-%dT%H:%M:%S%.3f"
//     pub end: String // absolute timestamp as string: "%Y-%m-%dT%H:%M:%S%.3f"
// }
