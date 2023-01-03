//! FIT data message.

use std::fmt;
use std::io::Cursor;

use crate::errors::FitError;

use super::definition_message::DefinitionMessage;
use super::data_field::DataField;

#[derive(Debug, Clone)]
/// FIT data message. Contains raw data.
pub struct DataMessage {
    /// FIT global identifier.
    pub global: u16,
    /// Name in FIT SDK `Profile.xlsx`.
    /// Optionally set after initial parse.
    pub name: Option<String>,
    /// Data fields.
    pub fields: Vec<DataField>,
    /// Developer data fields.
    pub dev_fields: Vec<DataField>,
    /// Message index.
    pub index: usize // slight performance decrease (50ms -> 52ms for large.fit)
}

impl DataMessage {
    /// New FIT data message. `cursor` represents the full FIT file data load,
    /// but its offset must be at the start of a data message.
    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        definition: &DefinitionMessage,
        index: usize
    ) -> Result<Self, FitError> {
        
        let arch = definition.architecture;
        
        let fields = definition.fields.iter()
            .map(|def| DataField::new(cursor, def, arch)) // slightly slower than direct init
            .collect::<Result<Vec<DataField>, FitError>>()?;

        let dev_fields = definition.dev_fields.iter()
            .map(|dev_def| DataField::new(cursor, dev_def, arch)) // slightly slower than direct init
            .collect::<Result<Vec<DataField>, FitError>>()?;

        Ok(Self {
            global: definition.global,
            name: None,
            fields,
            dev_fields,
            index
        })
    }

    /// Returns name/message type if set,
    /// and defaults to `UNKNOWN_TYPE_<ID>` if not.
    pub fn name(&self) -> String {
        self.name
            .as_ref()
            .map_or(format!("UNKNOWN_TYPE_{}", self.global), |u| u.to_owned())
    }
}

impl fmt::Display for DataMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Global: {} | {}",
            self.global,
            self.name.as_ref().map_or("UNKNOWN_TYPE", |n| n),
        )?;
        for fld in self.fields.iter() {
            writeln!(f, "      {}", fld)?;
        }
        for fld in self.dev_fields.iter() {
            writeln!(f, "  DEV {}", fld)?;
        }
        Ok(())
    }
}
