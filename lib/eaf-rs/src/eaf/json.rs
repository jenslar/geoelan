//! Simplified EAF structure for exporting to JSON.

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use super::AnnotationDocument;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Simplified EAF structure containing only media links,
/// properties, and tiers for exporting to JSON.
pub struct JsonEaf {
    pub media: Vec<String>,
    pub properties: HashMap<String, String>,
    pub tiers: Vec<JsonTier>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Simplified EAF tier for exporting to JSON.
pub struct JsonTier {
    pub tier_id: String,
    pub parent_ref: Option<String>,
    pub participant: Option<String>,
    pub annotator: Option<String>,
    pub annotations: Vec<JsonAnnotation>
}

impl Default for JsonTier {
    fn default() -> Self {
        Self {
            tier_id: String::default(),
            parent_ref: None,
            participant: None,
            annotator: None,
            annotations: Vec::new()
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Simplified EAF annotation for exporting to JSON.
/// Some annotation types are not well-supported by this structure.
pub struct JsonAnnotation {
    pub id: String,
    pub ref_id: Option<String>,
    pub start: i64,
    pub end: i64,
    pub value: String
}

impl Default for JsonEaf {
    fn default() -> Self {
        Self {
            media: Vec::new(),
            properties: HashMap::new(),
            tiers: Vec::new()
        }
    }
}

impl From<&AnnotationDocument> for JsonEaf {
    fn from(eaf: &AnnotationDocument) -> Self {
        // ensure that eaf is derived?
        let mut json = Self::default();

        json.media = eaf.media_abs_paths();
        json.properties = eaf.properties();

        for eaf_tier in eaf.tiers.iter() {
            let mut json_tier = JsonTier::default();
            json_tier.tier_id = eaf_tier.tier_id.to_owned();
            json_tier.parent_ref = eaf_tier.parent_ref.to_owned();
            json_tier.participant = eaf_tier.participant.to_owned();
            json_tier.annotator = eaf_tier.annotator.to_owned();

            // currently filtering out annotations with no explicit timestamps set
            // also tokenized tiers currently won't work
            json_tier.annotations = eaf_tier.annotations.iter()
                .filter_map(|a| if let (Some(ts1), Some(ts2)) = a.ts_val() {
                    Some(JsonAnnotation {
                        id: a.id(),
                        ref_id: a.ref_id(),
                        start: ts1,
                        end: ts2,
                        value: a.value()
                    })
                } else {
                    None
                })
                .collect();
            
            json.tiers.push(json_tier)
        }

        json
    }
}