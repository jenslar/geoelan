#![allow(dead_code)]

mod structs;

pub mod write {
    use std::path::PathBuf;

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
    pub fn timeorder(timeslots: String) -> String {
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

    pub fn annotation(id: usize, ts1: usize, ts2: usize, text: String) -> String {
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
        video: &PathBuf,
        audio: &PathBuf,
        fit: &String, // filename only
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

        let mut eaf_content = timeorder(timeslots.join(""));
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
    use std::path::PathBuf;
    use std::str;

    use crate::structs::{Annotation, Header, MediaDescriptor, Property};

    fn str_from_bytes(s: &[u8]) -> &str {
        str::from_utf8(s).expect("(!) Could not parse &[u8] into &str")
    }

    pub fn select_tier(tiers: &[String], tiertype: &str) -> String {
        println!("[EAF] Found the following tiers:");
        for (i, t) in tiers.iter().enumerate() {
            println!("  {:2}:  {}", i + 1, t);
        }
        loop {
            print!("Select {} tier: ", tiertype);
            stdout().flush().unwrap();
            let mut select = String::new();
            std::io::stdin()
                .read_line(&mut select)
                .expect("Failed to read line");
            let num = match select.trim().parse::<usize>() {
                Ok(n) => n - 1,
                Err(_) => {
                    println!("Not a number");
                    continue;
                }
            };
            match tiers.get(num) {
                Some(t) => return t.to_string(),
                None => {
                    println!("No such item");
                    continue;
                }
            }
        }
    }

    pub fn header(path: &PathBuf) -> Header {
        let mut reader = Reader::from_file(&path).unwrap();

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
                                let key = str_from_bytes(&a.key);
                                let val = str_from_bytes(&a.value);
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
                                let key = str_from_bytes(&a.key);
                                let val = str_from_bytes(&a.value);
                                if key == "NAME" {
                                    name = val.to_owned()
                                }
                            }
                        }
                        properties.push(Property {
                            name,
                            value: reader.read_text(e.name(), &mut Vec::new()).unwrap(),
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
                                let key = str_from_bytes(&a.key);
                                let val = str_from_bytes(&a.value);
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
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }

            // buf.clear()
        }

        Header {
            time_units,
            media_file,
            media_descriptor,
            properties,
        }
    }

    pub fn timeslots(path: &PathBuf) -> HashMap<String, u64> {
        // lookup for time_values via time_slot_id (&str, e.g. "ts1")

        let mut reader = Reader::from_file(&path).unwrap();
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
                                let key = str_from_bytes(&a.key);
                                let val = str_from_bytes(&a.value);
                                match key {
                                    "TIME_SLOT_ID" => {
                                        time_slot_id = Some(val.to_owned());
                                    }
                                    "TIME_VALUE" => {
                                        time_value = Some(val.parse::<u64>().unwrap());
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // other events exist, check quick_xml docs
            }

            if let (Some(ts_id), Some(ts_val)) = (time_slot_id, time_value) {
                ts.insert(ts_id, ts_val);
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();
        }

        ts
    }

    pub fn tiers(path: &PathBuf) -> Vec<String> {
        let mut t: Vec<String> = Vec::new();

        let mut reader = Reader::from_file(&path).unwrap();
        reader.trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name() == b"TIER" {
                        // match tier_id to selected one
                        for attr in e.attributes() {
                            if let Ok(a) = attr {
                                let key = str_from_bytes(&a.key);
                                let val = str_from_bytes(&a.value);

                                if key == "TIER_ID" {
                                    t.push(val.to_owned())
                                };
                            }
                        }
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("XmlError at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // other events exist, check quick_xml docs
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();
        }

        t
    }

    pub fn annotations(path: &PathBuf, tier: &Option<String>) -> Vec<Annotation> {
        let mut annots: Vec<Annotation> = Vec::new();

        let ts = timeslots(&path);

        let mut reader = Reader::from_file(&path).unwrap(); // std::io::BufReader<std::fs::File>
        reader.trim_text(true);

        let mut buf = Vec::new();

        // annotation values -> annots
        let mut annotation_id: Option<String> = None; // "a1"
        let mut annotation_ref: Option<String> = None; // Optional, ref_annotation?
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
                            if tier == &None {
                                continue;
                            } else if tier_found {
                                break;
                            }; // new tier encounterd, stop compiling annotations

                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    let t = tier.to_owned().unwrap();
                                    let key = str_from_bytes(&a.key);
                                    let val = str_from_bytes(&a.value);

                                    if key == "TIER_ID" && t == val {
                                        tier_found = true
                                    };
                                }
                            }
                        }
                        b"ALIGNABLE_ANNOTATION" => {
                            if tier != &None && !tier_found {
                                continue;
                            };
                            for attr in e.attributes() {
                                annot_found = true;
                                if let Ok(a) = attr {
                                    let key = str_from_bytes(&a.key);
                                    let val = str_from_bytes(&a.value);
                                    match key {
                                        "ANNOTATION_ID" => annotation_id = Some(val.to_owned()),
                                        "TIME_SLOT_REF1" => time_slot_ref1 = Some(val.to_owned()),
                                        "TIME_SLOT_REF2" => time_slot_ref2 = Some(val.to_owned()),
                                        _ => (),
                                    }
                                }
                            }
                        }
                        b"ANNOTATION_VALUE" => {
                            if annot_found {
                                let txt = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                                annotation_value = Some(txt);
                                annot_found = false;
                            }
                        }
                        _ => (),
                    }
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // other events exist, check quick_xml docs
            }

            if annotation_value.is_some() {
                annots.push(Annotation {
                    annotation_id: annotation_id.unwrap(),
                    annotation_ref: annotation_ref.to_owned(),
                    time_slot_value1: *ts.get(&time_slot_ref1.unwrap()).unwrap(),
                    time_slot_value2: *ts.get(&time_slot_ref2.unwrap()).unwrap(),
                    annotation_value: annotation_value.unwrap(),
                });
                annotation_id = None; // "a1"
                annotation_ref = None; // Optional, ref_annotation?
                time_slot_ref1 = None; // "ts1"
                time_slot_ref2 = None; // "ts2"
                annotation_value = None; // "a1"
            }

            // clear the buffer to keep memory usage low (if no borrow elsewhere)
            buf.clear();
        }

        annots
    }
}
