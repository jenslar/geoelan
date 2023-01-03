//! FIT definition message field.

use binread::BinRead;

use super::DataFieldAttributes;
use crate::Fit;
use crate::{errors::FitError, types::FieldDescriptionMessage};

#[derive(Debug, Copy, Clone, BinRead)]
pub struct BaseTypeDefinition(u8);

impl BaseTypeDefinition {
    /// New base type definition.
    pub fn new(def: u8) -> Self {
        Self(def)
    }

    /// Returns `false` for single byte type data (e.g. `u8`).
    /// Returns `true` for multi-byte data (e.g. `i32`).
    pub fn endian_ability(&self) -> bool {
        Fit::bit_set(self.0, 7)
    }

    /// Returns reserved bits as numerical value.
    pub fn reserved(&self) -> u8 {
        0b0110_0000 & self.0
    }

    /// Returns FIT Base Type Number.
    pub fn number(&self) -> u8 {
        0b0000_1111 & self.0
    }

    /// Returns byte length for FIT base types.
    pub fn base_len(&self) -> Result<u8, FitError> {
        match self.number() {
            0              // u8
            | 1            // i8
            | 2            // u8
            | 10           // u8
            | 13 => Ok(1), // u8

            3              // i16
            | 4            // u16
            | 11 => Ok(2), // u16

            5              // i32
            | 6            // u32
            | 8            // f32
            | 12 => Ok(4), // u32

            7 => Ok(1),    // String (unused)

            9              // f64
            | 14           // i64
            | 15           // u64
            | 16 => Ok(8), // u64

            t => Err(FitError::UnknownBaseType(t))
        }
    }
}

/// FIT message field definition.
#[derive(Debug, Clone, BinRead)]
pub struct DefinitionField {
    /// Field definition number.
    pub field_def_no: u8,
    /// Size in bytes.
    /// Multiples of base_type size,
    /// e.g. multiple of 2 for 16bit values,
    /// which indicates multiple values.
    pub size: u8,
    /// FIT base type (0-16).
    pub base_type: BaseTypeDefinition,
    /// Fit field attributes.
    #[br(default)]
    pub attributes: Option<DataFieldAttributes>
}

impl DefinitionField {
    /// Augment `DefinitionField` with developer data definitions,
    /// via `FieldDescriptionMessage`.
    pub fn augment(&mut self, field_descr: &FieldDescriptionMessage) {
        self.field_def_no = field_descr.field_definition_number;
        self.base_type = BaseTypeDefinition::new(field_descr.fit_base_type_id);
        self.attributes = Some(DataFieldAttributes::augment(field_descr));
    }
}