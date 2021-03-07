use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use crate::errors::EafError;
use crate::parse::{annotations, constraints, header, linguistic_types, tier_attribs, timeslots};

#[derive(Debug)]
pub struct EafFile {
    pub path: PathBuf,
    pub header: Header,
    pub timeslots: HashMap<String, u64>, // "ts2": 2681 -> id: timestamp in ms
    pub tiers: Vec<Tier>,
    pub linguistic_types: Vec<LinguisticType>,
    pub constraints: Vec<Constraint>,
}

impl EafFile {
    pub fn parse(path: &Path) -> Result<EafFile, EafError> {
        // ok to parse eaf in full on new(), instead of .parse()?
        let eaf_tier_attribs = tier_attribs(path)?;
        let mut eaftiers: Vec<Tier> = Vec::new();
        for tier_attribs in eaf_tier_attribs.iter() {
            let annots = annotations(path, Some(&tier_attribs.tier_id))?;
            let mut tokenized = false;
            if let Some(a) = annots.get(1) {
                // second annotation and on has PREVIOUS_ANNOTATION
                if a.attributes.previous_annotation.is_some() {
                    tokenized = true
                }
            }
            eaftiers.push(Tier {
                attributes: tier_attribs.to_owned(),
                tokenized,
                annotations: annots,
            });
        }
        Ok(EafFile {
            path: path.to_owned(),
            header: header(&path)?,
            timeslots: timeslots(&path)?,
            tiers: eaftiers,
            linguistic_types: linguistic_types(&path)?,
            constraints: constraints(&path)?,
        })
    }

    pub fn tier_ids(&self) -> Vec<String> {
        self.tiers
            .iter()
            .map(|i| i.attributes.tier_id.to_owned())
            .collect()
    }

    pub fn get_tier(&self, tier_id: &str) -> Option<Tier> {
        // should return Vec with max length == 1
        self.tiers
            .clone() // clone ok?
            .into_iter()
            .filter(|t| t.attributes.tier_id == tier_id)
            .collect::<Vec<Tier>>()
            .get(0)
            .cloned()
    }

    pub fn get_annotation(&self, tier: &Tier, annotation_id: &str) -> Option<Annotation> {
        // should return Vec with max length == 1
        tier.annotations
            .clone() // clone ok?
            .into_iter()
            .filter(|a| a.attributes.annotation_id == annotation_id)
            .collect::<Vec<Annotation>>()
            .get(0)
            .cloned()
    }

    pub fn derive_main_annotation(
        &self,
        tier: &Tier,
        annotation: &Annotation,
        err_on_tokenized: bool,
    ) -> Result<Annotation, EafError> {
        match &annotation.attributes.annotation_ref {
            Some(ref_id) => {
                // if annotation_ref exists, parent_ref tier must exist so unwrap should be safe here
                let parent_tier = self
                    .get_tier(&tier.attributes.parent_ref.clone().unwrap())
                    .unwrap();
                if parent_tier.tokenized {
                    return Err(EafError::TokenizedTier(parent_tier.attributes.tier_id));
                }
                let parent_annotation = self.get_annotation(&parent_tier, &ref_id).unwrap();
                self.derive_main_annotation(&parent_tier, &parent_annotation, err_on_tokenized)
            }
            None => Ok(annotation.to_owned()),
        }
    }

    /// Get main tier for specified tier.
    /// If main tier specified, it is returned as is.
    pub fn derive_main_tier(&self, tier: &Tier) -> Tier {
        match &tier.attributes.parent_ref {
            Some(t_id) => {
                let parent_tier = self.get_tier(t_id).unwrap(); // unwrap should be safe here
                self.derive_main_tier(&parent_tier)
            }
            None => tier.to_owned(),
        }
    }

    /// For dependent tiers only
    /// If main tier specified, it is returned as is.
    /// If dependent tier passed, it is return with time_slot_value set to
    /// Some(u64), those of main tier.
    /// Since timevalues will not be representative for tiers with a tokenized parent tier,
    /// `err_on_tokenized` allows to return error for this case.
    pub fn derive_timevalues(&self, tier: &Tier, err_on_tokenized: bool) -> Result<Tier, EafError> {
        let mut annotations: Vec<Annotation> = Vec::new();
        if tier.tokenized {
            return Err(EafError::TokenizedTier(tier.attributes.tier_id.to_owned()));
        }
        // return main tiers as is
        if tier.attributes.parent_ref.is_none() {
            return Ok(tier.to_owned());
        }
        for annotation in tier.annotations.iter() {
            let main_annot = self.derive_main_annotation(&tier, &annotation, err_on_tokenized)?;
            annotations.push(Annotation {
                attributes: AnnotationAttributes {
                    time_slot_value1: main_annot.attributes.time_slot_value1,
                    time_slot_value2: main_annot.attributes.time_slot_value2,
                    ..annotation.attributes.to_owned()
                },
                ..annotation.to_owned()
            })
        }

        Ok(Tier {
            annotations,
            ..tier.to_owned()
        })
    }
}

#[derive(Debug)]
/// EAF header
pub struct Header {
    pub time_units: String, // default "milliseconds"
    pub media_file: String, // attribute in <HEADER>, usually empty?
    pub media_descriptor: Vec<MediaDescriptor>,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
/// Part of EAF header, specifies linked media files
pub struct MediaDescriptor {
    pub extracted_from: Option<String>,
    pub mime_type: String,
    pub media_url: String,
    pub relative_media_url: String,
}

// Optional key, value element in EAF header:
// <PROPERTY NAME="SOME NAME">"VALUE"</PROPERTY>
#[derive(Debug)]
pub struct Property {
    pub name: String,  // attribute "NAME"
    pub value: String, // text content
}

#[derive(Debug, Clone)]
pub struct Tier {
    pub attributes: TierAttributes,
    pub tokenized: bool,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone)]
pub struct TierAttributes {
    pub tier_id: String,
    pub parent_ref: Option<String>,
    pub participant: Option<String>,
    pub annotator: Option<String>,
    pub linguistic_type_ref: String, // more?
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub attributes: AnnotationAttributes,
    pub annotation_value: String,
}

#[derive(Debug, Clone)]
pub struct AnnotationAttributes {
    // missing previous_annotation (tokenised tier)
    pub annotation_id: String,               // "a1"
    pub annotation_ref: Option<String>,      // reference annotation_id in parent tier
    pub previous_annotation: Option<String>, // for tokenised tiers
    pub time_slot_value1: Option<u64>, // none for ref_annotation, value in TIME_ORDER for e.g. "ts1"
    pub time_slot_value2: Option<u64>, // none
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Stereotype {
    SymbolicSubdivision, // Symbolic_Subdivision
    IncludedIn,          // Included_In
    SymbolicAssociation, // Symbolic_Association
    TimeSubdivision,     // Time_Subdivision
                         // Unknown(String) // only four specified in EAFv3.0 specification
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub stereotype: Stereotype,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct LinguisticType {
    pub constraints: Option<Stereotype>,
    pub graphic_references: bool,
    pub linguistic_type_id: String,
    pub time_alignable: bool,
}
