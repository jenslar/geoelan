use std::fmt;
use std::io::Cursor;

use crate::errors::FitError;

use super::definition_message::DefinitionMessage;
use super::data_field::DataField;

use super::value::Value;

#[derive(Debug, Clone)]
/// FIT message, containing data.
pub struct DataMessage {
    /// FIT global identifier.
    pub global: u16,
    /// Name in FIT SDK Profil.xlsx.
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
    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        definition: &DefinitionMessage,
        index: usize
    ) -> Result<Self, FitError> {
        
        let arch = definition.architecture;
        
        // let mut fields = Vec::new();
        // for field_def in definition.fields.iter() {
        //     // SLOWER
        //     // fields.push(DataField::new(cursor, field_def, arch)?)
        //     // FASTER
        //     fields.push(DataField{
        //         definition: field_def.to_owned(),
        //         attributes: None, // <- reason for performance difference?
        //         data: Value::new(cursor, field_def, arch)?
        //     })
        // }

        // let mut dev_fields = Vec::new();
        // for dev_field_def in definition.dev_fields.iter() {
        //     // SLOWER
        //     // dev_fields.push(DataField::new(cursor, dev_field_def, arch)?)
        //     // FASTER
        //     dev_fields.push(DataField{
        //         definition: dev_field_def.to_owned(),
        //         attributes: dev_field_def.attributes.to_owned(),
        //         data: Value::new(cursor, dev_field_def, arch)?
        //     })
        // }

        let fields = definition.fields.iter()
            .map(|field_def| {
                // SLOWER
                // fields.push(DataField::new(cursor, field_def, arch)?)
                // FASTER
                // DataField{
                //     definition: field_def.to_owned(),
                //     attributes: None, // <- reason for performance difference?
                //     data: Value::new(cursor, field_def, arch)?
                // }
                DataField::new(cursor, field_def, arch)
            })
            .collect::<Result<Vec<DataField>, FitError>>()?;

        let dev_fields = definition.dev_fields.iter()
            .map(|dev_field_def| {
                // SLOWER
                // dev_fields.push(DataField::new(cursor, dev_field_def, arch)?)
                // FASTER
                // dev_fields.push(DataField{
                //     definition: dev_field_def.to_owned(),
                //     attributes: dev_field_def.attributes.to_owned(),
                //     data: Value::new(cursor, dev_field_def, arch)?
                // })
                DataField::new(cursor, dev_field_def, arch)
            })
            .collect::<Result<Vec<DataField>, FitError>>()?;

        // SLOWER
        // let fields = definition.fields.iter()
        //     .map(|def| DataField::new(cursor, def, arch))
        //     .collect::<Result<Vec<_>, FitError>>()?;
        // SLOWER
        // let dev_fields = definition.dev_fields.iter()
        //     .map(|def| DataField::new(cursor, def, arch))
        //     .collect::<Result<Vec<_>, FitError>>()?;

        Ok(Self {
            global: definition.global,
            name: None,
            fields,
            dev_fields,
            index
        })
    }

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
