//! FIT field description message (global ID 206). Contains definitions for custom developer data.

use std::ops::Range;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::fit::DataMessage;
use crate::FitError;
use crate::Fit;

/// Field Description Message, global id 206.
/// Describes the structure for custom data.
#[derive(Debug, Clone)]
pub struct FieldDescriptionMessage {
    /// Field def no: 0, Index of the developer that this message maps to
    pub developer_data_index: u8,
    /// Field def no: 1 Field Number that maps to this message
    pub field_definition_number: u8,
    /// Field def no: 2 Base type of the field
    pub fit_base_type_id: u8,
    /// Field def no: 3, 64 bytes (not verified if 0-padded)
    pub field_name: String,
    // The following seem optional, but this is not verified. Not in e.g. Wahoo Rival Fit
    /// Field def no: 8, Units associated with the field, 16 bytes (not verified if 0-padded),
    pub units: Option<String>,
    /// Field def no: 4
    pub array: Option<u8>,
    /// Field def no: 5
    pub components: Option<String>,
    /// Field def no: 6
    /// Note that Profile.xslx has scale:s > 255 so need to upcast to u32 for DataField
    pub scale: Option<u8>,
    /// Field def no: 7
    /// Note that Profile.xslx has offset:s > 127 so need to upcast to i32 for DataField
    pub offset: Option<i8>,
    /// Field def no: 9
    pub bits: Option<String>,
    /// Field def no: 10
    pub accumulate: Option<String>,
    /// Field def no: 13, fit_base_unit (only foundo in "weight"?). Possible values: 0,1,2
    pub fit_base_unit_id: Option<u16>,
    /// Field def no: 14, mesg_num (a.k.a. FIT global ID)
    pub native_mesg_num: Option<u16>,
    /// Field def no: 15, Equivalent native field number
    pub native_field_num: Option<u8>,
    pub(crate) index: usize
}

impl FieldDescriptionMessage {
    /// Returns a `FieldDescriptionMessage`/206 from a restructured `DataMessage`.
    /// Error handling is for determining if a required field could not be assigned
    /// , i.e. was not present in input data.
    pub fn new(data_message: &DataMessage) -> Result<FieldDescriptionMessage, FitError> {
        let global_id = 206;

        if data_message.global != global_id {
            return Err(FitError::UnexpectedMessageType{expected: global_id, got: data_message.global})
        }

        // REQUIRED?
        let mut developer_data_index: Option<u8> = None; // id: 0
        let mut field_definition_number: Option<u8> = None; // id: 1
        let mut fit_base_type_id: Option<u8> = None; // id: 2
        let mut field_name: Option<String> = None; // id: 3, 64 bytes (up to? 0-padded?)

        // OPTIONAL? NOT IN WAHOO RIVAL (WATCH) FIT
        let mut units: Option<String> = None; // id: 8, 16 bytes (up to? 0-padded?)

        // OPTIONAL?
        let mut array: Option<u8> = None; // id: 4
        let mut components: Option<String> = None; // id: 5
        let mut scale: Option<u8> = None; // id: 6
        let mut offset: Option<i8> = None; // id: 7
        let mut bits: Option<String> = None; // id: 9
        let mut accumulate: Option<String> = None; // id: 10
        let mut fit_base_unit_id: Option<u16> = None; // id: 13 in Profile: only 0,1,2
        let mut native_mesg_num: Option<u16> = None; // id: 14 global id
        let mut native_field_num: Option<u8> = None; // id: 15

        for field in data_message.fields.iter() {
            match field.field_def_no() {
                0 => developer_data_index = field.data.as_ref().into(),
                1 => field_definition_number = field.data.as_ref().into(),
                2 => fit_base_type_id = field.data.as_ref().into(),
                3 => field_name = field.data.as_ref().into(),
                8 => units = field.data.as_ref().into(),
                // OPTIONAL?
                4 => array = field.data.as_ref().into(),
                5 => components = field.data.as_ref().into(),
                6 => scale = field.data.as_ref().into(),
                7 => offset = field.data.as_ref().into(),
                9 => bits = field.data.as_ref().into(),
                10 => accumulate = field.data.as_ref().into(),
                13 => fit_base_unit_id = field.data.as_ref().into(),
                14 => native_mesg_num = field.data.as_ref().into(),
                15 => native_field_num = field.data.as_ref().into(),
                _ => (),
            }
        }

        Ok(FieldDescriptionMessage {
            developer_data_index: developer_data_index
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 0})?,
            field_definition_number: field_definition_number
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 1})?,
            fit_base_type_id: fit_base_type_id
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 2})?,
            field_name: field_name
                .ok_or_else(|| FitError::ErrorAssigningField{global: global_id, field_def_no: 3})?,
            // OPTIONAL? NOT IN WAHOO RIVAL FIT
            units,
            array,
            components,
            scale,
            offset,
            bits,
            accumulate,
            fit_base_unit_id,
            native_mesg_num,
            native_field_num,
            index: data_message.index
        })
    }

    /// Parses all field_description_message/206 and returns these in a more accessible form.
    pub fn from_fit(
        fit: &Fit,
        range: Option<Range<usize>>, // slice indeces for session
    ) -> Result<Vec<Self>, FitError> {
        let global = 206_u16;

        let range = range.unwrap_or(0 .. fit.len());

        fit.records[range].par_iter()
            .filter(|rec| rec.global == global)
            .map(Self::new)
            .collect()
    }
}
