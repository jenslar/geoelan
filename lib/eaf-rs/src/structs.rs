use std::collections::HashMap;

#[derive(Debug)]
pub struct Eaf {
    pub header: Header,
    pub timeslots: HashMap<String, u64>, // "ts2": 2681
    // pub timeslots: HashMap<String, TimeSlot>, // "ts2": {2,2681}
    pub tiers: Vec<Tier>,
}

#[derive(Debug)]
pub struct Header {
    pub time_units: String, // default "milliseconds"
    pub media_file: String,
    pub media_descriptor: Vec<MediaDescriptor>,
    // pub video: Option<MediaDescriptor>,
    // pub audio: Option<MediaDescriptor>,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct MediaDescriptor {
    // Header
    pub extracted_from: Option<String>,
    pub mime_type: String,
    pub media_url: String,
    pub relative_media_url: String,
}

// Optional key, value element in EAF header:
// <PROPERTY NAME="SOME NAME">"VALUE"</PROPERTY>
#[derive(Debug)]
pub struct Property {
    pub name: String,    // attribute "NAME"
    pub value: String, // text content
}

#[derive(Debug)]
pub struct Tier {
    pub tier_id: String,
    pub ref_tier: Option<String>,
    pub linguistic_type_ref: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug)]
pub struct Annotation {
    pub annotation_id: String,          // "a1"
    pub annotation_ref: Option<String>, // ref_annotation?
    pub time_slot_value1: u64,          // value in TIME_ORDER for e.g. "ts1"
    pub time_slot_value2: u64,          // value in TIME_ORDER for e.g. "ts2"
    pub annotation_value: String, // "a1"
}
