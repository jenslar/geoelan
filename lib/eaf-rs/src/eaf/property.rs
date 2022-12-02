//! EAF property.

use serde::{Serialize, Deserialize};

/// Optional key, value element in EAF header.
/// Can be used to store custom information (be sure to pick a
/// unique attribute name).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Property {
    pub name: String,  // attribute "NAME"
    // #[serde(rename(deserialize = "$value"))] // NO GO FOR EAF: makes value attribute on serialization
    #[serde(rename = "$value")] // NO GO FOR JSON: adds '$value' node to structure
    pub value: String, // text content
}
