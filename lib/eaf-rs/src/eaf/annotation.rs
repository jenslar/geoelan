//! EAF annotation. An annotation can either be "aligned" (time aligned), with explicit references to time slots, or "referred" which instead refers to a "parent" annotation in the parent tier.

use std::{str::Chars, collections::HashMap};
use regex::Regex;
use serde::{Serialize, Deserialize};

// see: https://users.rust-lang.org/t/serde-deserializing-a-vector-of-enums/51647

/// ELAN annotations can be either an alignable annotation (in a main tier),
/// which contains explicit references to time slot ID:s, or a "reference annotation" (in a referred tier),
/// which instead of time slot ID:s refers to an annotation ID in the parent tier.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
// #[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename = "ANNOTATION")]
pub struct Annotation {
    #[serde(rename = "$value")]
    // #[serde(rename(deserialize = "$value"))]
    // #[serde(flatten)]
    pub annotation_type: AnnotationType,

    // below works around the quick-xml duolcation bug,
    // but is not a good representation,
    // since it implies one, both or none of the below are ok,
    // when in fact one and only one of the below must exist.
    // alignable_annotation: Option<AlignableAnnotation>,
    // ref_annotation: Option<RefAnnotation>,
}

impl Default for Annotation {
    fn default() -> Self {
        Self {annotation_type: AnnotationType::AlignableAnnotation(AlignableAnnotation::default())}
    }
}

impl From<RefAnnotation> for Annotation {
    fn from(ref_annotation: RefAnnotation) -> Self {
        Self {
            annotation_type: AnnotationType::RefAnnotation(ref_annotation.to_owned())
        }
    }
}

impl From<AlignableAnnotation> for Annotation {
    fn from(alignable_annotation: AlignableAnnotation) -> Self {
        Self {
            annotation_type: AnnotationType::AlignableAnnotation(alignable_annotation.to_owned())
        }
    }
}

// impl Serialize for Annotation {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str({
//             let head = "<ANNOTATION>".to_owned();
//             let tail = "</ANNOTATION>".to_owned();
//             let annot_type = match &self.annotation_type {
//                 AnnotationType::AlignableAnnotation(a) => {
//                     format!("<ALIGNABLE_ANNOTATION ANNOTATION_ID=\"{}\" TIME_SLOT_REF1=\"{}\" TIME_SLOT_REF2=\"{}\"><ANNOTATION_VALUE>{}</ANNOTATION_VALUE></ALIGNABLE_ANNOTATION>",
//                         a.annotation_id,
//                         a.time_slot_ref1,
//                         a.time_slot_ref2,
//                         a.annotation_value.0
//                     )
//                 },
//                 AnnotationType::RefAnnotation(a) => {
//                     format!("<ALIGNABLE_ANNOTATION ANNOTATION_ID=\"{}\" ANNOTATION_REF=\"{}\"><ANNOTATION_VALUE>{}</ANNOTATION_VALUE></ALIGNABLE_ANNOTATION>",
//                         a.annotation_id,
//                         a.annotation_ref,
//                         a.annotation_value.0
//                     )
//                 }
//             };
//             &[head, annot_type, tail].join("")
//         })
//     }
// }

impl Annotation {
    /// Creates a new annotation from annotation type.
    /// 
    /// `Annotation::from(my_annotation)` is usually simpler to use.
    pub fn new(annotation_type: &AnnotationType) -> Self {
        match annotation_type {
            AnnotationType::AlignableAnnotation(a) => Self::from(a.to_owned()),
            AnnotationType::RefAnnotation(a) => Self::from(a.to_owned())
        }
    }

    /// Creates a new aligned annotation.
    pub fn alignable(
        annotation_value: &str,
        annotation_id: &str,
        time_slot_ref1: &str,
        time_slot_ref2: &str,
    ) -> Self {
        let mut annotation = Self::default();

        annotation.set_id(annotation_id);
        annotation.set_value(annotation_value);
        annotation.set_ts_ref(time_slot_ref1, time_slot_ref2);

        annotation
    }

    /// Creates a new referred annotation.
    pub fn referred(
        annotation_value: &str,
        annotation_id: &str,
        annotation_ref: &str,
        previous: Option<&str>,
    ) -> Self {
        let mut annotation = Self::from(RefAnnotation::default());

        annotation.set_id(annotation_id);
        annotation.set_value(annotation_value);
        annotation.set_ref_id(annotation_ref);
        if let Some(p) = previous {
            annotation.set_previous(p)
        }

        annotation
    }

    /// Converts a ref annotation to an alignable annotation.
    /// If input annotation is already an alignable annotation,
    /// a copy is returned untouched.
    /// 
    /// Does not validate provided time slot references.
    pub fn to_alignable(&self, time_slot_ref1: &str, time_slot_ref2: &str) -> Annotation {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(_) => self.to_owned(),
            AnnotationType::RefAnnotation(a) => {
                Annotation::from(
                    AlignableAnnotation {
                        annotation_id: a.annotation_id.to_owned(),
                        ext_ref: a.ext_ref.to_owned(),
                        lang_ref: a.lang_ref.to_owned(),
                        cve_ref: a.cve_ref.to_owned(),
                        time_slot_ref1: time_slot_ref1.to_owned(),
                        time_slot_ref2: time_slot_ref2.to_owned(),
                        annotation_value: a.annotation_value.to_owned(),
                        tier_id: a.tier_id.to_owned(),
                        time_value1: a.time_value1,
                        time_value2: a.time_value2,
                    }
                )
            }
        }
    }

    /// Converts an alignable annotation to a ref annotation.
    /// If input annotation is already a ref annotation,
    /// a copy is returned untouched.
    /// 
    /// Does not validate specified reference annotation ID (`ref_id`)
    /// or previous annotation (`prev`).
    pub fn to_referred(&self, ref_id: &str, previous_annotation: Option<&str>, main_annotation: Option<&str>) -> Annotation {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                Annotation::from(
                    RefAnnotation {
                        annotation_id: a.annotation_id.to_owned(),
                        ext_ref: a.ext_ref.to_owned(),
                        lang_ref: a.lang_ref.to_owned(),
                        cve_ref: a.cve_ref.to_owned(),
                        annotation_ref: ref_id.to_owned(),
                        previous_annotation: previous_annotation.map(String::from),
                        annotation_value: a.annotation_value.to_owned(),
                        tier_id: a.tier_id.to_owned(),
                        time_value1: a.time_value1,
                        time_value2: a.time_value2,
                        main_annotation: main_annotation.map(String::from)
                    }
                )
            },
            AnnotationType::RefAnnotation(_) => self.to_owned()
        }
    }

    /// Returns annotation value.
    /// 
    /// TODO check if `<ANNOTATION_VALUE/>` raises error (or returns `None`) for `value()`.
    pub fn value(&self) -> String {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.annotation_value.0.to_owned(),
            AnnotationType::RefAnnotation(a) => a.annotation_value.0.to_owned()
        }
    }

    /// Sets annotation value.
    pub fn set_value(&mut self, annotation_value: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.annotation_value = AnnotationValue(annotation_value.to_owned());
            },
            AnnotationType::RefAnnotation(a) => {
                a.annotation_value = AnnotationValue(annotation_value.to_owned());
            }
        }
    }

    /// Returns annotation ID.
    pub fn id(&self) -> String {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.annotation_id.to_owned(),
            AnnotationType::RefAnnotation(a) => a.annotation_id.to_owned()
        }
    }

    /// Sets annotation ID.
    pub fn set_id(&mut self, annotation_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.annotation_id = annotation_id.to_owned(),
            AnnotationType::RefAnnotation(a) => a.annotation_id = annotation_id.to_owned()
        };
    }

    /// Returns referred annotation ID for a referred annotation,
    /// and `None` for an aligned annotation.
    /// I.e. the attribute `ANNOTATION_REF`, if annotation is a `REF_ANNOTATION`.
    pub fn ref_id(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::RefAnnotation(a) => Some(a.annotation_ref.to_owned()),
            _ => None,
        }
    }

    /// Sets annotation ref ID if annotation is a "referred annotation".
    pub fn set_ref_id(&mut self, ref_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::RefAnnotation(a) => a.annotation_ref = ref_id.to_owned(),
            _ => (),
        };
    }

    /// Returns true if annotation is a "referred annotation".
    pub fn is_ref(&self) -> bool {
        matches!(self.annotation_type, AnnotationType::RefAnnotation(_))
    }

    /// Returns annotation ID for "previous annotation" if it exists
    /// and annotation is a referred annotation, and None otherwise.
    /// If this attribute is set it indicates that the parent tier is tokenzied.
    /// 
    /// Note that the first annotation for a series of tokenized annotations does not
    /// contain the `PREVIOUS_ANNOTATION` attribute, only those that follow do.
    pub fn previous(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::RefAnnotation(a) => a.previous_annotation.to_owned(),
            _ => None,
        }
    }

    /// Sets annotation ref ID if annotation is a ref annotation.
    pub fn set_previous(&mut self, prev_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::RefAnnotation(a) => {
                a.previous_annotation = Some(prev_id.to_owned())
            },
            _ => (),
        };
    }

    /// Returns time slot references for an alignable annotation,
    /// and `None` for a referred annotation.
    /// I.e. the attributes `TS_REF1` and `TS_REF2` if annotation is an alignable annotation.
    pub fn ts_ref(&self) -> Option<(String, String)> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                Some((a.time_slot_ref1.to_owned(), a.time_slot_ref2.to_owned()))
            },
            _ => None
        }
    }

    /// Sets (new) time slot references for an alignable annotation.
    /// I.e. the attributes `TS_REF1` and `TS_REF` if annotation is an alignable annotation.
    pub fn set_ts_ref(&mut self, time_slot_ref1: &str, time_slot_ref2: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.time_slot_ref1 = time_slot_ref1.to_owned();
                a.time_slot_ref2 = time_slot_ref2.to_owned();
            },
            _ => ()
        }
    }

    /// Returns external reference if it exists.
    pub fn ext_ref(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.ext_ref.to_owned(),
            AnnotationType::RefAnnotation(a) => a.ext_ref.to_owned(),
        }
    }

    /// Sets external reference.
    pub fn set_ext_ref(&mut self, ext_ref: Option<&str>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.ext_ref = ext_ref.map(String::from),
            AnnotationType::RefAnnotation(a) => a.ext_ref = ext_ref.map(String::from),
        }
    }

    /// Returns language reference if it exists.
    pub fn lang_ref(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.lang_ref.to_owned(),
            AnnotationType::RefAnnotation(a) => a.lang_ref.to_owned(),
        }
    }

    /// Sets language reference.
    pub fn set_lang_ref(&mut self, lang_ref: Option<&str>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.lang_ref = lang_ref.map(String::from),
            AnnotationType::RefAnnotation(a) => a.lang_ref = lang_ref.map(String::from),
        }
    }

    /// Returns CVE reference if it exists.
    pub fn cve_ref(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.cve_ref.to_owned(),
            AnnotationType::RefAnnotation(a) => a.cve_ref.to_owned(),
        }
    }

    /// Sets CVE reference.
    pub fn set_cve_ref(&mut self, cve_ref: Option<&str>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => a.cve_ref = cve_ref.map(String::from),
            AnnotationType::RefAnnotation(a) => a.cve_ref = cve_ref.map(String::from),
        }
    }

    /// Returns main annotation ID for a "referred annotation",
    /// and `None` for an "aligned annotation" (or if `AnnotationDocument::derive()`
    /// has not been run).
    /// I.e. the annotation at the top of the hierarchy in the main tier.
    /// 
    /// Note that this field is not part of the EAF specification.
    pub fn main(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::RefAnnotation(a) => a.main_annotation.to_owned(),
            _ => None,
        }
    }

    /// Sets "main" annotation ID for a referred annotation.
    /// I.e. if the annotation is deep in a nested hierarchy
    /// of referred tiers, this sets the specified ID
    /// as representing the alignable annotation "at the top"
    /// in the main tier. Mostly for internal use, since "main annotation"
    /// is derived and set via `AnnotationDocument::derive()`.
    /// 
    /// Note that this field is not part of the EAF specification.
    pub fn set_main(&mut self, main_annotation: &str) {
        match &mut self.annotation_type {
            AnnotationType::RefAnnotation(a) => {
                a.main_annotation = Some(main_annotation.to_owned());
            },
            _ => (),
        }
    }

    /// Returns time slot values if set via e.g. `AnnotationDocument::derive()`.
    /// 
    /// Note that these fields are not part of the EAF specification.
    pub fn ts_val(&self) -> (Option<i64>, Option<i64>) {
    // pub fn ts_val(&self) -> (Option<u64>, Option<u64>) {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                (a.time_value1, a.time_value2)
            },
            AnnotationType::RefAnnotation(a) => {
                (a.time_value1, a.time_value2)
            },
        }
    }

    /// Sets `Annotation.time_slot_val1` and `Annotation.time_slot_val2`.
    /// 
    /// Note that these fields are not part of the EAF specification.
    pub fn set_ts_val(&mut self, time_value1: Option<i64>, time_value2: Option<i64>) {
    // pub fn set_ts_val(&mut self, time_value1: Option<u64>, time_value2: Option<u64>) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.time_value1 = time_value1;
                a.time_value2 = time_value2;
            },
            AnnotationType::RefAnnotation(a) => {
                a.time_value1 = time_value1;
                a.time_value2 = time_value2;
            },
        }
    }

    /// Returns tier ID.
    /// 
    /// Note that this field is not part of the EAF specification.
    pub fn tier_id(&self) -> Option<String> {
        match &self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.tier_id.to_owned()
            },
            AnnotationType::RefAnnotation(a) => {
                a.tier_id.to_owned()
            },
        }
    }

    /// Sets tier ID.
    /// 
    /// Note that this field is not part of the EAF specification.
    pub fn set_tier_id(&mut self, tier_id: &str) {
        match &mut self.annotation_type {
            AnnotationType::AlignableAnnotation(a) => {
                a.tier_id = Some(tier_id.to_owned());
            },
            AnnotationType::RefAnnotation(a) => {
                a.tier_id = Some(tier_id.to_owned());
            },
        }
    }

    /// Naive implementation of ngram. Checks lower case variants only.
    /// Optionally remove regex matches, before checking. Only usable
    /// for scripts which uses whitespace as a delimiter
    /// (i.e. CJK is out of scope for this implementation).
    /// Returns `HashMap<ngram, count>`.
    pub fn ngram(&self, size: usize, delete_regex: Option<&Regex>) -> HashMap<String, usize> {
        let mut ngrams: HashMap<String, usize> = HashMap::new();
        let split: Vec<String> = self.value()
            .split_ascii_whitespace()
            .map(|v| {
                // v.
                if let Some(rx) = delete_regex {
                    rx.replace_all(&v.to_lowercase(), "").to_string()
                } else {
                    v.to_lowercase()
                }
            })
            .collect();

        for value in split.windows(size) {
            *ngrams.entry(value.join(" ")).or_insert(0) += 1;
        }

        ngrams
    }
}

/// `AnnotationType` refers to either an `AlignableAnnotation`
/// or a `RefAnnotation`. Mostly for internal use to deserialize
/// and serialize the document correctly. Every annotation in the EAF
/// must one of these annotation types.
/// 
/// Note: Quick-xml currently has a bug when de/serializing enums. Tag is duplicated.
/// Symptom: rename is required for both enum and contained structs or no output,
/// but tags are duplicated.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
// #[serde(untagged)]
pub enum AnnotationType {
    // #[serde(rename = "$value")] // doesn't work
    // #[serde(rename = "ALIGNABLE_ANNOTATION")] // required, but causes double annotation tags
    // #[serde(rename(serialize = "$value"))] // doesn't work
    #[serde(rename(deserialize = "ALIGNABLE_ANNOTATION"))] // required, but causes double annotation tags
    // #[serde(rename = "ALIGNABLE_ANNOTATION")] // required, but causes double annotation tags
    AlignableAnnotation(AlignableAnnotation),
    // #[serde(rename = "$value")] // doesn't work
    // #[serde(rename(serialize = "$value"))] // doesn't work
    #[serde(rename(deserialize = "REF_ANNOTATION"))] // required, but causes double annotation tags
    // #[serde(rename = "REF_ANNOTATION")] // required, but causes double annotation tags
    RefAnnotation(RefAnnotation)
}

/// Alignable annotation. An annotation type found in a main tier,
/// with explicit time slot references.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "ALIGNABLE_ANNOTATION")] // causes double annotation tags
pub struct AlignableAnnotation {
    // Attributes
    pub annotation_id: String,
    pub ext_ref: Option<String>,
    pub lang_ref: Option<String>,
    pub cve_ref: Option<String>,
    pub time_slot_ref1: String,
    pub time_slot_ref2: String,
    // Child node with text value, but no attributes
    pub annotation_value: AnnotationValue,
    // pub annotation_value: String
    #[serde(skip)]
    pub tier_id: Option<String>, // not part of EAF spec
    #[serde(skip)]
    pub time_value1: Option<i64>, // not part of EAF spec, for populating/editing time order
    #[serde(skip)]
    pub time_value2: Option<i64>, // not part of EAF spec, for populating/editing time order
    // #[serde(skip)]
    // pub time_value1: Option<u64>, // not part of EAF spec, for populating/editing time order
    // #[serde(skip)]
    // pub time_value2: Option<u64>, // not part of EAF spec, for populating/editing time order
}

impl Default for AlignableAnnotation {
    fn default() -> Self {
        Self {
            annotation_id: "a1".to_owned(),
            ext_ref: None,
            lang_ref: None,
            cve_ref: None,
            time_slot_ref1: "ts1".to_owned(),
            time_slot_ref2: "ts2".to_owned(),
            annotation_value: AnnotationValue::default(),
            tier_id: None,
            time_value1: None,
            time_value2: None
        }
    }
}

/// Reference annotation. An annotation type found in reference tiers,
/// which refers to an annotation in its parent tier. Has no explicit
/// time slot references. These must derived via its parent annotation/s.
/// `AnnotationDocument.derive()` will do this, and populate,
/// `tier_id`, `time_slot_val1`,`time_slot_val2`, and `main_annotation`
/// (annotation ID for the corresponding alignable annotation in
/// the main tier for this hierarchy.)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "REF_ANNOTATION")] // causes double annotation tags
pub struct RefAnnotation{
    /// Attribute annotation ID.
    pub annotation_id: String,
    /// Attribute external reference.
    pub ext_ref: Option<String>,
    /// Attribute language reference.
    pub lang_ref: Option<String>,
    /// Attribute CVE reference.
    pub cve_ref: Option<String>,
    /// Attribute annotation reference in parent tier.
    pub annotation_ref: String,
    /// Attribute previous annotation for tokenized tiers.
    pub previous_annotation: Option<String>, // only for ref annotation?
    /// Child node annotation value.
    pub annotation_value: AnnotationValue,
    /// Tier ID.
    #[serde(skip)]
    pub tier_id: Option<String>, // not part of EAF spec
    /// Timeslot start value in milliseconds.
    #[serde(skip)]
    pub time_value1: Option<i64>, // not part of EAF spec, for populating/editing time order
    /// Timeslot end value in milliseconds.
    #[serde(skip)]
    pub time_value2: Option<i64>, // not part of EAF spec, for populating/editing time order
    // /// Timeslot start value in milliseconds.
    // #[serde(skip)]
    // pub time_value1: Option<u64>, // not part of EAF spec, for populating/editing time order
    // /// Timeslot end value in milliseconds.
    // #[serde(skip)]
    // pub time_value2: Option<u64>, // not part of EAF spec, for populating/editing time order
    /// Annotation ID for the corresponding alignable annotation in the main tier.
    #[serde(skip)]
    pub main_annotation: Option<String>, // not part of EAF spec, for populating/editing time order
}

impl Default for RefAnnotation {
    fn default() -> Self {
        Self {
            annotation_id: "a2".to_owned(),
            ext_ref: None,
            lang_ref: None,
            cve_ref: None,
            annotation_ref: "a1".to_owned(),
            previous_annotation: None,
            annotation_value: AnnotationValue::default(),
            tier_id: None,
            time_value1: None,
            time_value2: None,
            main_annotation: None
        }
    }
}

// Perhaps use this instead:
// https://users.rust-lang.org/t/serde-deserializing-a-vector-of-enums/51647
/// Annotation value.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd)]
#[serde(rename = "ANNOTATION_VALUE")]
pub struct AnnotationValue(String);

impl Default for AnnotationValue {
    fn default() -> Self {
        Self("".to_owned())
    }
}

impl AnnotationValue {
    /// Returns an iterator over the characters for the annotation value.
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }
    
    /// Returns character count for the annotation value.
    pub fn len(&self) -> usize {
        self.0.chars().count()
    }
    
    /// Splits the annotation value on white space or specified pattern.
    pub fn split(&self, pattern: Option<&str>) -> Vec<&str> {
        match pattern {
            Some(p) => self.0.split(p).collect(),
            None => self.0.split_whitespace().collect(),
        }
    }

    /// Replace string pattern for annotation value.
    pub fn replace(&self, from: &str, to: &str) -> Self {
        Self(self.0.replace(from, to))
    }

    /// Mutably replace string pattern for annotation value.
    pub fn replace_mut(&mut self, from: &str, to: &str) {
        self.0 = self.0.replace(from, to)
    }
}
