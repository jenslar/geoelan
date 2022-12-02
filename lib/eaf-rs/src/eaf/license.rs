//! EAF license.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
/// EAF license.
pub struct License {
    url: Option<String>
}

impl Default for License {
    fn default() -> Self {
        License{url: None}
    }
}