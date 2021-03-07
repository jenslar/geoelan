#![allow(dead_code)]

pub mod errors;
pub mod structs;

pub mod write {
    use std::path::Path;

    // DOC CONTAINER, HEADER ETC
    pub fn head(author: &str, ver: &str) -> String {
        let today = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ANNOTATION_DOCUMENT
    AUTHOR="{0}"
    DATE="{1}"
    FORMAT="{2}"
    VERSION="{2}"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:noNamespaceSchemaLocation="http://www.mpi.nl/tools/elan/EAFv{2}.xsd">
"#,
            author, today, ver
        )
    }

    /// Meant for Garmin VIRB only
    pub fn meta(uuid: &str, fit: &str, start: &str, stop: &str) -> String {
        format!(
            r#"<PROPERTY NAME="fit_uuid">{0}</PROPERTY>
        <PROPERTY NAME="fit_file">{1}</PROPERTY>
        <PROPERTY NAME="fit_start">{2}</PROPERTY>
        <PROPERTY NAME="fit_end">{3}</PROPERTY>"#,
            uuid, fit, start, stop
        )
    }

    pub fn header(
        vid_rel: &str,
        vid_abs: &str,
        wav_rel: &str,
        wav_abs: &str,
        meta: &str,
    ) -> String {
        format!(
            r#"    <HEADER MEDIA_FILE="" TIME_UNITS="milliseconds">
        <MEDIA_DESCRIPTOR
            MEDIA_URL="file://{0}"
            MIME_TYPE="video/mp4"
            RELATIVE_MEDIA_URL="./{1}"/>
        <MEDIA_DESCRIPTOR
            EXTRACTED_FROM="file://{0}"
            MEDIA_URL="file://{2}"
            MIME_TYPE="audio/x-wav"
            RELATIVE_MEDIA_URL="./{3}"/>
        {4}
    </HEADER>
"#,
            vid_abs, vid_rel, wav_abs, wav_rel, meta
        )
    }

    pub fn tail() -> String {
        // eaf root tail, not all linguistic_type + constraints are needed...
        String::from(
            r#"    <TIER LINGUISTIC_TYPE_REF="default-lt" TIER_ID="default"/>
    <LINGUISTIC_TYPE GRAPHIC_REFERENCES="false" LINGUISTIC_TYPE_ID="default-lt" TIME_ALIGNABLE="true"/>
    <CONSTRAINT DESCRIPTION="Time subdivision of parent annotation's time interval, no time gaps allowed within this interval" STEREOTYPE="Time_Subdivision"/>
    <CONSTRAINT DESCRIPTION="Symbolic subdivision of a parent annotation. Annotations refering to the same parent are ordered" STEREOTYPE="Symbolic_Subdivision"/>
    <CONSTRAINT DESCRIPTION="1-1 association with a parent annotation" STEREOTYPE="Symbolic_Association"/>
    <CONSTRAINT DESCRIPTION="Time alignable annotations within the parent annotation's time interval, gaps are allowed" STEREOTYPE="Included_In"/>
</ANNOTATION_DOCUMENT>"#,
        )
    }

    // TIME
    pub fn timeorder(timeslots: &str) -> String {
        if timeslots.len() == 0 {
            format!(
                r#"    <TIME_ORDER/>
    "#
            )
        } else {
            format!(
                r#"    <TIME_ORDER>
    {}    </TIME_ORDER>
    "#,
                timeslots
            )
        }
    }

    pub fn timeslot(idx: usize, val: usize) -> String {
        // time_order > time_slot
        format!(
            r#"        <TIME_SLOT TIME_SLOT_ID="ts{0}" TIME_VALUE="{1}"/>
"#,
            idx, val
        )
    }

    // TIERS, ANNOTATIONS
    // annotation > alignable_annotation > annotation_value
    pub fn tier(id: &str, annotations: &str) -> String {
        format!(
            r#"    <TIER LINGUISTIC_TYPE_REF="default-lt" TIER_ID="{0}">
{1}    </TIER>
"#,
            id, annotations
        )
    }

    pub fn annotation(id: usize, ts1: usize, ts2: usize, text: &str) -> String {
        format!(
            r#"        <ANNOTATION>
            <ALIGNABLE_ANNOTATION ANNOTATION_ID="a{0}" TIME_SLOT_REF1="ts{1}" TIME_SLOT_REF2="ts{2}">
                <ANNOTATION_VALUE>{3}</ANNOTATION_VALUE>
            </ALIGNABLE_ANNOTATION>
        </ANNOTATION>
"#,
            id, ts1, ts2, text
        )
    }

    pub fn build(
        video: &Path,
        audio: &Path,
        fit: &str, // filename only
        uuid: &[String],
        video_start: &str,
        video_end: &str,
        annotations: &[String],
        timeslots: &[String],
        geo_tier: bool,
    ) -> String {
        let mut eaf_doc = String::new();

        // need to strip
        let video_str = video.display().to_string();
        let video_rel_str = video.file_name().unwrap().to_string_lossy().to_string(); // ok?
        let audio_str = audio.display().to_string();
        let audio_rel_str = audio.file_name().unwrap().to_string_lossy().to_string(); // ok?

        let eaf_meta = meta(
            &uuid.join(";"),
            &fit,
            &video_start.to_string(),
            &video_end.to_string(),
        );
        let eaf_header = header(
            // TODO better solution for windows UNC path hack
            // See https://github.com/rust-lang/rust/issues/42869
            &video_rel_str.trim_start_matches(r"\\?"),
            &video_str.trim_start_matches(r"\\?"),
            &audio_rel_str.trim_start_matches(r"\\?"),
            &audio_str.trim_start_matches(r"\\?"),
            &eaf_meta,
        );

        let mut eaf_content = timeorder(&timeslots.join(""));
        if geo_tier {
            eaf_content.push_str(&tier("geo", &annotations.join("")));
        }

        eaf_doc.push_str(&head("LANG-KEY", "3.0"));
        eaf_doc.push_str(&eaf_header);
        eaf_doc.push_str(&eaf_content); // time_slots + geo-tier
        eaf_doc.push_str(&tail());

        eaf_doc
    }
}

pub mod parse {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::collections::HashMap;
    use std::io::{stdout, Write};
    use std::path::Path;
    use std::str;

    use crate::errors::EafError;
    use crate::structs::{
        Annotation, AnnotationAttributes, Constraint, EafFile, Header, LinguisticType,
        MediaDescriptor, Property, Stereotype, Tier, TierAttributes,
    };

    pub fn select_tier(
        eaf_file: &EafFile,
        tiertype: &str,
        print_prefix: Option<&str>,
    ) -> Result<Tier, EafError> {
        let prefix = match print_prefix {
            Some(p) => p,
            None => "",
        };
        println!("{}Found the following tiers:", prefix);
        for (i, t) in eaf_file.tiers.iter().enumerate() {
            println!(
                "{}{:2}: {:36} {:5} annotations. Tokenized: {}",
                " ".repeat(prefix.len()),
                i + 1,
                t.attributes.tier_id,
                t.annotations.len(),
                t.tokenized
            );
        }
        loop {
            print!("{}Select {} tier: ", prefix, tiertype);
            stdout().flush()?;
            let mut select = String::new();
            std::io::stdin().read_line(&mut select)?;
            let num = match select.trim().parse::<usize>() {
                Ok(n) => n - 1,
                Err(_) => {
                    println!("Not a number");
                    continue;
                }
            };
            match eaf_file.tiers.get(num) {
                Some(t) => {
                    if t.tokenized {
                        println!("Select a non-tokenized tier");
                        continue;
                    }
                    return Ok(t.to_owned());
                }
                None => {
                    println!("No such item");
                    continue;
                }
            }
        }
    }

    pub fn header(path: &Path) -> Result<Header, EafError> {
        let mut reader = Reader::from_file(&path)?;

        let mut time_units: String = String::new();
        let mut media_file: String = String::new();
        let mut properties: Vec<Property> = Vec::new();
        let mut media_descriptor: Vec<MediaDescriptor> = Vec::new();

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"HEADER" => {
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;
                                match key {
                                    "MEDIA_FILE" => {
                                        media_file = val.to_owned();
                                    }
                                    "TIME_UNITS" => {
                                        time_units = val.to_owned();
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                    b"PROPERTY" => {
                        let mut name = String::new();
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;
                                if key == "NAME" {
                                    name = val.to_owned()
                                }
                            }
                        }
                        properties.push(Property {
                            name,
                            value: reader.read_text(e.name(), &mut Vec::new())?,
                        });
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => {
                    if e.name() == b"MEDIA_DESCRIPTOR" {
                        let mut extracted_from: Option<String> = None;
                        let mut mime_type: String = String::new();
                        let mut media_url: String = String::new();
                        let mut relative_media_url: String = String::new();
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;
                                match key {
                                    "EXTRACTED_FROM" => extracted_from = Some(val.to_owned()),
                                    "MEDIA_URL" => media_url = val.to_owned(),
                                    "MIME_TYPE" => mime_type = val.to_owned(),
                                    "RELATIVE_MEDIA_URL" => relative_media_url = val.to_owned(),
                                    _ => (),
                                }
                            }
                        }
                        media_descriptor.push(MediaDescriptor {
                            extracted_from,
                            media_url,
                            mime_type,
                            relative_media_url,
                        })
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (),
            }

            buf.clear() // NOTE 2102013->15 previously uncommented...?
        }

        Ok(Header {
            time_units,
            media_file,
            media_descriptor,
            properties,
        })
    }

    pub fn timeslots(path: &Path) -> Result<HashMap<String, u64>, EafError> {
        // lookup for time_values via time_slot_id (&str, e.g. "ts1")

        let mut reader = Reader::from_file(&path)?;
        reader.trim_text(true);

        let mut ts: HashMap<String, u64> = HashMap::new();

        let mut buf = Vec::new();

        loop {
            let mut time_slot_id: Option<String> = None;
            let mut time_value: Option<u64> = None;
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    if e.name() == b"TIME_SLOT" {
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;
                                match key {
                                    "TIME_SLOT_ID" => {
                                        time_slot_id = Some(val.to_owned());
                                    }
                                    "TIME_VALUE" => {
                                        time_value = Some(val.parse::<u64>()?);
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (), // other events exist, check quick_xml docs
            }

            if let (Some(ts_id), Some(ts_val)) = (time_slot_id, time_value) {
                ts.insert(ts_id, ts_val);
            }

            buf.clear();
        }

        Ok(ts)
    }

    pub fn tiers(path: &Path) -> Result<Vec<String>, EafError> {
        let mut t: Vec<String> = Vec::new();

        let mut reader = Reader::from_file(&path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name() == b"TIER" {
                        // match tier_id to selected one
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;

                                if key == "TIER_ID" {
                                    t.push(val.to_owned())
                                };
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (), // other events exist, check quick_xml docs
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();
        }

        Ok(t)
    }

    pub fn tier_attribs(path: &Path) -> Result<Vec<TierAttributes>, EafError> {
        let mut t: Vec<TierAttributes> = Vec::new();

        let mut reader = Reader::from_file(&path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut tier_id: Option<String> = None;
        let mut parent_ref: Option<String> = None;
        let mut participant: Option<String> = None;
        let mut annotator: Option<String> = None;
        let mut linguistic_type_ref: Option<String> = None;
        let mut break_loop = false;

        loop {
            tier_id = None;
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name() == b"TIER" {
                        // match tier_id to selected one
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;

                                match key {
                                    "TIER_ID" => tier_id = Some(val.into()),
                                    "PARENT_REF" => parent_ref = Some(val.into()),
                                    "PARTICIPANT" => participant = Some(val.into()),
                                    "ANNOTATOR" => annotator = Some(val.into()),
                                    "LINGUISTIC_TYPE_REF" => linguistic_type_ref = Some(val.into()),
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break_loop = true, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (), // other events exist, check quick_xml docs
            }

            if tier_id.is_some() {
                t.push(TierAttributes {
                    tier_id: tier_id.to_owned().unwrap(), // FIXME unwrap tier_id
                    parent_ref: parent_ref.to_owned(),
                    participant: participant.to_owned(),
                    annotator: annotator.to_owned(),
                    linguistic_type_ref: linguistic_type_ref.to_owned().unwrap(), // FIXME unwrap()
                });
                tier_id = None;
                parent_ref = None;
                participant = None;
                annotator = None;
                linguistic_type_ref = None;
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();

            // break here, rather than Event::Eof to push last tier attributes
            if break_loop {
                break;
            }
        }

        Ok(t)
    }

    /// Collects all annotations in EAF or for a single tier
    pub fn annotations(path: &Path, tier_id: Option<&String>) -> Result<Vec<Annotation>, EafError> {
        let mut annots: Vec<Annotation> = Vec::new();

        let ts = timeslots(&path)?;

        let mut reader = Reader::from_file(&path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        // annotation values -> annots
        let mut annotation_id: Option<String> = None; // "a1"
        let mut annotation_ref: Option<String> = None;
        let mut previous_annotation: Option<String> = None;
        let mut time_slot_ref1: Option<String> = None; // "ts1"
        let mut time_slot_ref2: Option<String> = None; // "ts2"
        let mut annotation_value: Option<String> = None; // "a1"

        let mut tier_found = false;
        let mut annot_found = false;

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"TIER" => {
                            // match tier_id to selected one
                            if tier_id == None {
                                continue;
                            } else if tier_found {
                                // new tier encounterd, stop compiling annotations
                                break;
                            };

                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    let t_id = tier_id.to_owned().unwrap(); // unwrap ok, checked above
                                    let key = str::from_utf8(&a.key)?;
                                    let val = str::from_utf8(&a.value)?;

                                    if key == "TIER_ID" && t_id == &val {
                                        tier_found = true
                                    };
                                }
                            }
                        }
                        b"ALIGNABLE_ANNOTATION" => {
                            if tier_id.is_some() && !tier_found {
                                continue;
                            };
                            for attr in e.attributes() {
                                annot_found = true;
                                if let Ok(a) = attr {
                                    let key = str::from_utf8(&a.key)?;
                                    let val = str::from_utf8(&a.value)?;
                                    match key {
                                        "ANNOTATION_ID" => annotation_id = Some(val.to_owned()),
                                        "TIME_SLOT_REF1" => time_slot_ref1 = Some(val.to_owned()),
                                        "TIME_SLOT_REF2" => time_slot_ref2 = Some(val.to_owned()),
                                        _ => (),
                                    }
                                }
                            }
                        }
                        b"REF_ANNOTATION" => {
                            if tier_id.is_some() && !tier_found {
                                continue;
                            };
                            for attr in e.attributes() {
                                annot_found = true;
                                if let Ok(a) = attr {
                                    let key = str::from_utf8(&a.key)?;
                                    let val = str::from_utf8(&a.value)?;
                                    match key {
                                        "ANNOTATION_ID" => annotation_id = Some(val.to_owned()),
                                        "ANNOTATION_REF" => annotation_ref = Some(val.to_owned()),
                                        "PREVIOUS_ANNOTATION" => {
                                            previous_annotation = Some(val.to_owned())
                                        }
                                        _ => (),
                                    }
                                }
                            }
                        }
                        b"ANNOTATION_VALUE" => {
                            if annot_found {
                                let txt = reader.read_text(e.name(), &mut Vec::new())?;
                                annotation_value = Some(txt); // trimming whitespace already set globally
                                annot_found = false;
                            }
                        }
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (), // other events exist, check quick_xml docs
            }

            if annotation_value.is_some() {
                annots.push(Annotation {
                    attributes: AnnotationAttributes {
                        annotation_id: annotation_id.unwrap(),
                        annotation_ref: annotation_ref.to_owned(),
                        previous_annotation: previous_annotation.to_owned(),
                        time_slot_value1: time_slot_ref1.map_or(None, |t| ts.get(&t).cloned()),
                        time_slot_value2: time_slot_ref2.map_or(None, |t| ts.get(&t).cloned()),
                    },
                    annotation_value: annotation_value.unwrap(),
                });
                annotation_id = None; // "a1"
                annotation_ref = None; // Optional, ref_annotation?
                previous_annotation = None; // Optional, ref_annotation?
                time_slot_ref1 = None; // "ts1"
                time_slot_ref2 = None; // "ts2"
                annotation_value = None; // "a1"
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();
        }

        Ok(annots)
    }

    pub fn constraints(path: &Path) -> Result<Vec<Constraint>, EafError> {
        let mut c: Vec<Constraint> = Vec::new();

        let mut reader = Reader::from_file(&path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut description: Option<String> = None;
        let mut stereotype: Option<Stereotype> = None;

        let mut break_loop = false;

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    if e.name() == b"CONSTRAINT" {
                        // match tier_id to selected one
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;

                                match key {
                                    "DESCRIPTION" => description = Some(val.into()),
                                    "STEREOTYPE" => {
                                        match val {
                                            "Symbolic_Subdivision" => {
                                                stereotype = Some(Stereotype::SymbolicSubdivision)
                                            }
                                            "Included_In" => {
                                                stereotype = Some(Stereotype::IncludedIn)
                                            }
                                            "Symbolic_Association" => {
                                                stereotype = Some(Stereotype::SymbolicAssociation)
                                            }
                                            "Time_Subdivision" => {
                                                stereotype = Some(Stereotype::TimeSubdivision)
                                            }
                                            _ => (), // only four listed stereotypes in eaf spec
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break_loop = true, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (), // other events exist, check quick_xml docs
            }

            if let Some(s) = stereotype {
                c.push(Constraint {
                    stereotype: s,
                    description: description.unwrap(),
                });
                stereotype = None;
                description = None;
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();

            // break here, rather than Event::Eof to push last constraint
            if break_loop {
                break;
            }
        }

        Ok(c)
    }

    pub fn linguistic_types(path: &Path) -> Result<Vec<LinguisticType>, EafError> {
        let mut l: Vec<LinguisticType> = Vec::new();

        let mut reader = Reader::from_file(&path)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        // Linguistic Type
        let mut constraint: Option<Stereotype> = None;
        let mut graphic_references: Option<bool> = None;
        let mut linguistic_type_id: Option<String> = None;
        let mut time_alignable: Option<bool> = None;

        let mut break_loop = false;

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    if e.name() == b"LINGUISTIC_TYPE" {
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str::from_utf8(&a.key)?;
                                let val = str::from_utf8(&a.value)?;

                                match key {
                                    "CONSTRAINTS" => match val {
                                        "Symbolic_Subdivision" => {
                                            constraint = Some(Stereotype::SymbolicSubdivision)
                                        }
                                        "Included_In" => constraint = Some(Stereotype::IncludedIn),
                                        "Symbolic_Association" => {
                                            constraint = Some(Stereotype::SymbolicAssociation)
                                        }
                                        "Time_Subdivision" => {
                                            constraint = Some(Stereotype::TimeSubdivision)
                                        }
                                        _ => (),
                                    },
                                    "GRAPHIC_REFERENCES" => match val {
                                        "true" => graphic_references = Some(true),
                                        "false" => graphic_references = Some(false),
                                        _ => (),
                                    },
                                    "LINGUISTIC_TYPE_ID" => linguistic_type_id = Some(val.into()),
                                    "TIME_ALIGNABLE" => match val {
                                        "true" => time_alignable = Some(true),
                                        "false" => time_alignable = Some(false),
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break_loop = true, // exits the loop when reaching end of file
                Err(e) => return Err(EafError::QuickXMLError(e)),
                _ => (), // other events exist, check quick_xml docs
            }

            if linguistic_type_id.is_some() {
                l.push(LinguisticType {
                    constraints: constraint,
                    graphic_references: graphic_references.unwrap(),
                    linguistic_type_id: linguistic_type_id.unwrap(),
                    time_alignable: time_alignable.unwrap(),
                });
                constraint = None;
                graphic_references = None;
                linguistic_type_id = None;
                time_alignable = None;
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();

            // break here, rather than Event::Eof to push last constraint
            if break_loop {
                break;
            }
        }

        Ok(l)
    }
}
