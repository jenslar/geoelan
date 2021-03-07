//! Various structs and methods FIT-files and parsed FIT data.
//! Where possible, fields are named according to FIT SDK Profiles.xlsx.
#![allow(dead_code)]
use chrono::prelude::*;
use rayon::prelude::*;
use std::fmt;
use std::fs::File;
use std::{collections::HashMap, path::Path, path::PathBuf};

use crate::{
    errors::{FitError, ParseError},
    parse_fit, process,
};

/// FIT Base Type.
/// The data types that may occur in a FIT-file, see FIT SDK.
/// Non-String values are returned as `Enum(Vec<T>)` for performance
/// and code verbosity reasons.
#[derive(Debug, Clone)]
pub enum BaseType {
    STRING(String),
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
/// As a pre-caution every type has its own unpack fn,
/// even when they are of the same primal type.
/// Possibly not a good implementation, but allows for
/// tracking down specific errors.
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
    pub fn get_string(&self, global: u16, field_def: u8) -> Result<String, ParseError> {
        match self {
            BaseType::STRING(val) => Ok(val.into()),
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

/// Expected and actually read size in bytes. For error handling.
#[derive(Debug, Copy, Clone)]
pub struct DataSize {
    pub expected: usize,
    pub read: usize,
}

/// The available 3D sensor types as specified in FIT SDK
#[derive(Debug, Copy, Clone)]
pub enum ThreeDSensorType {
    /// FIT global ID: 164
    Gyroscope,
    /// FIT global ID: 165
    Accelerometer,
    /// FIT global ID: 208
    Magnetometer,
}

/// Used in FitFile as a flag to indicate
/// whether all records were parsed or not.
#[derive(Debug, Clone, Copy)]
pub enum ParseMethod {
    /// Set if `FitFile::parse()` was used.
    /// `FitFile.records` contains all/partial records, depending on errors.
    Full,
    /// Set if `FitFile::parse_filter()` was used.
    /// `FitFile.records` contains only records with specified global id.
    Filter(u16),
    /// Set if `FitFile::debug()` was used.
    /// `FitFile.records` contains all/partial records. If an error is
    /// raised, only `FitError::Fatal(err)` will be returned as `Err(err)`.
    /// `FitError::Partial(err, data)` returns extracted data only.
    Debug,
}

/// FitFile struct
#[derive(Debug)]
pub struct FitFile {
    pub path: PathBuf,
    pub header: FitHeader,
    pub records: Vec<DataMessage>, // all records ordered as logged
    pub crc: Option<u16>,
    pub parse: ParseMethod,
}

impl FitFile {
    /// Parses the FIT-file in full and returns a FitFile struct, returning
    /// a FitError for most errors.
    /// `partial_return_on_error` returns partially extracted
    /// data up until an error was raised, discarding the
    /// error. Fatal errors (i.e. not able to parse the FIT-file
    /// at all) are still returned.
    /// Sets `FitFile.parse` to `ParseMethod::Full`
    pub fn parse(path: &Path, partial_return_on_error: bool) -> Result<FitFile, FitError> {
        if partial_return_on_error {
            match parse_fit(path, None, false, false) {
                Ok(d) => Ok(d),
                Err(FitError::Partial(_, d)) => Ok(d),
                Err(e @ FitError::Fatal(_)) => return Err(e),
            }
        } else {
            parse_fit(path, None, false, false)
        }
    }

    /// Similar to `FitFile::parse()`, but only returns FitFile
    /// records with the specified `global_id`. Developer data
    /// will be discarded. If further filtering is required
    /// use `parse()`. `parse_filter()` is intended
    /// for faster repeated parsing of a specific message type.
    /// Sets `FitFile.parse` to `ParseMethod::Filter(global_id)`
    pub fn parse_filter(
        path: &Path,
        global_id: u16,
        partial_return_on_error: bool,
    ) -> Result<FitFile, FitError> {
        if partial_return_on_error {
            match parse_fit(path, Some(&global_id), false, false) {
                Ok(d) => Ok(d),
                Err(FitError::Partial(_, d)) => Ok(d),
                Err(e @ FitError::Fatal(_)) => return Err(e),
            }
        } else {
            parse_fit(path, Some(&global_id), false, false)
        }
    }

    /// Parse the FIT-file returning all messages in a FitData struct.
    /// Will print data as it is being parsed. Filtering on UUID not possible.
    /// `unchecked_string` = true means BaseType::STRING will be parsed
    /// with `unsafe {std::str::from_utf8_unchecked()}`.
    /// Sets `FitFile.parse` to `ParseMethod::Debug`
    pub fn debug(path: &Path, unchecked_string: bool) -> Result<FitFile, FitError> {
        match parse_fit(path, None, true, unchecked_string) {
            Ok(d) => Ok(d),
            Err(FitError::Partial(e, d)) => {
                println!("Aborted at error: {}", e);
                Ok(d)
            }
            Err(e @ FitError::Fatal(_)) => Err(e),
        }
    }

    /// Filters FIT records on FIT Global ID
    pub fn filter(&self, global_id: u16) -> Vec<DataMessage> {
        self.records
            .par_iter()
            .filter(|m| m.global == global_id)
            .map(|v| v.to_owned())
            .collect::<Vec<DataMessage>>()
    }

    /// Filters FIT records on FIT Global ID, but also return
    /// their indeces in the original Vec<DataMessage>
    pub fn index_filter(&self, global_id: u16) -> Vec<(usize, DataMessage)> {
        self.records
            .par_iter()
            .enumerate()
            .filter(|(_, m)| m.global == global_id)
            .map(|(i, v)| (i, v.to_owned()))
            .collect()
    }

    /// Garmin VIRB only.
    /// Filters records on specific recording session by specifying its first UUID.
    /// Optionally also filters on FIT global ID to get e.g. gps_metadata/160
    /// for a specific recording session.
    pub fn filter_session(&self, uuid_start: &str, global_id: Option<u16>) -> Vec<DataMessage> {
        // Find boundary indeces for session by determining first/last camera_event
        // for the session
        // println!("session, uuid: {}", uuid_start);
        let mut uuid_found = false;
        let mut idx1: Option<usize> = None; // start boundary
        let mut idx2: Option<usize> = None; // end boundary
        let indexed_cam = self.index_filter(161);
        for (i, cam) in indexed_cam.iter() {
            // must be executed in order
            for field in cam.fields.iter() {
                match field.field_definition_number {
                    1 => {
                        if let BaseType::ENUM(s) = &field.data {
                            if uuid_found {
                                if idx1.is_none() && s[0] == 0 {
                                    // camera_event 0 = recording session start
                                    idx1 = Some(*i);
                                }
                                if idx1.is_some() && idx2.is_none() && s[0] == 2 {
                                    // camera_event 2 = recording session end
                                    idx2 = Some(*i + 1);
                                    break;
                                }
                            }
                        }
                    }
                    2 => {
                        if let BaseType::STRING(s) = &field.data {
                            if uuid_start == s && !uuid_found {
                                uuid_found = true;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        // Uncomment to check if sliced correctly
        // println!("idx1: {:?}, idx2: {:?}", idx1, idx2);

        match (idx1, idx2) {
            (Some(i1), Some(i2)) => match global_id {
                Some(g) => self.records[i1..i2]
                    .par_iter()
                    .filter(|r| r.global == g)
                    .map(|v| v.to_owned())
                    .collect(),
                None => self.records[i1..i2].to_owned(),
            },
            (_, _) => Vec::new(), // if any idx hasn't been set the filter is incorrect
        }
    }

    /// Group FitFile.records into message types.
    /// Key is FIT global ID.
    pub fn group(&self) -> HashMap<u16, Vec<DataMessage>> {
        let mut grouped_records: HashMap<u16, Vec<DataMessage>> = HashMap::new();
        self.records
            .iter() // par_iter() not possible due to borrow in closure
            .for_each(|r| {
                grouped_records
                    .entry(r.global)
                    .or_insert(Vec::new())
                    .push(r.to_owned())
            });
        grouped_records
    }

    /// FIT-file size
    pub fn file_size(&self) -> std::io::Result<u64> {
        Ok(File::open(&self.path)?.metadata()?.len())
    }

    /// Total number of records
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns true if FitFile contains no records.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn uuid(&self) -> Result<Vec<String>, FitError> {
        // Returning error to see if/what field was not assigned
        let cam = process::parse_cameraevent(&self, None)?;
        let mut uuids: Vec<String> = cam
            .par_iter()
            .map(|evt| evt.camera_file_uuid.to_owned())
            .collect();
        uuids.dedup(); // duplicate uuids are grouped together
        Ok(uuids)
    }

    /// VIRB only.
    /// Derives start time of FIT-file via timestamp_correlation/162
    /// with added time offset in hours as DateTime object.
    pub fn t0(&self, offset: i64) -> Result<DateTime<Utc>, FitError> {
        let tc = process::parse_timestampcorrelation(&self)?;

        Ok(
            Utc.ymd(1989, 12, 31).and_hms_milli(0, 0, 0, 0) // FIT start time
        + chrono::Duration::hours(offset) // NOTE: means offset is not encoded as proper timezone
        + chrono::Duration::seconds(
            tc.timestamp as i64 - tc.system_timestamp as i64)
        + chrono::Duration::milliseconds(
            tc.timestamp_ms as i64 - tc.system_timestamp_ms as i64),
        )
    }

    /// VIRB only.
    /// Returns unique uuids for VIRB action camera FIT-files,
    /// grouped into sessions.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no camera_event/161 can be parsed.
    pub fn sessions(&self) -> Result<Vec<Vec<String>>, FitError> {
        let cam = process::parse_cameraevent(&self, None)?;
        let mut sessions: Vec<Vec<String>> = Vec::new();
        let mut session: Vec<String> = Vec::new();
        for evt in cam.iter() {
            if evt.camera_event_type == 6 {
                // also last in session, succeeds 2
                continue;
            }
            session.push(evt.camera_file_uuid.to_owned());
            if evt.camera_event_type == 2 {
                session.dedup(); // uuids logged in order
                sessions.push(session.to_owned());
                session.clear();
            }
        }
        Ok(sessions)
    }

    /// VIRB only.
    /// Returns formatted gps_metadata/160.
    /// Some devices may have gps_metadata, but not necessarily all fields
    /// present in VIRB data.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no gps_metadata/160 can be parsed.
    pub fn gps(&self, uuid: Option<&String>) -> Result<Vec<GpsMetadata>, FitError> {
        process::parse_gpsmetadata(&self, uuid)
    }

    /// VIRB only.
    /// Returns formatted camera_event/161.
    /// `force_partial_on_error` will forward partial data,
    /// but return ParseError::NoDataForMessageType
    /// if no gps_metadata/160 can be parsed.
    pub fn cam(&self, uuid: Option<&String>) -> Result<Vec<CameraEvent>, FitError> {
        process::parse_cameraevent(&self, uuid)
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

// impl fmt::Display for FitHeader {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "headersize: {:4}\nprotocol:   {:4}\nprofile:    {:4} {:?}",
//             self.headersize,
//             self.protocol,
//             self.units.as_ref().map_or("N/A", |u| u),
//             self.data, // not the prettiest for fields with large arrays, i.e 3d sensor data
//         )
//     }
// }

/// FIT definition message
/// Specifies the structure of data structure it precedes, can be overwritten
/// DefintionField is used for both predefined data in Profile.xslx and developer data
#[derive(Debug)]
pub struct DefinitionField {
    /// Byte 0: Defined in the Global FIT profile for the specified FIT message
    pub field_definition_number: u8,
    /// Byte 1, Size (in bytes) of the specified FIT message’s field
    pub size: u8,
    /// Byte 2, Base type of the specified FIT message’s field
    pub base_type: u8,
    pub field_name: String,
    // Below is FieldDescription content
    // See Profile.xsls for predefined data or field_description message (global 206) for developer data
    /// `units`, extracted from field_description/206
    pub units: Option<String>,
    /// `scale`, extracted from field_description/206
    pub scale: Option<u8>,
    /// `offset`, extracted from field_description/206
    pub offset: Option<i8>,
}

/// FIT definition field for developer data
#[derive(Debug, Copy, Clone)]
pub struct DeveloperField {
    /// Byte 0: Maps to the field_definition_number of a field_description Message
    pub field_number: u8,
    /// Byte 1: Size (in bytes) of the specified FIT message’s field
    pub size: u8,
    /// Byte 2: Maps to the developer_data_index of a developer_data_id in a field_description/206 data message
    pub developer_data_index: u8,
}

/// Field Description Message, global id 206
/// Describes the structure for custom data
#[derive(Debug, Clone)]
pub struct FieldDescriptionMessage {
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

impl fmt::Display for DataField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:4} {:22} {:20} {:?}",
            self.field_definition_number,
            self.description,
            self.units.as_ref().map_or("N/A", |u| u),
            self.data, // not the prettiest for fields with large arrays, i.e 3d sensor data
        )
    }
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

impl fmt::Display for DataMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Global ID: {0} | Message type: {1} | Header: {2:?}/{2:#010b}",
            self.global, self.description, self.header,
        )?;
        for fld in self.fields.iter() {
            writeln!(f, "    {}", fld)?
        }
        for fld in self.dev_fields.iter() {
            writeln!(f, "DEV {}", fld)?
        }
        Ok(())
    }
}

/// FIT Message Type, from Messages sheet in Profile.xlsx
pub struct FitMessageType {
    pub name: String,   // first column in Profile.xlsx Messages
    pub global_id: u16, // mesg_num in Profile.xlsx Types
    pub field_types: HashMap<u8, FitMessageFieldType>, // k: field_def_no
}

// impl FitMessageTypes {
//     fn new(global_id: u16) -> FitMessageTypes {
//         // crate::messages2
//     }
// }

/// FIT Message Field Type, from Messages sheet in Profile.xlsx
pub struct FitMessageFieldType {
    pub field_def_no: u8, // numerical, some are unfortunately strings in Profile.xlsx
    pub field_name: String,
    pub field_type: String,
    pub array: Option<String>, // e.g. [N] or [3]
    pub components: Option<String>,
    pub scale: Option<String>,  // numerical
    pub offset: Option<String>, // numerical
    pub units: Option<String>,
    pub bits: Option<String>,       // numerical
    pub accumulate: Option<String>, // comma separated numericals
    pub ref_field_name: Option<String>,
    pub ref_field_value: Option<String>,
}

/// VIRB only.
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

/// VIRB only.
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

/// VIRB only. (message type may exist on other devices, but not all fields)
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

/// For converting gps_metadata/161 to decimal values.
#[derive(Debug, Clone)]
pub struct Point {
    pub latitude: f64,  // id:1
    pub longitude: f64, // id:2
    pub altitude: f64,  // id:3 f32?
    pub speed: f64,     // id:4 f32?
    // pub velocity: f64,     // id:4
    pub heading: f64, // id:5 f32?
    pub time: crate::Duration,
    pub text: Option<String>,
}

impl GpsMetadata {
    /// Convert gps_metadata basetype values to decimal degrees etc
    pub fn to_point(&self) -> Point {
        Point {
            latitude: (self.latitude as f64) * (180.0 / 2.0_f64.powi(31)),
            longitude: (self.longitude as f64) * (180.0 / 2.0_f64.powi(31)),
            altitude: (self.altitude as f64 / 5.0) - 500.0,
            speed: self.speed as f64 / 1000.0,
            heading: self.heading as f64 / 100.0, // scale 100
            time: {
                chrono::Duration::seconds(self.timestamp as i64)
                    + chrono::Duration::milliseconds(self.timestamp_ms as i64)
            },
            text: None,
        }
    }
}

/// Parsed `record` data message, global id 20. Developer data will be discarded.
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
    pub temperature: Option<i8>, // id: 13, in wahoo bolt, IGNORE?
    pub gps_accuracy: Option<u8>, // id: 31, in wahoo bolt fit
}

/// VIRB only. (?)
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

/// VIRB only. (?)
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
