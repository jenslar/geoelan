#![crate_name = "eaf_rs"]
//! `eaf-rs` is a crate for de/serializing ELAN Annotation Format (.eaf) files.
//! ELAN is a tool for annotating audio-visual media, and is developed by Max Planck Institute for
//! Psycholinguistics, The Language Archive (no affiliation). See <https://archive.mpi.nl/tla/elan>.
//! Some descriptive text for struct memebers etc is borrowed from the EAF format manuals linked below.
//! 
//! EAF format specifications and schemas can be found here:
//! - EAF v2.7
//! 	+ xsd: <http://www.mpi.nl/tools/elan/EAFv2.7.xsd>
//! 	+ manual: <http://www.mpi.nl/tools/elan/EAF_Annotation_Format.pdf>
//! - EAF v2.8:
//! 	+ xsd: <http://www.mpi.nl/tools/elan/EAFv2.8.xsd>
//! 	+ manual: <http://www.mpi.nl/tools/elan/EAF_Annotation_Format_2.8_and_ELAN.pdf>
//! - EAF v3.0:
//! 	+ xsd: <http://www.mpi.nl/tools/elan/EAFv3.0.xsd>
//! 	+ manual: <http://www.mpi.nl/tools/elan/EAF_Annotation_Format_3.0_and_ELAN.pdf>
//! 
//! `eaf-rs` uses [quick-xml's](https://github.com/tafia/quick-xml) [serde](https://serde.rs) support for de/serialization.
//! 
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

pub mod eaf;
pub mod ffmpeg;

pub use eaf::{
    EafError,
    AnnotationDocument,
    Scope,
    License,
    Header,
    MediaDescriptor,
    Property,
    TimeOrder,
    TimeSlot,
    Tier,
    Annotation,
    LinguisticType,
    Constraint,
    StereoType,
    Language,
    LexiconRef,
    Index,
    Locale,
    ControlledVocabulary,
    JsonAnnotation,
    JsonEaf,
    JsonTier,
};