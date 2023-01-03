//! EAF tier.

use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde::{Deserialize, Serialize};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::{
    Annotation,
    EafError
};

/// EAF tier.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Tier {
    pub tier_id: String,
    pub participant: Option<String>,
    pub annotator: Option<String>,
    pub linguistic_type_ref: String, // more?
    // pub default_locale: Option<String>, // TODO refers to language_code in Locale
    pub parent_ref: Option<String>,
    // pub ext_ref: Option<> // TODO external reference
    // pub lang_ref: Option<> // TODO language reference
    #[serde(rename = "ANNOTATION", default)] // default required...?
    // #[serde(rename(serialize = "$value"))]
    pub annotations: Vec<Annotation>,
}

impl Default for Tier {
    fn default() -> Self {
        Self {
            tier_id: "default".to_owned(),
            participant: None,
            annotator: None,
            parent_ref: None,
            linguistic_type_ref: "default-lt".to_owned(), // more?
            annotations: Vec::new(),
        }
    }
}

impl Tier {
    /// Create new tier.
    pub fn new(
        tier_id: &str,
        annotations: Option<&[Annotation]>,
        linguistic_type_ref: Option<&str>,
        parent_ref: Option<&str>,
    ) -> Self {
        let mut tier = Self::default();
        tier.tier_id = tier_id.to_owned();
        if let Some(a) = annotations {
            tier.annotations = a.to_owned()
        }
        if let Some(l) = linguistic_type_ref {
            tier.linguistic_type_ref = l.to_owned()
        }
        if let Some(p) = parent_ref {
            tier.parent_ref = Some(p.to_owned())
        }
        tier
    }

    /// Create new referred tier from values, assumed to be in chronologial order.
    /// Number of values are not equal to the number of annotations in the parent tier,
    /// values will be laid out in order until the last one.
    /// If no start index is specified the first annotation ID will succeed the last one
    /// in the parent tier. If the parent is empty an empty ref tier is created.
    // pub fn new_ref(values: &[String], tier_id: String, parent: &Tier, linguistic_type_ref: String, start_index: Option<usize>) -> Result<Tier, EafError> {
    pub fn ref_from_values(
        values: &[String],
        tier_id: &str,
        parent: &Tier,
        linguistic_type_ref: &str,
        start_index: Option<usize>,
    ) -> Result<Tier, EafError> {
        let mut ref_tier = Tier::default();
        ref_tier.tier_id = tier_id.to_owned();
        ref_tier.parent_ref = Some(parent.tier_id.to_owned());
        ref_tier.linguistic_type_ref = linguistic_type_ref.to_owned();

        if !values.is_empty() {
            if values.len() > parent.len() {
                return Err(EafError::RefTierAlignmentError((
                    tier_id.to_owned(),
                    parent.tier_id.to_owned(),
                )));
            }

            let first_idx: usize = if let Some(id) = start_index {
                id
            } else {
                match parent.max_a_id() {
                    Some(id) => id.replace("a", "").parse()?,
                    None => 1,
                }
            };

            let mut id_count: usize = 0;
            for (i, val) in values.iter().enumerate() {
                if let Some(parent_a) = parent.annotations.get(i) {
                    let a = Annotation::referred(
                        val,
                        &format!("a{}", first_idx + id_count),
                        &parent_a.id(),
                        None,
                    );
                    ref_tier.add(&a);
                    id_count += 1;
                }
            }
        }

        Ok(ref_tier)
    }

    /// Returns true if tier is a referred tier.
    pub fn is_ref(&self) -> bool {
        self.parent_ref.is_some()
    }

    /// Returns true if the tier is tokenized.
    pub fn is_tokenized(&self) -> bool {
        self.iter().any(|a| a.previous().is_some())
    }

    /// Returns number of annotations in tier.
    pub fn len(&self) -> usize {
        self.annotations.len()
    }

    /// Returns a reference to the first annotation, if the tier is not empty.
    pub fn first(&self) -> Option<&Annotation> {
        self.annotations.first()
    }

    /// Returns a mutable reference to the first annotation, if the tier is not empty.
    pub fn first_mut(&mut self) -> Option<&mut Annotation> {
        self.annotations.first_mut()
    }

    /// Returns a reference to the last annotation, if the tier is not empty.
    pub fn last(&self) -> Option<&Annotation> {
        self.annotations.last()
    }

    /// Returns a mutable reference to the last annotation, if the tier is not empty.
    pub fn last_mut(&mut self) -> Option<&mut Annotation> {
        self.annotations.last_mut()
    }

    /// Returns a reference to the annotation with specified ID, if it exits.
    pub fn find(&self, annotation_id: &str) -> Option<&Annotation> {
        self.annotations.iter().find(|a| a.id() == annotation_id)
    }

    /// Queries annotation values.
    /// Returns tuples in the form
    /// (Annotation Index, Tier ID, Annotation ID, Annotation value, Ref Annotation ID).
    /// where index corresponds to annotation order in the EAF-file.
    pub fn query(&self, pattern: &str, ignore_case: bool) -> Vec<(usize, String, String, String, Option<String>)> {
        self.iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let org_val = a.value();
                let (val, ptn) = match ignore_case {
                    true => (org_val.to_lowercase(), pattern.to_lowercase()),
                    false => (org_val.to_owned(), pattern.to_owned()),
                };
                if val.contains(&ptn) {
                    Some((i + 1, self.tier_id.to_owned(), a.id(), org_val, a.ref_id()))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Match annotation values against regular expression.
    /// Returns tuples in the form
    /// (Annotation Index, Tier ID, Annotation ID, Annotation value, Ref Annotation ID).
    /// where index corresponds to annotation order in the EAF-file.
    /// TODO regular expressions
    pub fn query_rx(&self, regex: &Regex) -> Vec<(usize, String, String, String, Option<String>)> {
        self.iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let org_val = a.value();
                if regex.is_match(&org_val) {
                    Some((i + 1, self.tier_id.to_owned(), a.id(), org_val, a.ref_id()))
                } else {
                    None
                }
                // let (val, ptn) = match ignore_case {
                //     true => (org_val.to_lowercase(), pattern.to_lowercase()),
                //     false => (org_val.to_owned(), pattern.to_owned()),
                // };
                // if val.contains(&ptn) {
                //     Some((i + 1, self.tier_id.to_owned(), a.id(), org_val))
                // } else {
                //     None
                // }
                // None
            })
            .collect()
    }

    /// Naive implementation of ngram. Checks lower case variants only.
    /// Optionally remove regex matches, before checking. Only usable
    /// for scripts which uses whitespace as a delimiter
    /// (i.e. CJK is out of scope for this implementation).
    /// 
    /// - `tier = true` compiles ngrams across annotation boundaries
    /// - `tier = false` compiles per annotation and combines the result
    /// 
    /// Returns `HashMap<ngram, count>`.
    pub fn ngram(&self, size: usize, regex_remove: Option<&Regex>, tier: bool) -> HashMap<String, usize> {
        let mut ngrams: HashMap<String, usize> = HashMap::new();

        if tier {
            // ngrams per tier
            let tokens = self.annotations.iter()
                .flat_map(|a| a.value()
                    .split_ascii_whitespace()
                    .map(|v| {
                        // v.
                        if let Some(rx) = regex_remove {
                            rx.replace_all(&v.to_lowercase(), "").to_string()
                        } else {
                            v.to_lowercase()
                        }
                    }).collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();
            for value in tokens.windows(size) {
                *ngrams.entry(value.join(" ")).or_insert(0) += 1;
            }
        } else {
            // ngrams per annotation
            self.iter()
                .for_each(|a| ngrams.extend(a.ngram(size, regex_remove)));
        }
        
        ngrams
    }

    /// Returns all words/tokens in all annotation values in tier.
    /// Splits on whitespace, meaning CJK will not work with this method.
    /// Optionally return all unique words.
    pub fn tokens(
        &self,
        strip_prefix: Option<&str>,
        strip_suffix: Option<&str>,
        unique: bool,
        ignore_case: bool,
    ) -> Vec<String> {
        let prefix: Vec<char> = strip_prefix
            .map(|s| s.chars().collect())
            .unwrap_or(Vec::new());
        let suffix: Vec<char> = strip_suffix
            .map(|s| s.chars().collect())
            .unwrap_or(Vec::new());

        let mut tokens: Vec<String> = self.annotations
            // .iter()
            .par_iter() // possibly slower for tiers with few annotations
            .map(|a| {
                a.value()
                    .split_whitespace()
                    .map(|str_slice| {
                        let string = str_slice
                            .trim_start_matches(prefix.as_slice())
                            .trim_end_matches(suffix.as_slice());
                        // run prefix/start and suffix/end mathces
                        // independantly since they are not necessarily identical
                        // string = string
                        //     .trim_start_matches(prefix.as_slice())
                        //     .trim_end_matches(suffix.as_slice());
                        // string = string.trim_start_matches(prefix.as_slice());
                        // string = string.trim_end_matches(suffix.as_slice());

                        if ignore_case {
                            string.to_lowercase()
                        } else {
                            string.to_owned()
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        tokens.sort();

        if unique {
            tokens.dedup();
        }

        tokens
    }

    /// Adds an annotation as last item in tier.
    /// Does not evaluate whether e.g. referred annotation ID is
    /// valid for a referred annotation.
    ///
    /// Make sure to add corresponding time slot value to `AnnotationDocument`.
    ///
    /// To add an annotation at an abitrary position (e.g. in the middle of a tier),
    /// use `AnnotationDocument::add_annotation()` instead,
    /// since time slots may have to be added and re-mapped.
    pub fn add(&mut self, annotation: &Annotation) {
        self.annotations.push(annotation.to_owned())
    }

    /// Join two tiers. The first tier's (the one this method is used on)
    /// attributes will be preserved, the second one's will discarded.
    /// No checks for duplicate annotations will be made.
    pub fn join(&mut self, tier: &Tier) {
        self.annotations.extend(tier.annotations.to_owned());
    }

    pub fn overlaps(&self) {
        // cmp current boundaries with next
        // either:
        // - join
        // - preserve current (shift start or next forwards, back to back)
        // - preserve next (shift end of current backwards, back to back)
    }

    /// Merge two tiers. The first tier's (the one this method is used on)
    /// is prioritized in terms of what attributes will be preserved.
    /// If join is set to `true` overlapping annotations will be joined.
    /// Otherwise, the first annotation in chronological order
    /// will be prioritised. The second annotation's start time stamp.
    /// TODO currently does not remap ID values. Should be optional if implemented.
    pub fn merge(&mut self, tier: &Tier, join: bool) -> Result<(), EafError> {
        // Return error if tiers are not the same type
        if self.is_ref() != tier.is_ref() {
            return Err(EafError::IncompatibleTiers((
                self.tier_id.to_owned(),
                tier.tier_id.to_owned(),
            )));
        }

        // dedup, but only for annotations that are exactly the same, including id, timestamps etc
        let mut annotations: HashSet<Annotation> = HashSet::new();
        println!("SELF BEFORE: {}", self.len());
        annotations.extend(self.annotations.to_owned());
        println!("TIER BEFORE: {}", tier.len());
        annotations.extend(tier.annotations.to_owned());

        // create vec and sort remaining annotations
        let mut sorted: Vec<Annotation> = annotations.into_iter().collect();
        sorted.sort_by_key(|a| a.id()); // should maybe sort by timestamp...? check with Han if annot id is always ordered?
        println!("MERGED: {}", sorted.len());

        self.annotations = sorted;

        // Solve overlaps. Currently hinges on that sorting annotations via ID make them chronologically sorted
        // let mut adjusted: Vec<Annotation> = Vec::new();
        // // sorted.windows(2).inspect(|f| )
        // for annots in sorted.windows(2) {
        //     let a1 = &annots[0];
        //     let a2 = &annots[1];

        //     match (a1.ts_val(), a2.ts_val()) {
        //         ((Some(ts1), Some(te1)), (Some(ts2), Some(te2))) => {

        //         },
        //         _ => return Err(EafError::MissingTimeslotVal(format!("{} and/or {}", a1.id(), a2.id())))
        //     }
        // }

        // self.annotations = sorted;
        // if join {
        //     for annots in sorted.windows(2) {

        //     }
        // }

        // Join annotators
        match (&self.annotator, &tier.annotator) {
            (Some(a1), Some(a2)) => self.annotator = Some(format!("{a1};{a2}")),
            (None, Some(a2)) => self.annotator = Some(a2.to_owned()),
            _ => (),
        }

        // Join participants
        match (&self.participant, &tier.participant) {
            (Some(p1), Some(p2)) => self.participant = Some(format!("{p1};{p2}")),
            (None, Some(p2)) => self.participant = Some(p2.to_owned()),
            _ => (),
        }

        Ok(())
    }

    /// Extends existing annotations as the last items in tier.
    /// Does not evaluate whether e.g. referred annotation ID is
    /// valid for a referred annotation.
    ///
    /// Make sure to add the corresponding time slot value to `AnnotationDocument`.
    ///
    /// To add an annotation at an abitrary position (e.g. in the middle of a tier),
    /// use `AnnotationDocument::add_annotation()` instead,
    /// since time slots may have to be added and re-mapped.
    pub fn extend(&mut self, annotations: &[Annotation]) {
        self.annotations.extend(annotations.to_owned())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = Annotation> {
        self.annotations.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Annotation> {
        self.annotations.iter_mut()
    }

    /// Returns a list of time slots generated from the annotations in the tier.
    /// Make sure to run `AnnotationDocument::derive()` or time slot values may not be set.
    pub fn timeslots(&self) -> HashMap<String, Option<i64>> {
        // pub fn timeslots(&self) -> HashMap<String, Option<u64>> {
        let mut ts: HashMap<String, Option<i64>> = HashMap::new();
        // let mut ts: HashMap<String, Option<u64>> = HashMap::new();
        self.iter().for_each(|a| {
            if let (Some((ref1, ref2)), (val1, val2)) = (a.ts_ref(), a.ts_val()) {
                ts.insert(ref1, val1);
                ts.insert(ref2, val2);
            }
        });
        ts
    }

    /// Returns annotation values only.
    pub fn values(&self) -> Vec<String> {
        self.iter().map(|a| a.value()).collect()
    }

    pub fn max_a_id(&self) -> Option<String> {
        let mut id: Vec<String> = self.iter().map(|a| a.id()).collect();

        id.sort();

        id.last().cloned()
    }
}
