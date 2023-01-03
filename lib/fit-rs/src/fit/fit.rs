//! Core FIT functionality.
//!
//! ```rs
//! use crate::fit_rs::Fit;
//! use std::path::Path;
//! 
//! fn main -> std::io::Result<()> {
//!     // Parse a FIT-file.
//!     let fit_path = Path::new("FITFILE.fit");
//!     let fit = Fit::new(&fit_path)?;
//! 
//!     for record in fit.records.iter() {
//!         println!("{record:?}");
//!     }
//! 
//!     // Extract UUID from Garmin VIRB action camera.
//!     let mp4_path = PathBuf::from("VIRB_MP4FILE.MP4");
//!     let uuid = Fit::uuid_mp4(&mp4_path)?;
//!     println!("{uuid}");
//! 
//!     Ok(())
//! }
//! ```

use std::{path::{PathBuf, Path}, io::Cursor, collections::HashMap, ops::Range};

use binread::{BinReaderExt, BinRead};
use rayon::{iter::{IntoParallelRefIterator, ParallelIterator}, prelude::IntoParallelRefMutIterator};
use time::{PrimitiveDateTime, Date, Month};

use crate::{
    errors::FitError,
    types::{
        FieldDescriptionMessage,
        FitPoint,
        SensorType,
    },
    CameraEvent,
    GpsMetadata,
    SensorData,
    TimestampCorrelation,
    Record,
    FitSession,
    profile::message_type::FitMessageType
};
use super::{
    fit_header::FitHeader,
    data_message::DataMessage,
    definition_message::DefinitionMessage,
    message_header::{MessageHeader, MessageType}
};

/// Fit core data struct, containing parsed FIT data, header etc.
#[derive(Debug, Clone, Default)]
pub struct Fit {
    /// Path to parsed FIT-file
    pub path: PathBuf,
    /// The header, containing data size etc
    pub header: FitHeader,
    /// The actual data in logging/chronological order
    pub records: Vec<DataMessage>,
    pub index: HashMap<String, Range<usize>> // optionally populated post-parse
}

impl Fit {
    /// Parse FIT-data in full.
    pub fn new(path: &Path) -> Result<Self, FitError> {
        Self::parse(path, None)
    }

    /// Parse FIT-data. Optionally filter on specified `global` ID 
    /// while parsing, which speeds up reads considerably if only
    /// a single data type is of interest. Developer data
    /// is not supported when filtering.
    pub fn parse(path: &Path, global: Option<u16>) -> Result<Self, FitError> {

        // TODO 220812 REGRESSION CHECK on 20mb virb fit old code 60ms faster on work laptop
        
        let mut cursor = Self::cursor(path)?;
        let len = cursor.get_ref().len();

        let fitheader = FitHeader::new(&mut cursor)?;

        let data_size = fitheader.data_size(len) as u64;

        // Simple incremental index for data messages,
        // that can be used to sort in e.g. chronological order,
        // even after filtering on type
        let mut data_index = 0;
        
        let mut definitions: HashMap<u8, DefinitionMessage> = HashMap::new();
        let mut data_messages: Vec<DataMessage> = Vec::new();
        let mut field_descriptions: HashMap<(u8, u8), FieldDescriptionMessage> = HashMap::new();

        while cursor.position() < data_size {

            let header: MessageHeader = cursor.read_ne()?;
            let id = header.id();

            match header.kind() {

                MessageType::Definition => {

                    let definition = DefinitionMessage::new(
                        &mut cursor,
                        &header,
                        &field_descriptions
                    )?;
                    
                    definitions.insert(
                        id,
                        definition 
                    );
                },

                MessageType::Data => {

                    let definition = definitions.get(&id).ok_or_else(||
                        FitError::UnknownDefinition {local: id, offset: cursor.position()}
                    )?;

                    if let Some(g) = global {
                        if definition.global != g {
                            // If message does not have correct global ID
                            // derive message length and set position
                            // to next message
                            let pos = cursor.position();
                            cursor.set_position(pos + definition.data_size() as u64);
                            continue;
                        }
                    }

                    let data_message = DataMessage::new(
                        &mut cursor,
                        &definition,
                        data_index
                    )?;

                    // Parse and store custom developer definitions
                    if data_message.global == 206 {
                        let field_descr = FieldDescriptionMessage::new(&data_message)?;
                        // Require both field_definition_number and developer_data_index
                        // to create a unique key since third parties are not
                        // always using this correctly, sometimes causing ID collisions
                        field_descriptions.insert(
                            (field_descr.field_definition_number, field_descr.developer_data_index),
                            field_descr,
                        );
                    }

                    data_messages.push(data_message);

                    data_index += 1; // data message index
                }
            }
        }

        Ok(Fit{
            path: path.to_owned(),
            header: fitheader,
            records: data_messages,
            index: HashMap::new()
        })
    }

    /// Debug FIT-data. Optionally filter on specified `global` ID 
    /// while parsing, which speeds up reads considerably if only
    /// a single data type is of interest. Developer data
    /// is not supported when filtering.
    pub fn debug(path: &Path, global: Option<u16>) -> Result<Self, FitError> {

        // TODO 220812 REGRESSION CHECK on 20mb virb fit old code 60ms faster on work laptop
        
        let mut cursor = Self::cursor(path)?;
        let len = cursor.get_ref().len();

        let fitheader = FitHeader::new(&mut cursor)?;

        println!("{fitheader:#?}");

        let data_size = fitheader.data_size(len) as u64;

        // Simple incremental index for data messages,
        // that can be used to sort in e.g. chronological order,
        // even after filtering on type
        let mut data_index = 0;
        
        let mut definitions: HashMap<u8, DefinitionMessage> = HashMap::new();
        let mut data_messages: Vec<DataMessage> = Vec::new();
        let mut field_descriptions: HashMap<(u8, u8), FieldDescriptionMessage> = HashMap::new();

        while cursor.position() < data_size {

            // println!("FIELD DESCRIPTIONS:\n{field_descriptions:#?}");

            print!("OFFSET {} | ", cursor.position());

            let header: MessageHeader = cursor.read_ne()?;
            let id = header.id();

            match header.kind() {

                MessageType::Definition => {

                    let definition = DefinitionMessage::new(
                        &mut cursor,
                        &header,
                        &field_descriptions
                    )?;

                    println!("{definition:#?}");
                    
                    definitions.insert(
                        id,
                        definition 
                    );
                },

                MessageType::Data => {

                    let definition = definitions.get(&id).ok_or_else(||
                        FitError::UnknownDefinition {local: id, offset: cursor.position()}
                    )?;

                    if let Some(g) = global {
                        if definition.global != g {
                            // If message does not have correct global ID
                            // derive message length and set position
                            // to next message
                            let pos = cursor.position();
                            cursor.set_position(pos + definition.data_size() as u64);
                            continue;
                        }
                    }

                    let data_message = DataMessage::new(
                        &mut cursor,
                        &definition,
                        data_index
                    )?;

                    println!("{data_message:#?}");

                    // Parse and store custom developer definitions
                    if data_message.global == 206 {
                        let field_descr = FieldDescriptionMessage::new(&data_message)?;
                        // Require both field_definition_number and developer_data_index
                        // to create a unique key since third parties are not
                        // always using this correctly, sometimes causing ID collisions
                        field_descriptions.insert(
                            (field_descr.field_definition_number, field_descr.developer_data_index),
                            field_descr,
                        );
                    }

                    data_messages.push(data_message);

                    data_index += 1; // data message index
                }
            }
        }

        Ok(Fit{
            path: path.to_owned(),
            header: fitheader,
            records: data_messages,
            index: HashMap::new()
        })
    }

    fn cursor(path: &Path) -> std::io::Result<Cursor<Vec<u8>>> {
        let bytes = std::fs::read(&path)?;
        Ok(Cursor::new(bytes))
    }

    /// Read single FIT value from cursor with correct endianess
    /// determined via FIT data field `architecture` value.
    /// 
    /// Currently only used to read a `u16` for `DefinitionMessage`.
    pub(crate) fn read<T: Sized + BinRead>(cursor: &mut Cursor<Vec<u8>>, arch: u8) -> Result<T, FitError> {
        match arch {
            // Little Endian
            0 => cursor.read_le::<T>().map_err(|err| FitError::BinReadError(err)),
            // Big Endian
            1 => cursor.read_be::<T>().map_err(|err| FitError::BinReadError(err)),
            // Invalid architecture value
            _ => Err(FitError::InvalidArchitecture{arch, pos: cursor.position()})
        }
    }

    /// Returns `true` if bit at `position` is set. For checking FIT message headers.
    /// Panics if `position` is not a value between 0 and 7 (inclusive).
    /// 
    /// FIT 1 byte message header:
    /// ```
    /// Bit idx    7 6 5 4 3 2 1 0
    /// Header   | x x x x x x x x |
    ///            | | |
    ///            | | ╰- 1 = contains custom developer field definitions
    ///            | ╰--- 0 = Data, 1 = Definition
    ///            ╰----- 1 = Compressed time stamp header
    /// ```
    pub(crate) fn bit_set(byte: u8, position: u8) -> bool {
        assert!((0..=7).contains(&position)); // ensure u8 8-bit range.
        byte & (1 << position) != 0
    }

    /// Returns total number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &DataMessage> {
        self.records.iter()
    }

    /// VIRB only.
    /// Indexes FIT-file by generating a hashmap to `Fit.index`
    /// with first UUID in session as key and corresponding
    /// indeces (`Fit.records[start_idx .. end_idx]`) as `std::ops::Range<usize>`.
    pub fn index(&mut self) -> Result<(), FitError> {
        let camera_events = self.camera(None)?;

        let mut index: HashMap<String, Range<usize>> = HashMap::new();
        
        let mut uuid = None;
        let mut start = None;

        camera_events.iter() // can not use par_iter, since assigning outer variable
            .for_each(|event| 
                // Find start of session
                if event.camera_event_type == 0 {
                    uuid = Some(event.camera_file_uuid.to_owned());
                    start = Some(event.index);
                // Find end of session
                } else if start.is_some() && event.camera_event_type == 2 {
                    if let (Some(u), Some(s)) = (uuid.take(), start.take()) {
                        index.insert(u, s .. event.index + 1);
                    }
                }
        );

        self.index = index;

        Ok(())
    }

    /// Filter FIT data on FIT global ID (e.g. `record` global ID = 20),
    /// and/or within range indeces.
    /// 
    /// `range` is mostly there to filter FIT data on a specific recording session
    /// for Garmin VIRB cameras.
    pub fn filter(&self, global_id: Option<u16>, range: Option<&Range<usize>>) -> Vec<DataMessage> {
        let range = range.cloned().unwrap_or(0 .. self.len());

        match global_id {
            Some(g) => self.records[range]
                .par_iter()
                .filter(|r| r.global == g) // a bit cleaner than filter_map...
                .cloned()
                .collect::<Vec<DataMessage>>(),
            None => self.records[range].to_owned(),
        }
    }

    /// VIRB only.
    /// Derive start/end indeces in `FIT.records`
    /// for all recording sessions. Use `FitSession::range()`
    /// to get range for specific recording session
    /// for other `FIT` methods.
    pub fn sessions(&self) -> Result<Vec<FitSession>, FitError> {
        let mut sessions: Vec<FitSession> = Vec::new();
        let mut session: FitSession = FitSession::default();

        // Get all camera events.
        let cam = self.camera(None)?;
        
        for evt in cam.iter() {
            // Match event types.
            // Logged chronologically so
            // the first encountered event should
            // never be e.g. 2 (session end).
            match evt.camera_event_type {
                // 0 = recording session start
                0 => {
                    session.path = self.path.to_owned();
                    session.start = evt.index;
                    session.uuid.push(evt.camera_file_uuid.to_owned());
                },
                // 2 = recording session end
                2 => {
                    session.end = evt.index;
                    session.uuid.dedup(); // works since logged chronologically
                    sessions.push(session);
                    session = FitSession::default();
                },
                // Push UUID in between event types 0 and 2
                // Duplicate UUIDs will always sit next to each other.
                _ => session.uuid.push(evt.camera_file_uuid.to_owned())
            }
        };

        Ok(sessions)
    }

    /// Group Fit.records into message types.
    /// Key is numerical FIT global ID.
    pub fn group(&self) -> HashMap<u16, Vec<DataMessage>> {
        let mut grouped_records: HashMap<u16, Vec<DataMessage>> = HashMap::new();
        self.records.iter()
            .for_each(|r| {
                grouped_records
                    .entry(r.global)
                    .or_insert(Vec::new())
                    .push(r.to_owned())
            });
        grouped_records
    }

    /// VIRB only.
    /// 
    /// Derives start time of FIT-file via `timestamp_correlation/162`
    /// with option time offset in hours (e.g. time zone).
    /// If no `timestamp_correlation/162` exists in input
    /// `ParseError::NoDataForMessageType(162)` will be returned.
    /// If a required field cannot be assigned, its field definition number
    /// will be returned in `ParseError::ErrorAssigningFieldValue(FIELD_NO)`.
    /// 
    /// `default_on_error` ensures a time for first logged message can be returned.
    /// If `default_on_error = true` and no `timestamp_correlation`/`162` can be found,
    /// Garmin's FIT base start time is used: 1989-12-31T00:00:00.000.
    /// VIRB and watches log timestamps differently. VIRB logs a relative timestamp
    /// from start of FIT-file that has to be augmented by a correlation value logged
    /// at GPS satellite sync. Watches seem to log the full value directly and not need
    /// the correlation value.
    pub fn t0(&self, offset_hours: i64, default_on_error: bool) -> Result<PrimitiveDateTime, FitError> {
        let fit_datetime = Self::fit_basetime()?;
        let tc = match TimestampCorrelation::from_fit(self) {
            Ok(t) => t,
            Err(err) => match default_on_error {
                true => return Ok(fit_datetime), // FIT start time
                false => return Err(err.into())
            }
        };

        Ok(
            fit_datetime // FIT start time
            + time::Duration::hours(offset_hours) // TODO 220808 change offset to proper timezone?
            + time::Duration::seconds(tc.timestamp as i64 - tc.system_timestamp as i64)
            + time::Duration::milliseconds(tc.timestamp_ms as i64 - tc.system_timestamp_ms as i64),
        )
    }

    /// Returns `PrimitiveDateTime` object for FIT
    /// start datetime `1989-12-31 00:00:00.000`.
    // pub fn fit_datetime(&self, offset_hours: Option<i8>) -> Result<PrimitiveDateTime, FitError> {
    fn fit_basetime() -> Result<PrimitiveDateTime, FitError> {
        let basetime = Date::from_calendar_date(1989, Month::December, 31)?
            .with_hms_milli(0, 0, 0, 0)?;
        
        // if let Some(offset) = offset_hours {
        //     let datetime_off = datetime.assume_offset(UtcOffset::from_whole_seconds(offset_hours as i32 * 3600)?);
        // }

        Ok(basetime)
    }

    /// Looks up name, units, scale and offset for most
    /// message types documented in Profile.xlsx.
    /// Message types with complex fields are not supported.
    pub fn augment(&mut self) {
        self.records.par_iter_mut().for_each(|m| {
            let mt = FitMessageType::get(m.global);
            m.name = Some(mt.name.to_owned());
            m.fields.par_iter_mut().for_each(|f| {
                // Only augmenting standard fields,
                // since dev fields should already have
                // name, units, scale, offset set.
                if let Some(fld_descr) = mt.fields.get(&f.field_def_no()) {
                    f.set_name(&fld_descr.name);
                    f.set_units(fld_descr.units.as_deref());
                    f.set_scale(fld_descr.scale);
                    f.set_offset(fld_descr.offset);
                }
            });
        });
    }

    /// Garmin VIRB only.
    /// 
    /// Returns all `camera_event` messages in FIT-file.
    pub fn camera(&self, range: Option<&Range<usize>>) -> Result<Vec<CameraEvent>, FitError> {
        CameraEvent::from_fit(self, range)
    }

    pub fn gps(&self, range: Option<&Range<usize>>) -> Result<Vec<GpsMetadata>, FitError> {
        GpsMetadata::from_fit(self, range)
    }

    /// Returns calibrated sensor data for the following sensors (if present):
    /// - Magnetometer (3D)
    /// - Gyroscope (3D)
    /// - Accelerometer (3D)
    /// - Barometer (1D)
    /// 
    /// Current FIT-specification has no 2D sensors.
    pub fn sensor(
        &self,
        sensor_type: &SensorType,
        range: Option<&Range<usize>>
    ) -> Result<Vec<SensorData>, FitError> {
        SensorData::from_fit(&self, range, sensor_type)
    }

    /// Presumably VIRB only.
    /// Returns `TimestampCorrelation`.
    pub fn time(&self) -> Result<TimestampCorrelation, FitError> {
        TimestampCorrelation::from_fit(&self)
    }

    /// Returns a sub-set of `Record/20` relating to location only.
    /// Currently supported fields are present for VIRB cameras,
    /// but may not be for other devices:
    /// - timestamp, field definition number 253
    /// - latitude, field definition number 0
    /// - longitude, field definition number 1
    /// - distance, field definition number 5
    /// - speed, field definition number 73
    /// - altitude, field definition number 78
    /// - gps_accuracy, field definition number 31
    /// 
    /// If `no_fail` is set to `true`, records with errors
    /// relating to missing fields will be discarded.
    pub fn record(
        &self,
        range: Option<&Range<usize>>,
        no_fail: bool
    ) -> Result<Vec<Record>, FitError> {
        Record::from_fit(self, range, no_fail)
    }

    pub fn points(&self, range: Option<&Range<usize>>) -> Result<Vec<FitPoint>, FitError> {
        // TODO some non-VIRB devices have gps_metadata but only a small sub-set of the VIRB fields
        // TODO these will raise errors, whereas devices that don't have gps_metadata will not...
        let mut points: Vec<FitPoint> = self.gps(range)?
            .par_iter()
            .map(|p| p.to_point())
            .collect();
        
        if points.is_empty() {
            points = self.record(range, true)?
                .par_iter()
                .map(|p| p.to_point())
                .collect()
        }

        Ok(points)
    }
}