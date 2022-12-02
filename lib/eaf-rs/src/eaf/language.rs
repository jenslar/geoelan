//! EAF language.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF Language.
pub struct Language {
    pub lang_def: String,
    pub lang_id: String,
    pub lang_label: String,
}
