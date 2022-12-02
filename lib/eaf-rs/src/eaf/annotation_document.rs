//! The core data structure for a deserialized EAF file.
//! Example:
//! ```
//! use eaf_rs::annotation_document::AnnotationDocument;
//! fn main() -> std::io::Result<()> {
//!     let path = std::path::Path::new("MYEAF.eaf");
//!     let eaf = AnnotationDocument::deserialize(&path, true)?;
//!     println!("{:#?}", eaf);
//!     Ok(())
//! }
//! ```
//!
//! Note that some methods expect `AnnotationDocument::index()` and `AnnotationDocument::derive()`
//! to be called before they are run. This is done automatically for most methods and on deserialization.
//! `AnnotationDocument::index()` indexes the EAF speeding up many "getter" methods,
//! whereas and `AnnotationDocument::derive()` derives values such as time values
//! for annotation boundaries and sets these directly at the annotation level to make them more independent.

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::{
    Annotation,
    Constraint,
    StereoType,
    ControlledVocabulary,
    EafError,
    Header,
    Index,
    JsonEaf,
    Language,
    LexiconRef,
    License,
    LinguisticType,
    Locale,
    Tier,
    TimeOrder
};

/// Returns "unspecified" as `String`
/// To get around quick-xml not adding attributes with
/// empty strings ("") as value.
pub fn unspecified() -> String {
    "unspecified".to_owned()
}

/// Returns local date and time as ISO8601-formatted string.
pub fn today() -> String {
    // chrono::Local::now()
    // .format("%+") // %+ = ISO8601 formatted string
    // .to_string()
    time::OffsetDateTime::now_utc().to_string()
}

/// Return path as string with optional prefix, e.g. 'file://' for EAF media URLs.
///
/// Currently only handles Unicode paths. Always returns a string, but failed
/// `Path::file_name_()` unwraps return "NONE" as a dummy value.
pub fn path_to_string(path: &Path, prefix: Option<&str>, filename_only: bool) -> String {
    let path_str = match filename_only {
        true => path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or("NONE".to_owned()),
        false => path.as_os_str().to_string_lossy().to_string(),
    };
    format!("{}{}", prefix.unwrap_or(""), path_str)
}

/// Used for methods and function where
/// scope is important, e.g. token
/// or ngram stats.
pub enum Scope {
    /// Depending on usage,
    /// contained value can be
    /// e.g. internal annotation ID
    /// or tier ID.
    Annotation(Option<String>),
    /// Depending on usage,
    /// contained value can be
    /// e.g. internal annotation ID
    /// or tier ID.
    Tier(Option<String>),
    File
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(rename = "ANNOTATION_DOCUMENT")]
/// Core data structure for an ELAN annotation format file (`.eaf`).
/// De/Serializable. Make sure to validate output, since breaking changes
/// were introduced in EAF v2.8. E.g. valid EAF v2.7 documents with
/// controlled vocabularies do not validate against EAF v2.8+
/// schemas. Currently fails on ELAN Template Files (`.etf`),
/// since ETF is a simplified form of the EAF standard that
/// does not expect some elements that are required for an `.eaf` file.
///
/// Example:
/// ```
/// use eaf_rs::annotation_document::AnnotationDocument;
/// fn main() -> std::io::Result<()> {
///     let path = std::path::Path::new("MYEAF.eaf");
///     let eaf = AnnotationDocument::deserialize(&path, true)?;
///     println!("{:#?}", eaf);
///     Ok(())
/// }
/// ```
pub struct AnnotationDocument {
    #[serde(rename = "xmlns:xsi")]
    xmlns_xsi: String,
    #[serde(rename = "xsi:noNamespaceSchemaLocation")]
    xsi_nonamespaceschemalocation: String,
    #[serde(default = "unspecified")] // String "unspecified", quick-xml ignores attr with empty string (bug?)
    /// EAF author attribute.
    pub author: String, // required even if only ""
    #[serde(default = "today")] // fn that returns current ISO8601 datetime as String
    /// EAF ISO8601 date time attribute.
    pub date: String,
    /// EAF format attribute, e.g. "3.0".
    pub format: String,
    /// EAF version attribute, e.g. "3.0".
    pub version: String,
    /// EAF license attribute.
    pub license: Option<License>,
    /// EAF header. Contains media paths.
    pub header: Header,
    /// EAF time slots, used to specify annotation boundaries (defaults to milliseconds).
    /// Note that time values are optional.
    pub time_order: TimeOrder,
    #[serde(rename = "TIER", default)]
    /// EAF tier. Contains annotations.
    pub tiers: Vec<Tier>,
    #[serde(rename = "LINGUISTIC_TYPE", default)]
    /// EAF linguistic type. Referred to in tier attributes and specifies
    /// e.g. if the tier is time-alignable.
    pub linguistic_types: Vec<LinguisticType>,
    #[serde(rename = "LOCALE", default)]
    /// EAF locale.
    pub locales: Vec<Locale>,
    #[serde(rename = "LANGUAGE", default)]
    /// EAF languages.
    pub languages: Vec<Language>,
    #[serde(rename = "CONSTRAINT", default)]
    /// EAF constraints.
    pub constraints: Vec<Constraint>,
    #[serde(rename = "CONTROLLED_VOCABULARY", default)]
    /// EAF controlled vocabularies.
    pub controlled_vocabularies: Vec<ControlledVocabulary>,
    #[serde(rename = "LEXICON_REF", default)]
    /// EAF lexicon references.
    pub lexicon_refs: Vec<LexiconRef>,
    #[serde(skip)]
    /// Not part of EAF specification. Toggle to check whether annotations have
    /// e.g. derived time slot values set.
    derived: bool,
    #[serde(skip)]
    /// Not part of EAF specification. Index with mappings for
    /// e.g. annotation ID to time slot values.
    pub index: Index, // should ideally be 'pub(crate): Index'
    #[serde(skip)]
    /// Not part of EAF specification. Toggle to check whether `AnnotationDocument`
    /// is indexed.
    indexed: bool,
}

impl Default for AnnotationDocument {
    fn default() -> Self {
        Self {
            xmlns_xsi: "xmlns:xsi".to_owned(),
            xsi_nonamespaceschemalocation: "xsi:noNamespaceSchemaLocation".to_owned(),
            author: unspecified(), // required so must fill with e.g. "" as default if no value
            date: today(),
            format: "3.0".to_owned(),
            version: "3.0".to_owned(),
            license: None,
            header: Header::default(),
            time_order: TimeOrder::default(),
            tiers: vec![Tier::default()],
            linguistic_types: vec![LinguisticType::default()],
            locales: vec![Locale::default()],
            languages: Vec::new(),
            constraints: Vec::new(),
            controlled_vocabularies: Vec::new(),
            lexicon_refs: Vec::new(),
            derived: false,
            index: Index::default(),
            indexed: false,
        }
    }
}

impl AsRef<AnnotationDocument> for AnnotationDocument {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl AnnotationDocument {
    /// Deserialize EAF XML-file.
    /// If `derive` is set, all annotations will have the following derived and set:
    /// - Explicit time values.
    /// - Tier ID.
    /// - Main annotation ID for referred annotations (i.e. the ID for the alignable annotation in the main tier of the hierarchy).
    ///
    /// While `derive` is convenient if working on a single file,
    /// parsing will take \~2x the time.
    /// Set to `false` to gain some speed if you are batch processing,
    /// and all you want to do is to e.g. scrub media paths for multiple EAF-files.
    pub fn deserialize(path: &Path, derive: bool) -> Result<Self, EafError> {
        // Let Quick XML use serde to deserialize
        let mut eaf: AnnotationDocument = quick_xml::de::from_str(&std::fs::read_to_string(path)?)
            .map_err(|e| EafError::QuickXMLDeError(e))?;

        // index file first...
        eaf.index();

        // ...then derive (uses index)
        if derive {
            // Could return Eaf without deriving if it fails,
            // with the caveat that the serialized file
            // may not work in ELAN.
            // Tested pympi's Eaf.merge(), which merged tiers
            // and generated EAFs that validate against schema,
            // but do not run in ELAN, since there are
            // "ref annotations" referring to non-existing
            // "main annotations".
            eaf.derive()?;
        }

        Ok(eaf)
    }

    /// Serialize `AnnotationDocument` to string.
    pub fn serialize(&self) -> Result<String, EafError> {
        let mut eaf = self.to_owned(); // better to take &mut self as arg...?
        if eaf.author == "" {
            eaf.author = unspecified() // quick-xml ignores attr with empty string ""
        }
        for lexref in eaf.lexicon_refs.iter_mut() {
            if lexref.url == "" {
                lexref.url = unspecified()
            }
        }

        // Should already be set for deserialized EAF:s.
        eaf.set_ns();

        Ok([
                r#"<?xml version="1.0" encoding="UTF-8"?>"#, // no decl added to string...
                &quick_xml::se::to_string(&eaf).map_err(|e| EafError::QuickXMLDeError(e))?,
            ]
            .join("\n")
            // Ugly workaround below for fixing quick-xml/serde enum bug,
            // adding line breaks, and indenting output (default is a single line).
            .replace("<AlignableAnnotation>", "") // due to quick-xml bug duplicating tags for struct -> enum
            .replace("</AlignableAnnotation>", "") // due to quick-xml bug duplicating tags for struct -> enum
            .replace("<RefAnnotation>", "") // due to quick-xml bug duplicating tags for struct -> enum -> struct
            .replace("</RefAnnotation>", "") // due to quick-xml bug duplicating tags for struct -> enum -> struct
            .replace("<CvEntry>", "") // due to quick-xml bug duplicating tags for struct -> enum -> struct
            .replace("</CvEntry>", "") // due to quick-xml bug duplicating tags for struct -> enum -> struct
            .replace("<CvEntryMl>", "") // due to quick-xml bug duplicating tags for struct -> enum -> struct
            .replace("</CvEntryMl>", "") // due to quick-xml bug duplicating tags for struct -> enum -> struct
            .replace(
                "<ANNOTATION_VALUE></ANNOTATION_VALUE>",
                "\n\t\t\t\t<ANNOTATION_VALUE/>",
            )
            .replace("><", ">\n<") // add line breaks
            .replace("<ANNOTATION_VALUE>", "\t\t\t\t<ANNOTATION_VALUE>")
            .replace("<HEADER", "\t<HEADER")
            .replace("</HEADER", "\t</HEADER")
            .replace("<MEDIA_DESCRIPTOR", "\t\t<MEDIA_DESCRIPTOR")
            .replace("<PROPERTY", "\t\t<PROPERTY")
            .replace("<TIME_ORDER", "\t<TIME_ORDER")
            .replace("</TIME_ORDER", "\t</TIME_ORDER")
            .replace("<TIME_SLOT", "\t\t<TIME_SLOT")
            .replace("<TIER", "\t<TIER")
            .replace("</TIER", "\t</TIER")
            .replace("<ANNOTATION>", "\t\t<ANNOTATION>")
            .replace("</ANNOTATION>", "\t\t</ANNOTATION>")
            .replace("<ALIGNABLE_ANNOTATION", "\t\t\t<ALIGNABLE_ANNOTATION")
            .replace("</ALIGNABLE_ANNOTATION", "\t\t\t</ALIGNABLE_ANNOTATION")
            .replace("<REF_ANNOTATION", "\t\t\t<REF_ANNOTATION")
            .replace("</REF_ANNOTATION", "\t\t\t</REF_ANNOTATION")
            .replace("<LINGUISTIC_TYPE", "\t<LINGUISTIC_TYPE")
            .replace("<CONSTRAINT", "\t<CONSTRAINT")
            .replace("<LANGUAGE", "\t<LANGUAGE")
            .replace("<LOCALE", "\t<LOCALE")
            .replace("<LEXICON_REF", "\t<LEXICON_REF")
            .replace("<CONTROLLED_VOCABULARY", "\t<CONTROLLED_VOCABULARY")
            .replace("</CONTROLLED_VOCABULARY", "\t</CONTROLLED_VOCABULARY")
            .replace("<CV_ENTRY", "\t\t<CV_ENTRY")
            .replace("<CV_ENTRY_ML", "\t\t<CV_ENTRY_ML")
            .replace("<CVE_VALUE", "\t\t<CVE_VALUE")
            .replace("<DESCRIPTION", "\t\t<DESCRIPTION")
        )
    }

    /// Serializes the full AnnotationDocument structure to JSON as is.
    /// `simple` set to `true` serializes to a simplified JSON structure.
    pub fn to_json(&self, simple: bool) -> serde_json::Result<String> {
        match simple {
            true => serde_json::to_string(&JsonEaf::from(self)),
            false => serde_json::to_string(&self),
        }
    }

    /// Set EAF XML namespaces.
    fn set_ns(&mut self) {
        self.xmlns_xsi = "http://www.w3.org/2001/XMLSchema-instance".to_owned();
        self.xsi_nonamespaceschemalocation =
            format!("http://www.mpi.nl/tools/elan/EAFv{}.xsd", self.version);
    }

    /// Serialize and write file to disk.
    /// Return `Ok(true)` if file was written to disk,
    /// otherwise either `Ok(false)` (user aborted write, but no errors),
    /// or `Err<EafError>` for IO/EAF errors.
    pub fn write(&self, path: &Path) -> Result<bool, EafError> {
        /// Acknowledgement, overwriting file.
        fn acknowledge(message: &str) -> std::io::Result<bool> {
            loop {
                print!("(!) {} (y/n): ", message);
                std::io::stdout().flush()?;
                let mut overwrite = String::new();
                let _ = std::io::stdin().read_line(&mut overwrite)?;

                return match overwrite.to_lowercase().trim() {
                    "y" | "yes" => Ok(true),
                    "n" | "no" => Ok(false),
                    _ => {
                        println!("Enter y/yes or n/no");
                        continue;
                    }
                };
            }
        }

        let content = self.serialize()?;

        let write = if path.exists() {
            acknowledge(&format!("{} already exists. Overwrite?", path.display()))?
        } else {
            true
        };

        if write {
            let mut outfile = File::create(&path)?;
            outfile.write_all(content.as_bytes())?;
        }

        Ok(write)
    }

    /// Generate new AnnotationDocument with a single tier created from
    /// tuples in the form `(annotation value, start time ms, end time ms)`.
    pub fn from_values(
        values: &[(String, i64, i64)],
        tier_id: Option<&str>,
    ) -> Result<Self, EafError> {
        let mut eaf = AnnotationDocument::default();
        let tier_id_str = tier_id.unwrap_or("default");
        eaf.set_tier_id(tier_id_str, "default")?;
        eaf.index(); // adds changed tier_id_str to index

        let mut count = 1;

        for (i, (value, ts_val1, ts_val2)) in values.iter().enumerate() {
            let mut a = Annotation::alignable(
                value,
                &format!("a{}", i + 1),
                &format!("ts{}", count + i),
                &format!("ts{}", count + i + 1),
            );

            count += 1;

            // Set AlignableAnnotation.
            a.set_ts_val(Some(*ts_val1), Some(*ts_val2));

            eaf.add_annotation(&a, tier_id_str, false)?;
        }

        eaf.index();
        eaf.derive()?;

        Ok(eaf)
    }

    /// Derives and sets the following in all `Annotation` structs:
    /// - Time values in milliseconds.
    /// - Annotation ID for referred main annotation (may or may not be the same as annotation_ref)
    /// - Tier ID
    ///
    /// Mostly for internal use. Also makes annotations less dependent,
    /// since they now contain explicit time slot values etc.
    pub fn derive(&mut self) -> Result<(), EafError> {
        // copy since otherwise error is raised
        // better solution would be nice
        let eaf_copy = self.to_owned();

        // Iter mut over self...
        for tier in self.tiers.iter_mut() {
            for annotation in tier.annotations.iter_mut() {
                // ...check if annotion is ref annotation...
                let (ref1, ref2) = match annotation.is_ref() {
                    true => {
                        // ...then use the copy for deriving main annotation for ref annotations.
                        let ma = eaf_copy.main_annotation(&annotation.id()).ok_or(
                            EafError::MissingMainAnnotation((annotation.id(), annotation.ref_id())),
                        )?;

                        // Set main annotion ID for ref annotation...
                        annotation.set_main(&ma.id());

                        // ...then get annotation ID for main annotation.
                        ma.ts_ref()
                            .ok_or(EafError::MissingTimeslotRef(annotation.id()))?
                    }

                    // Raise error if annotation in main tier returns no time slot references.
                    false => annotation
                        .ts_ref()
                        .ok_or(EafError::MissingTimeslotRef(annotation.id()))?,
                };

                let val1 = eaf_copy.ts_val(&ref1);
                let val2 = eaf_copy.ts_val(&ref2);
                annotation.set_ts_val(val1, val2);
                annotation.set_tier_id(&tier.tier_id);
            }
        }

        self.derived = true; // set check for .derive()

        Ok(())
    }

    /// Indexes the ELAN-file with the following mappings:
    /// - `a2t`: Annotation ID to tier ID
    /// - `a2ref`: Annotation ID to ref annotation ID
    /// - `t2a`: Tier ID to list of annotation ID:s
    /// - `t2ref`: Tier ID to ref tier ID
    /// - `id2ts`: Time slot ID to time slot value
    /// - `ts2id`: Time slot value to Time slot ID
    /// - `a2ts`: Annotation ID to time slot id/ref tuple, `(time_slot_ref1, time_slot_ref2)`.
    /// - `a2idx`: Annotation ID to `(idx1, idx2)` in `AnnotationDocument.tiers[idx1].annotations[idx2]`
    /// - `t2idx`: Tier ID to `idx` in `AnnotationDocument.tiers[idx]`
    ///
    /// Speeds up many "getter" methods, such as finding cross referenced annotations,
    /// time values for referred annotations etc. Done automatically on deserialization.
    /// Re-run as necessary, after external edit etc. Automatic for internal methods,
    /// such as adding an annotation or a tier.
    pub fn index(&mut self) {
        let mut a2t: HashMap<String, String> = HashMap::new();
        let mut a2ref: HashMap<String, String> = HashMap::new();
        let mut t2a: HashMap<String, Vec<String>> = HashMap::new();
        let mut t2ref: HashMap<String, String> = HashMap::new();
        let mut a2ts: HashMap<String, (String, String)> = HashMap::new(); // Annotation ID -> time slot ref1, ref2
        let mut a2idx: HashMap<String, (usize, usize)> = HashMap::new(); // Annotation ID -> tier idx, annot idx
        let mut t2idx: HashMap<String, usize> = HashMap::new(); // Tier ID -> tier idx

        self.tiers.iter().enumerate().for_each(|(idx_t, t)| {
            // Tier ID -> tier idx in self.tiers
            t2idx.insert(t.tier_id.to_owned(), idx_t);

            // Tier ID -> Ref tier ID
            if let Some(t_id) = t.parent_ref.to_owned() {
                t2ref.insert(t.tier_id.to_owned(), t_id);
            }

            // Used for Tier ID -> [Annotation ID, ...]
            let mut a_id: Vec<String> = Vec::new();

            t.annotations.iter().enumerate().for_each(|(idx_a, a)| {
                let id = a.id();

                // Annotation ID -> (Tier index, Annotation index)
                a2idx.insert(id.to_owned(), (idx_t, idx_a));

                // Annotation ID -> Annotation ref ID
                if let Some(ref_id) = a.ref_id() {
                    a2ref.insert(id.to_owned(), ref_id);
                };

                // Annotation ID -> Tier ID
                a2t.insert(id.to_owned(), t.tier_id.to_owned());

                // Annotation ID -> (time slot ref 1, time slot ref2)
                if let Some((ref1, ref2)) = a.ts_ref() {
                    a2ts.insert(id.to_owned(), (ref1, ref2));
                }

                a_id.push(id);
            });

            // Tier ID -> [Annotation ID, ...]
            t2a.insert(t.tier_id.to_owned(), a_id);
        });

        self.index = Index {
            a2t,
            a2ref,
            t2a,
            t2ref,
            ts2tv: self.time_order.index(),
            tv2ts: self.time_order.index_rev(),
            a2ts,
            a2idx,
            t2idx,
        };

        self.indexed = true;
    }

    /// Generates empty ELAN-file with specified media files linked.
    pub fn with_media(media_paths: &[PathBuf]) -> Self {
        let mut eaf = Self::default();
        for path in media_paths.iter() {
            eaf.add_media(path, None);
        }
        eaf
    }

    /// Links specified media files.
    pub fn with_media_mut(&mut self, media_paths: &[PathBuf]) {
        for path in media_paths.iter() {
            self.add_media(path, None);
        }
    }

    /// Adds new media path to header as a new media descriptor.
    pub fn add_media(&mut self, path: &Path, extracted_from: Option<&str>) {
        self.header.add_media(path, extracted_from)
    }

    /// Removes specific media file from header if it is set.
    /// Matches on file name, not the entire path.
    pub fn remove_media(&mut self, path: &Path) {
        self.header.remove_media(path)
    }

    /// Scrubs absolute media paths in header, and optionally relative ones.
    /// Absolute paths sometimes contain personal information, such as user name.
    /// If both paths are scrubbed media files have to be completely re-linked in ELAN.
    pub fn scrub_media(&mut self, keep_filename: bool) {
        self.header.scrub_media(keep_filename)
    }

    /// Returns all media paths as string tuples,
    /// `(media_url, relative_media_url)`.
    /// `media_url` is optional.
    pub fn media_paths(&self) -> Vec<(String, Option<String>)> {
        self.header.media_paths()
    }

    /// Returns all linked absolute media paths as strings.
    pub fn media_abs_paths(&self) -> Vec<String> {
        self.header.media_abs_paths()
    }

    /// Returns all linked relative media paths (optional value) as strings.
    pub fn media_rel_paths(&self) -> Vec<String> {
        self.header.media_rel_paths()
    }

    /// Returns a hashmap (name: value) of all properties in header.
    /// Key: name (`NAME` attribute)
    /// Value: value (element text value)
    pub fn properties(&self) -> HashMap<String, String> {
        self.header
            .properties
            .iter()
            .map(|p| (p.name.to_owned(), p.value.to_owned()))
            .collect()
    }

    /// Retrurns hashmap of all time slots.
    /// - Key: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    /// - Value: timeslot value in milliseconds (may be `None`).
    pub fn timeslots(&self) -> HashMap<String, Option<i64>> {
        if self.indexed {
            self.index.ts2tv.to_owned()
        } else {
            self.time_order.index()
        }
    }

    /// Reverse lookup table for time slot values.
    /// - Key: timeslot value in milliseconds.
    /// - Value: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    ///
    /// Only includes time slots with a time value set.
    pub fn timeslots_rev(&self) -> HashMap<i64, String> {
        if self.indexed {
            self.index.tv2ts.to_owned()
        } else {
            self.time_order.index_rev()
        }
    }

    /// Returns the time slot ID for specified time slot value.
    /// Note that a time slot value is not required according to the EAF specification.
    ///
    /// Requires that `AnnotationDocument.index()` has been run.
    pub fn ts_id(&self, ts_val: i64) -> Option<String> {
        self.index.tv2ts.get(&ts_val).cloned()
    }

    /// Returns the time value if one is specified for the time slot id,
    /// `None` otherwise, or if there are no time slots.
    /// Note that a time slot value is not required according to the EAF specification.
    ///
    /// Requires that `AnnotationDocument.index()` has been run.
    pub fn ts_val(&self, ts_id: &str) -> Option<i64> {
        *self.index.ts2tv.get(ts_id)?
    }

    /// Returns the smallest time slot value.
    /// Does not provide media boundaries,
    /// only the first time slot with a time value.
    pub fn ts_min_val(&self) -> Option<i64> {
        if self.indexed {
            self.index.tv2ts.keys().min().cloned() // use ts2id.keys() to ensure value
        } else {
            self.time_order.min_val()
        }
    }

    /// Returns the largest time slot value.
    /// Does not provide media boundaries,
    /// only the last time slot with a time value.
    pub fn ts_max_val(&self) -> Option<i64> {
        // pub fn ts_max(&self) -> Option<u64> {
        if self.indexed {
            self.index.tv2ts.keys().max().cloned() // use ts2id.keys() to ensure value
        } else {
            self.time_order.max_val()
        }
    }

    /// Shift all time values with the specified value in milliseconds.
    /// `allow_negative` ignores if the resulting time values are negative,
    /// otherwise `EafError::ValueTooSmall(time_value)` is raised.
    pub fn shift(&mut self, shift_ms: i64, allow_negative: bool) -> Result<(), EafError> {
        self.time_order.shift(shift_ms, allow_negative)
    }

    /// Merges two ELAN-files if possible. This is done with the assumption that both
    /// files link the same media files and that tiers with identical tier ID:s have the same attributes,
    /// which is important for tier hierarchy, linguistic types etc. This is up to the user
    /// to ensure.
    ///
    /// Caveats:
    /// - Linked files and any tier attributes will be inherited from the first file only.
    /// - Time slots without a time value will be discarded.
    pub fn merge(paths: &[PathBuf; 2]) -> Result<Self, EafError> {
        // let mut eaf1 = AnnotationDocument::deserialize(&paths[0], true)?;
        // let eaf1_a_len = eaf1.a_len();
        // let eaf1_ts_len = eaf1.time_order.len();

        // // remap eaf2 annotation id:s + time slot id:s to start after eaf1
        // // at this point there may be duplicate annotations and time slots
        // let mut eaf2 = AnnotationDocument::deserialize(&paths[1], true)?;
        // eaf2.remap(Some(eaf1_a_len+1), Some(eaf1_ts_len+1))?;

        // eaf1.time_order.join(&eaf2.time_order);

        // for tier2 in eaf2.tiers.iter() {
        //     if let Some(tier1) = eaf1.get_tier_mut(&tier2.tier_id) {
        //         // Either join tiers if both have same tier id or...
        //         tier1.join(tier2, true);
        //     } else {
        //         // ... add as new tier (adds timeslots as well)
        //         eaf1.add_tier(Some(tier2.to_owned()))?
        //     }
        // }

        let mut eaf1 = AnnotationDocument::deserialize(&paths[0], true)?;
        let eaf1_a_len = eaf1.a_len();
        let eaf1_ts_len = eaf1.time_order.len();
        let mut eaf2 = AnnotationDocument::deserialize(&paths[1], true)?;
        eaf2.remap(Some(eaf1_a_len + 1), Some(eaf1_ts_len + 1))?;

        // Check for matching tier ID:s: merge if match, add as new tier otherwise
        for tier2 in eaf2.tiers.iter() {
            if let Some(tier1) = eaf1.get_tier_mut(&tier2.tier_id) {
                println!("Merging {} & {}", tier1.tier_id, tier2.tier_id);
                tier1.merge(tier2, false)?;
            } else {
                println!("Adding {}", tier2.tier_id);
                eaf1.add_tier(Some(tier2.to_owned()), None)?;
            }
        }

        // let timeslots = TimeOrder::

        eaf1.remap(None, None)?;

        // TODO comparing annotation overlaps, duplication: dedup, by somehow comparing annotation + time values
        // TODO comparing timeslots: eaf1 + eaf2 *must* be derived, then use annotations' derived ts values as basis for what to keep
        // TODO fix overlapping annotation boundaries, option to join these

        Ok(eaf1)
    }

    // fn sort_annotations(&mut self, join: bool) -> Result<(), EafError> {
    //     if !self.indexed {self.index()}
    //     if !self.derived {self.derive()?} // adds time values to annotations for sort

    //     Ok(())
    // }

    /// Match annotation values against a string.
    /// Returns a vec with tuples: `(Annotation Index, Tier ID, Annotation ID, Annotation value)`.
    pub fn query(&self, pattern: &str, ignore_case: bool) -> Vec<(usize, String, String, String, Option<String>)> {
        // (Annotation Index, Tier ID, Annotation ID, Annotation value)
        self.tiers
            // .iter()
            .par_iter()
            .filter_map(|t| {
                let results = t.query(pattern, ignore_case);
                if results.is_empty() {
                    None
                } else {
                    Some(results)
                }
            })
            .flatten()
            .collect()
    }

    /// Match annotation values against a regular expression.
    /// Returns a vec with tuples: `(Annotation Index, Tier ID, Annotation ID, Annotation value)`.
    pub fn query_rx(&self, regex: &Regex) -> Vec<(usize, String, String, String, Option<String>)> {
        // (Annotation Index, Tier ID, Annotation ID, Annotation value)
        self.tiers
            // .iter()
            .par_iter()
            .filter_map(|t| {
                let results = t.query_rx(regex);
                if results.is_empty() {
                    None
                } else {
                    Some(results)
                }
            })
            .flatten()
            .collect()
    }

    /// Returns all words/tokens in ELAN-file. Does not work with languages
    /// that do not use white space to delimit words/tokens.
    /// Optionally, `strip_prefix` and `strip_suffix` are strings containing characters
    /// that will be stripped, so that for `strip_prefix = Some("<*")`: "<hi", "*hi", "hi"
    /// all become "hi" in the output.
    pub fn tokens(
        &self,
        strip_prefix: Option<&str>,
        strip_suffix: Option<&str>,
        unique: bool,
        ignore_case: bool,
    ) -> Vec<String> {
        let mut tokens: Vec<String> = self.tiers
            // .iter()
            .par_iter()
            .map(|t| t.tokens(strip_prefix, strip_suffix, unique, ignore_case))
            .flatten()
            .collect();

        tokens.sort();

        if unique {
            tokens.dedup();
        }

        tokens
    }

    /// Naive implementation of ngram. Checks lower case variants only.
    /// Optionally remove regex matches, before checking. Only usable
    /// for scripts which uses whitespace as a delimiter
    /// (i.e. CJK is out of scope for this implementation).
    /// 
    /// Scope:
    /// - `Scope::Tier(Some(TIER_ID))` compiles ngrams across annotation boundaries
    /// - `Scope::Annotation(Some(TIER_ID))` compiles ngrams across annotation boundaries
    /// - `Scope::File` compiles ngrams across annotation and tier boundaries and combines the result
    /// 
    /// Returns `HashMap<ngram, count>`.
    pub fn ngram(&self, size: usize, regex_remove: Option<&Regex>, scope: Scope) -> HashMap<String, usize> {
        let mut ngrams: HashMap<String, usize> = HashMap::new();
        match scope {
            Scope::Annotation(tier_id) => {
                if let Some(t_id) = tier_id {
                    match self.get_tier(&t_id) {
                        Some(t) => return t.ngram(size, regex_remove, false),
                        None => return HashMap::new()
                    };
                } else {
                    return HashMap::new()
                }
            },
            Scope::Tier(tier_id) => {
                if let Some(t_id) = tier_id {
                    match self.get_tier(&t_id) {
                        Some(t) => return t.ngram(size, regex_remove, true),
                        None => return HashMap::new()
                    };
                } else {
                    return HashMap::new()
                }
            },
            Scope::File => {
                self.tiers.iter()
                    .for_each(|t| ngrams.extend(t.ngram(size, regex_remove, true)))
            },
        }

        ngrams
    }

    /// Returns total number of annotations in EAF.
    pub fn a_len(&self) -> usize {
        self.tiers.iter()
            .map(|t| t.len())
            .sum()
    }

    /// Returns number of tiers in EAF.
    pub fn t_len(&self) -> usize {
        self.tiers.len()
    }

    /// Pushes a time slot to time order as last item.
    /// Ensures the time slots does not exist, but does not verify
    /// that specified time slot ID/value is greater than exisiting ones.
    pub fn add_timeslot(&mut self, id: &str, val: Option<i64>, index: bool) {
        // pub fn push_timeslot(&mut self, id: &str, val: Option<u64>, index: bool) {
        self.time_order.add(id, val);
        if index {
            self.index()
        } else {
            self.indexed = false
        }
    }

    /// Returns a copy of all annotations in ELAN-file or for specified tier.
    pub fn annotations(&self, tier_id: Option<&str>) -> Result<Vec<Annotation>, EafError> {
        // clone to avoid having to pass &mut self for index+derive...
        let mut eaf = self.to_owned();

        if !eaf.indexed {
            eaf.index()
        }; // needed for derive()
        if !eaf.derived {
            eaf.derive()?
        };

        if let Some(id) = tier_id {
            eaf.tiers.into_iter()
                .find(|t| t.tier_id == id)
                .map(|t| t.annotations)
                .ok_or(EafError::InvalidTierId(id.to_owned()))
            // or just Option -> None?
        } else {
            Ok(eaf.tiers.into_iter()
                .flat_map(|t| t.annotations)
                .collect())
        }
    }

    /// Verifies that:
    /// - time slot reference ID:s are valid for alignable annotations.
    /// - reference annotation ID:s are valid for referred annotations.
    /// - reference tier ID:s are valid for referred tiers.
    /// Does not raise errors, only print stats.
    pub fn validate(&mut self, _verbose: bool) {
        // if !self.indexed {self.index()}

        // let mut t_orphans: Vec<(String, String)> = Vec::new(); // (tier ID, ref tier ID)
        // let mut a_orphans: Vec<String> = Vec::new();
        // let mut ts_orphans: Vec<String> = Vec::new();

        // self.tiers.iter()
        //     .for_each(|t| {
        //         if let Some(t_id) = &t.parent_ref {
        //             if !self.exists(t_id).0 {
        //                 t_orphans.push((t.tier_id.to_owned(), t_id.to_owned()));
        //             }
        //         }
        //     })
        unimplemented!()
    }

    /// Remove time slots with duplicate time slot values,
    /// and remap time slot references in annotations.
    /// Does not deduplicate time slots with an empty value,
    /// but their time slot ID may change.
    pub fn dedup(&mut self) {
        // NOTE Perhaps not necessary since some eaf (v2.7) generated by ELAN have duplicate time slot values.
        //      Still, the issue may be that since finding min/max time slot values return first match,
        //      max value found may precede a lower time slot value. Deduping + remapping should help
        //      avoid incorrect boundaries in those cases.
        unimplemented!()
    }

    /// Remaps time slots and annotation ID:s so that they start on 1 or, optionally,
    /// specified annotation ID and/or time slot ID.
    /// For use with e.g. `filter()`, where parts of the EAF have been filtered out.
    /// Resets ID counters for timeslots and annotations to start on 1.
    /// Relabels and remaps the following numerical identifiers:
    /// - annotation ID:s for all annotations.
    /// - references to annotation ID:s for referred annotations.
    /// - time slot ID:s.
    /// - references to time slot ID:s for aligned annotations.
    pub fn remap(&mut self, a_idx: Option<usize>, ts_idx: Option<usize>) -> Result<(), EafError> {
        if !self.indexed {
            self.index()
        } // does not work for merged tiers if contains duped annot id:s.

        // 1. Remap time slots and create lookup table for current time slot ID -> new time slot ID
        let ts_map = self.time_order.remap(ts_idx);

        // 2. Create lookup table for current annotation ID -> new annotation ID
        let start_a_id = a_idx.unwrap_or(0);
        let a_map: HashMap<String, String> = self.annotations(None)?
            .iter()
            .enumerate()
            .map(|(i, a)| (a.id(), format!("a{}", start_a_id + i + 1)))
            .collect();

        // 3. Remap annotation ID and reference annotation ID.
        for tier in self.tiers.iter_mut() {
            for annotation in tier.iter_mut() {
                let annotation_id = annotation.id();

                // Look up and set new annotation ID. Required for all annotations.
                let new_a_id = a_map.get(&annotation_id)
                    .ok_or(EafError::InvalidAnnotationId(annotation_id.to_owned()))?;
                annotation.set_id(&new_a_id);
                // println!("A ID  {} -> {}", annotation_id, new_a_id);

                // Look up and set new time slot references. Required for alignable annotations.
                if let Some((ts1, ts2)) = self.index.a2ts.get(&annotation_id) {
                    let new_ts1 = ts_map.get(ts1)
                        .ok_or(EafError::InvalidTimeSlotId(ts1.to_owned()))?;
                    let new_ts2 = ts_map.get(ts2)
                        .ok_or(EafError::InvalidTimeSlotId(ts2.to_owned()))?;
                    annotation.set_ts_ref(new_ts1, new_ts2);

                    // println!("A TS1 {} -> {}", ts1, new_ts1);
                    // println!("A TS2 {} -> {}", ts2, new_ts2);
                }

                // If it exists, look up and set reference annotation ID. Required for referred annotations.
                if let Some(new_a_ref) = annotation.ref_id().and_then(|r| a_map.get(&r)) {
                    // println!("A REF {:?} -> {}", annotation.ref_id(), new_ref_id);
                    annotation.set_ref_id(new_a_ref);
                }

                // If it exists, look up and set previous annotation ID.
                if let Some(new_a_prev) = annotation.previous().and_then(|r| a_map.get(&r)) {
                    // println!("A REF {:?} -> {}", annotation.previous(), new_ref_id);
                    annotation.set_previous(new_a_prev);
                }
            }
        }

        // Re-index + derive EAF with updated values.
        self.index();
        self.derive()?;

        Ok(())
    }

    /// Creates a new `AnnotationDocument` struct, filtered
    /// according to `start`, `end` boundaries in milliseconds.
    /// All annotations within that time span will be intact.
    ///
    /// - `media_dir`: Path to the directory containing linked media files.
    /// - `ffmpeg_path`: Custom path to ffmpeg. If ffmpeg is already in `PATH`,
    /// set this to `None`.
    /// - `process_media`: If set to `true`, the original media files will be
    /// processed according to the specified time span and linked. Requires ffmpeg.
    /// The original media files will remain untouched on disk.
    pub fn filter(
        &self,
        start: i64,
        end: i64,
        media_dir: Option<&Path>,
        ffmpeg_path: Option<&Path>,
        process_media: bool,
    ) -> Result<Self, EafError> {
        let mut eaf = self.to_owned();

        // 1. Filter time order.
        //    Time slots without time values between min/max time slots
        //    with a time value will be preserved. However, the range
        //    can only start and end with a time slot with a time value set.
        let time_order = eaf.time_order.filter(start, end)
            .ok_or(EafError::InvalidTimeSpan((start, end)))?;
        eaf.time_order = time_order.to_owned();

        // 2. Make sure annotations have derived timestamps etc.
        if !eaf.derived {
            eaf.derive()?
        }

        // Owned `Index` to avoid borrow errors...
        let index = eaf.index.to_owned();

        // 3. Iterate over tiers and annotations...
        for tier in eaf.tiers.iter_mut() {
            let annots: Vec<Annotation> = tier
                .iter()
                .filter(|a| {
                    // ...then retrieve time slot ID. Need to check if each annotation
                    // is a ref annotation by trying to retrieve `main_annotation` reference...
                    let ts_ids = match &a.main() {
                        // Ref annotation
                        Some(id) => index.a2ts.get(id).to_owned(),
                        // Alignable annotation
                        None => index.a2ts.get(&a.id()).to_owned(),
                    };

                    // Do the actual filtering based on whether filtered `ts_id` Vec
                    // contains the time slot references/ID:s in question.
                    if let Some((ts_id1, ts_id2)) = ts_ids {
                        time_order.contains_id(&ts_id1) && time_order.contains_id(&ts_id2)
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            tier.annotations = annots;
        }

        // Generates new media files from time span and sets these as new media url:s.
        if process_media {
            for mdsc in eaf.header.media_descriptor.iter_mut() {
                mdsc.timespan(start, end, media_dir, ffmpeg_path)?;
            }
        }

        eaf.indexed = false;

        // 4. remap/update identifiers so that:
        //    - annotation ID:s start on "a1",
        //      including remapping ref annotation ID:s.
        //    - time slot ID:s start on "ts1",
        //      including remapping time slot references.
        //    - re-indexes eaf
        eaf.remap(None, None)?;

        Ok(eaf)
    }

    /// Attempts to add an annotation as last item in tier with specified tier ID,
    /// together with corresponding time slot in time order.
    /// If time values are not set (or are intentionally `None`) in the annotation,
    /// time slots with empty time slot values will be created, but note that
    /// time slots with no time value can never be the the final time slot.
    /// (Re-)index is optional. This is for cases where annotations are added in batch,
    /// in which case it may be better to index only once when done.
    pub fn add_annotation(
        &mut self,
        annotation: &Annotation,
        tier_id: &str,
        index: bool,
    ) -> Result<(), EafError> {
        // Derive if not done.
        if !self.derived {
            self.derive()?
        }

        // Check if annotation with same ID already exists.
        if matches!(&self.exists(&annotation.id()), (_, true)) {
            return Err(EafError::AnnotationIDExists(annotation.id()));
        }

        // Ensure referred annotation ID exists if ref annotation
        if let Some(ref_id) = annotation.ref_id() {
            if matches!(&self.exists(&ref_id), (_, false)) {
                return Err(EafError::InvalidAnnotationId(annotation.id()));
            }
        } else {
            // Add time slots if alignable annotation.
            let (ts_id1, ts_id2) = annotation.ts_ref()
                .ok_or(EafError::MissingTimeslotRef(annotation.id()))?;
            let (ts_val1, ts_val2) = annotation.ts_val();

            // Add time slots to time order. Only adds if it does not exist.
            self.add_timeslot(&ts_id1, ts_val1, false);
            self.add_timeslot(&ts_id2, ts_val2, false);
        }

        self.get_tier_mut(tier_id)
            .ok_or(EafError::InvalidTierId(tier_id.to_owned()))?
            .add(annotation);

        // Index or set `indexed` to false if not.
        if index {
            self.index()
        } else {
            self.indexed = false
        }

        Ok(())
    }

    /// Add an annotation along the time line to the specified tier,
    /// including between, before, or after existing annotations.
    pub fn add_annotation2(
        &mut self,
        annotation: &Annotation,
        tier_id: &str,
        index: bool,
    ) -> Result<(), EafError> {
        Ok(())
    }

    /// Returns reference to annotion with specified annotation ID if it exits.
    pub fn get_annotation(&self, id: &str) -> Option<&Annotation> {
        let (t_idx, a_idx) = self.index.a2idx.get(id)?;
        self.tiers.get(*t_idx)?.annotations.get(*a_idx)
    }

    /// Returns a mutable reference to annotion with specified annotation ID if it exits.
    pub fn get_annotation_mut(&mut self, id: &str) -> Option<&mut Annotation> {
        let (t_idx, a_idx) = self.index.a2idx.get(id)?;
        self.tiers.get_mut(*t_idx)?.annotations.get_mut(*a_idx)
    }

    /// Returns a reference to main annotation ID for specified ref annotation ID.
    pub fn main_annotation(&self, id: &str) -> Option<&Annotation> {
        match &self.index.a2ref.get(id) {
            Some(i) => self.main_annotation(i), // no mut version due to borrow issue here...
            None => self.get_annotation(id),
        }
    }

    /// Returns a mutable reference to main annotation ID for specified ref annotation ID.
    pub fn main_annotation_mut(&mut self, id: &str) -> Option<&mut Annotation> {
        let main_id = self.main_annotation(id)?.id(); // not mutable...
        self.get_annotation_mut(&main_id)
    }

    /// Returns a reference to main tier for specified ref tier ID.
    pub fn main_tier(&self, id: &str) -> Option<&Tier> {
        match &self.index.t2ref.get(id) {
            Some(i) => self.main_tier(i),
            None => self.get_tier(id),
        }
    }

    /// Returns mutable reference to main tier for specified ref tier ID.
    pub fn main_tier_mut(&mut self, id: &str) -> Option<&mut Tier> {
        let main_id = self.main_tier(id)?.tier_id.to_owned(); // not mutable...
        self.get_tier_mut(&main_id)
    }

    /// Returns a reference to the parent tier if
    /// specified tier ID is a referred tier.
    /// Returns `None` if tier ID is a main tier,
    /// or if either tier ID or the parent tier ID
    /// does not exist.
    pub fn parent_tier(&self, id: &str) -> Option<&Tier> {
        match &self.get_tier(id)?.parent_ref {
            Some(ref_id) => self.get_tier(ref_id),
            None => None,
        }
    }

    /// Returns references to all child tiers if present.
    pub fn child_tiers(&self, id: &str) -> Vec<&Tier> {
        self.tiers.iter()
            .filter_map(|t| if t.parent_ref.as_deref() == Some(id) {
                Some(t)
            } else {
                None
            })
            .collect()
    }

    /// Checks if tier with specified tier ID is tokenized.
    /// `recursive` checks if any parent is tokenized and returns `true`
    /// for the first tokenized parent found.
    /// Returns false if not tokenized or if tier ID does not exist.
    pub fn is_tokenized(&self, tier_id: &str, recursive: bool) -> bool {
        if recursive {
            let tier = match self.get_tier(tier_id) {
                Some(t) => t,
                None => return false,
            };

            // can only return true immediately if
            // tier with ID `tier_id` is tokenized
            let is_tkn = tier.is_tokenized();
            if is_tkn {
                true
            } else {
                // false, so need to check if parents are tokenized
                match &tier.parent_ref {
                    Some(id) => self.is_tokenized(id, recursive),
                    None => is_tkn,
                }
            }
        } else {
            self.get_tier(tier_id)
                .map(|t| t.is_tokenized())
                .unwrap_or(false)
        }
    }

    /// Adds a tier as the final item.
    /// If no tier is specified, an empty, default tier is appended - `stereotype`
    /// will be ignored in this case if set.
    pub fn add_tier(
        &mut self,
        tier: Option<Tier>,
        stereotype: Option<&StereoType>,
    ) -> Result<(), EafError> {
        match tier {
            Some(t) => {
                let ext_time_order = TimeOrder::from_hashmap(t.timeslots());
                self.time_order.join(&ext_time_order); // TODO should remap, dedup if necessary as well
                                                       // println!("{:?}", stereotype);
                let lt = match stereotype {
                    Some(s) => LinguisticType::new(&t.linguistic_type_ref, Some(s)),
                    None => LinguisticType::default(), // "default-lt" for a main, alignable tier
                };

                if !self.linguistic_types.contains(&lt) {
                    self.add_linguistic_type(&lt, true)
                }

                self.tiers.push(t);
            }
            None => self.tiers.push(Tier::default()),
        }

        self.index();
        self.derive()?;

        Ok(())
    }

    pub fn add_linguistic_type(&mut self, ling_type: &LinguisticType, add_constraint: bool) {
        if add_constraint {
            match &ling_type.constraints {
                Some(s) => {
                    let c = Constraint::from_string(s);
                    // let c = Constraint::from(s.to_owned()); // From trait doesn't work?
                    if !self.constraints.contains(&c) {
                        self.add_constraint(&c)
                    }
                }
                None => {}
            }
        }
        self.linguistic_types.push(ling_type.to_owned())
    }

    pub fn add_constraint(&mut self, constraint: &Constraint) {
        self.constraints.push(constraint.to_owned())
    }

    /// Returns a list of all tier IDs.
    pub fn tier_ids(&self) -> Vec<String> {
        if self.indexed {
            self.index.t2a.keys()
                .cloned()
                .collect()
        } else {
            self.tiers.iter()
                .map(|t| t.tier_id.to_owned())
                .collect()
        }
    }

    /// Returns specified tier.
    pub fn get_tier(&self, id: &str) -> Option<&Tier> {
        if self.indexed {
            let t_idx = self.index.t2idx.get(id)?;
            self.tiers.get(*t_idx)
        } else {
            self.tiers.iter().find(|t| t.tier_id == id)
        }
    }

    /// Get mutable tier.
    pub fn get_tier_mut(&mut self, id: &str) -> Option<&mut Tier> {
        if self.indexed {
            let t_idx = self.index.t2idx.get(id)?;
            self.tiers.get_mut(*t_idx)
        } else {
            self.tiers.iter_mut()
                .find(|t| t.tier_id == id)
        }
    }

    /// Change tier ID for existing tier.
    pub fn set_tier_id(&mut self, new_tier_id: &str, old_tier_id: &str) -> Result<(), EafError> {
        self.tiers.iter_mut()
            .find(|t| t.tier_id == old_tier_id)
            .map(|t| t.tier_id = new_tier_id.to_owned())
            .ok_or(EafError::InvalidTierId(old_tier_id.to_owned()))
    }

    /// Checks if specified ID exists as either tier ID and/or annotation ID.
    /// Returns `(bool, bool)` for `(tier exists, annotation exists)`.
    pub fn exists(&self, id: &str) -> (bool, bool) {
        (
            // use Index to check if `id` exists.
            self.index.t2a.keys().any(|t| t == id), // tier id
            self.index.a2t.keys().any(|a| a == id), // annotation id
        )
    }
}
