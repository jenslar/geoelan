//! FIT definition message.
//! Holds definitions for data messages documented in [FIT SDK](https://developer.garmin.com/fit).

use std::{io::Cursor, collections::HashMap};

use binread::BinReaderExt;

use super::MessageHeader;
use crate::Fit;
use crate::{errors::FitError, types::FieldDescriptionMessage};

use super::definition_field::DefinitionField;

/// FIT definition message.
#[derive(Debug)]
pub struct DefinitionMessage {
    /// Reserved. Byte 0.
    pub reserved: u8,
    /// Architecture/Endianess. Byte 1.
    /// Possible values: 0=LE, 1=BE
    pub architecture: u8,
    /// FIT global message ID. Bytes 2-3.
    pub global: u16,
    /// Definition fields. Bytes 3.. (varying length)
    pub fields: Vec<DefinitionField>,
    /// Developer definition fields (3 bytes/each).
    pub dev_fields: Vec<DefinitionField>,
}

impl DefinitionMessage {
    /// New definition message from cursor.
    /// Cursor position must be just after message header
    /// (this is mostly to avoid reading the header twice).
    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        header: &MessageHeader,
        field_descriptions: &HashMap<(u8, u8), FieldDescriptionMessage>,
    ) -> Result<DefinitionMessage, FitError> {
        // Can't use BinRead derive since architecture is
        // required to read global u16 with correct endianess...
        let reserved: u8 = cursor.read_ne()?;
        let architecture: u8 = cursor.read_ne()?;
        // TODO 230102 force LE, if unimplemented `debug` parameter is passed
        // let mut architecture: u8 = cursor.read_ne()?;
        // if architecture > 1 {
        //     architecture = 0; // Little Endian
        // }
        
        // Any multi-byte data must use architecture to determine endianess
        let global: u16 = Fit::read(cursor, architecture)?;

        let field_number: u8 = cursor.read_ne()?;
        let mut fields: Vec<DefinitionField> = Vec::new();
        for _ in 0..field_number as usize {
            fields.push(cursor.read_ne::<DefinitionField>()?);
        }
        
        let mut dev_fields: Vec<DefinitionField> = Vec::new();
        if header.dev_fields() {
            let dev_field_number: u8 = cursor.read_ne()?;
            for _ in 0..dev_field_number as usize {
                // Generate initial field definition
                let mut dev_field = cursor.read_ne::<DefinitionField>()?;

                // Get field description (error if it does not exist since these developer field defs)
                let field_descr = field_descriptions.get(&(dev_field.field_def_no, dev_field.base_type.number()))
                .ok_or_else(||
                    FitError::UnknownFieldDescription {field_number: dev_field.field_def_no, developer_data_index: dev_field.base_type.number()})?;
                
                // Augment existing field def with field description
                dev_field.augment(&field_descr);

                dev_fields.push(dev_field);
            }
        }

        Ok(DefinitionMessage {
            reserved,
            architecture,
            global,
            fields,
            dev_fields
        })
    }

    /// Returns the size of the data in bytes the definition describes,
    /// excluding the 1 byte header.
    pub fn data_size(&self) -> i64 {
        // technically lacking 1 byte for header,
        // but not needed since the cursor has already moved
        // 1 byte for the header
        self.fields.iter()
            .chain(self.dev_fields.iter())
            .map(|def| def.size as i64)
            .sum()
    }
}
