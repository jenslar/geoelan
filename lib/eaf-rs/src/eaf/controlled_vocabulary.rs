//! EAF controlled vocabularies.

use serde::{Serialize, Deserialize};

// using enum for annotation type
// see: https://users.rust-lang.org/t/serde-deserializing-a-vector-of-enums/51647
// TODO errors on optional <DESCRIPTION ...> child element in v2.8 eaf, parallel to entry:
// /Users/jens/dev/TESTDATA/eaf/2014-10-13_1800_US_CNN_Newsroom_12-493.eaf
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CONTROLLED_VOCABULARY")]
pub struct ControlledVocabulary {
    pub cv_id: String,
    pub ext_ref: Option<String>,
    pub description: Option<String>, // invalid attribute in EAF v2.8+, can instead be a value parallel to entry
    #[serde(rename = "$value")]
    pub entry: Vec<CVType>, // ORG
}

impl Default for ControlledVocabulary {
    fn default() -> Self {
        Self {
            cv_id: String::default(),
            ext_ref: None,
            description: None, // ORG
            entry: vec!(CVType::CvEntryMl(CvEntryMl::default())) // ORG
        }
    }
}

/// Contains the possibilities for CV entries,
/// depending on EAF version.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CVType {
    /// Description, 0-1 occurrences.
    #[serde(rename(deserialize = "DESCRIPTION"))]
    Description(Description),
    /// EAF v2.7 and less, 0-multiple occurrences.
    #[serde(rename(deserialize = "CV_ENTRY"))]
    CvEntry(CvEntry),
    /// EAF v2.8 and above, 0-multiple occurrences.
    #[serde(rename(deserialize = "CV_ENTRY_ML"))]
    CvEntryMl(CvEntryMl),
}

/// Controlled Vocabulary Entry for EAF v2.7
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CV_ENTRY")]
pub struct CvEntry {
    // cv_id: String,
    pub description: Option<String>,
    pub ext_ref: Option<String>,
    #[serde(rename = "$value")]
    pub value: String
}

/// Controlled Vocabulary Entry for EAF v2.8+
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CV_ENTRY_ML")]
pub struct CvEntryMl {
    pub cve_id: String,
    pub ext_ref: Option<String>,
    // #[serde(rename = "$value")]
    // description: Description, // note: elem parallel w cve
    #[serde(rename = "CVE_VALUE")]
    pub cve_values: Vec<CveValue>,
    // #[serde(rename = "DESCRIPTION3")]
    // descriptions: Vec<Description>
}

impl Default for CvEntryMl {
    fn default() -> Self {
        Self {
            cve_id: String::default(),
            ext_ref: None,
            // description: Description::default(),
            cve_values: Vec::new(),
            // descriptions: Vec::new()
        }
    }
}

// TODO this currently creates the wrong structure in KebabCase:
// TODO e.g. <Description><Description lang_reg="eng"/></Description>, rather than
// TODO e.g. <DESCRIPTION lang_reg="eng"/>
/// EAF v2.8+
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "CVE_VALUE")]
pub struct CveValue {
    pub lang_ref: String,
    pub description: Option<String>,
    #[serde(rename = "$value")]
    pub value: String
}

// EAF v2.8+
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
// #[serde(rename = "DESCRIPTION2")]
pub struct Description {
    pub lang_ref: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}
