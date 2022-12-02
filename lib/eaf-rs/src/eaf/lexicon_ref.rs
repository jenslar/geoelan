//! EAF lexicon reference.

use serde::{Serialize, Deserialize};
use super::annotation_document::unspecified;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF lexicon reference.
pub struct LexiconRef {
    pub datcat_id: Option<String>, // ""
    pub datcat_name: Option<String>, // "lexical-unit"
    pub lexicon_id: String, // "cwg2007_2020"
    pub lexicon_name: String, // "cwg2007_2020"
    pub lex_ref_id: String, // "lr1"
    pub name: String, // "cwg2007_2020"
    #[serde(rename = "TYPE")]
    pub component_type: String, // "elan lexicon component"
    pub url: String, // ""
}

impl Default for LexiconRef {
    fn default() -> Self {
        Self {
            // datcat_id: String::default(),
            // datcat_name: String::default(),
            datcat_id: None,
            datcat_name: None,
            lexicon_id: String::default(),
            lexicon_name: String::default(),
            lex_ref_id: String::default(),
            name: String::default(),
            component_type: String::default(),
            url: unspecified(),
        }
    }
}
