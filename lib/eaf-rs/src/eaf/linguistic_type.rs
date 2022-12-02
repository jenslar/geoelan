//! EAF linguistic type.

use serde::{Serialize, Deserialize};

use super::StereoType;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
/// EAF linguistic type.
pub struct LinguisticType {
    pub linguistic_type_id: String,
    pub time_alignable: bool,
    pub constraints: Option<String>, // ideally Constraint enum
    pub graphic_references: bool,
    pub controlled_vocabulary: Option<String>,
    pub ext_ref: Option<String>,
    pub lexicon_ref: Option<String>,
}

impl Default for LinguisticType {
    fn default() -> Self {
        Self {
            linguistic_type_id: "default-lt".to_owned(),
            time_alignable: true,
            constraints: None,
            graphic_references: false,
            controlled_vocabulary: None,
            ext_ref: None,
            lexicon_ref: None, // refers to element LEXICON_REF
        }
    }
}

impl LinguisticType {
    /// Checks whether the linguistic type is time alignable, depending on constraints.
    pub fn time_alignable(stereotype: StereoType, has_constraint: bool) -> bool {
        match (stereotype, has_constraint) {
            (StereoType::IncludedIn, true) => true, // time alignable: true
            (StereoType::SymbolicAssociation, true) => false, // time alignable: true
            (StereoType::SymbolicSubdivision, true) => false, // time alignable: true
            (StereoType::TimeSubdivision, true) => true, // time alignable: true
            (_, false) => true
        }
    }
    
    pub fn new(id: &str, stereotype: Option<&StereoType>) -> Self {
        let alignable = match stereotype {
            Some(s) => s.time_alignable(),
            None => true
        };
        Self{
            linguistic_type_id: id.to_owned(),
            time_alignable: alignable,
            constraints: stereotype.map(|s| s.to_owned().into()),
            // constraints: stereotype.map(|s| s.to_constraint().stereotype),
            graphic_references: false,
            controlled_vocabulary: None,
            ext_ref: None,
            lexicon_ref: None, // refers to element LEXICON_REF
        }
    }
}
