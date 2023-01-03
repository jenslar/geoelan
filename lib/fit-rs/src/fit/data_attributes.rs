//! FIT field data attributes as described in Profile.xlsx in the [FIT SDK](https://developer.garmin.com/fit/).

use std::fmt;

use crate::types::FieldDescriptionMessage;

#[derive(Debug, Clone, Default)]
pub struct DataFieldAttributes {
    pub name: String,
    pub scale: Option<u32>,
    pub offset: Option<i32>,
    pub units: Option<String>
}

impl DataFieldAttributes {
    pub fn augment(field_description: &FieldDescriptionMessage) -> Self {
        Self{
            name: field_description.field_name.to_owned(),
            scale: field_description.scale.map(u32::from),
            offset: field_description.offset.map(i32::from),
            units: field_description.units.to_owned(),
        }
    }

    pub fn augment_mut(&mut self, field_description: &FieldDescriptionMessage) {
        self.name = field_description.field_name.to_owned();
        self.scale = field_description.scale.map(u32::from);
        self.offset = field_description.offset.map(i32::from);
        self.units = field_description.units.to_owned();
    }
}

impl fmt::Display for DataFieldAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:22} SCL:{:5} OFF:{:5} UNIT:{:12}",
                self.name,
                self.scale.as_ref().map_or(&1, |v| v),
                self.offset.as_ref().map_or(&0, |v| v),
                self.units.as_ref().map_or("N/A", |n| &n[..]),
        )
    }
}
