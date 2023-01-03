//! FIT data message field.

use std::fmt;
use std::io::Cursor;

use crate::errors::FitError;

use super::DataFieldAttributes;
use super::DefinitionField;

use super::value::Value;

/// FIT data message field.
#[derive(Debug, Clone)]
pub struct DataField {
    /// Field definition.
    pub definition: DefinitionField,
    /// Optional attributes listing
    /// field name, scale, offset, unit.
    /// Required for developer data.
    pub attributes: Option<DataFieldAttributes>,
    /// Data values.
    /// Container enum for Vec<T>,
    /// where T are types defined in the
    /// [FIT SDK](https://developer.garmin.com/fit/)
    pub data: Value,
}

impl DataField {
    /// New FIT data message field. `cursor` represents the full FIT file data load,
    /// but its offset must be at the start of a data message field.
    pub fn new(cursor: &mut Cursor<Vec<u8>>, field_def: &DefinitionField, arch: u8) -> Result<Self, FitError> {
        Ok(Self {
            definition: field_def.to_owned(),
            attributes: field_def.attributes.to_owned(),
            data: Value::new(cursor, field_def, arch)?
        })
    }

    // pub fn value<T>(&self) -> Option<Vec<T>> {
    //     self.data.into()
    //     // None
    // }

    /// FIT field definition number.
    pub fn field_def_no(&self) -> u8 {
        self.definition.field_def_no
    }

    /// Set attributes to default.
    fn init_attr(&mut self) {
        if self.attributes.is_none() {
            self.attributes = Some(DataFieldAttributes::default())
        }
    }

    /// Returns field name if set.
    pub fn name(&self) -> Option<&str> {
        self.attributes.as_ref()
        .map(|attr| attr.name.as_str())
    }
    
    /// Returns field scale if set.
    pub fn scale(&self) -> Option<u32> {
        self.attributes.as_ref()
        .and_then(|attr| attr.scale)
    }
    
    /// Returns field offset if set.
    pub fn offset(&self) -> Option<i32> {
        self.attributes.as_ref()
        .and_then(|attr| attr.offset)
    }
    
    /// Returns field units if set.
    pub fn units(&self) -> Option<&str> {
        self.attributes.as_ref()
            .and_then(|attr| attr.units.as_deref())
    }

    /// Set field name.
    pub fn set_name(&mut self, name: &str) {
        self.init_attr();
        self.attributes.as_mut()
        .map(|attr| attr.name = name.to_owned());
    }
    
    /// Set field scale.
    pub fn set_scale(&mut self, scale: Option<u32>) {
        self.init_attr();
        self.attributes.as_mut()
        .map(|attr| attr.scale = scale);
    }
    
    /// Set field offset.
    pub fn set_offset(&mut self, offset: Option<i32>) {
        self.init_attr();
        self.attributes.as_mut()
        .map(|attr| attr.offset = offset);
    }
    
    /// Set field units.
    pub fn set_units(&mut self, units: Option<&str>) {
        self.init_attr();
        self.attributes.as_mut()
            .map(|attr| attr.units = units.map(String::from));
    }
}

impl fmt::Display for DataField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:4} {:22} SCL:{:5} OFF:{:5} UNIT:{:12} {:?}",
            self.field_def_no(),
            self.name().map_or("UNKNOWN_FIELD", |n| &n[..]),
            self.scale().as_ref().map_or(&1, |v| v),
            self.offset().as_ref().map_or(&0, |v| v),
            self.units().map_or("N/A", |n| &n[..]),
            self.data, // bad display for fields with large arrays, e.g. 3d sensor data
        )
    }
}