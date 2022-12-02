//! EAF locale.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF locale.
pub struct Locale {
    pub language_code: String,
    pub country_code: Option<String>,
    pub variant: Option<String>,
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            language_code: "eng".to_owned(),
            country_code: None,
            variant: None,
        }
    }
}