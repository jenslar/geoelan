//! Internal index for faster retreival of various cross-referenced values, such as immediately finding
//! the "main" annotation for a referred annotation, or the millisecond time values for its boundaries.
//! Mostly for internal use.
//! 
//! Contains the following mappings:
//! - `a2t`: Annotation ID to tier ID
//! - `a2ref`: Annotation ID to ref annotation ID
//! - `t2a`: Tier ID to list of annotation ID:s
//! - `t2ref`: Tier ID to ref tier ID
//! - `id2ts`: Time slot ID to time slot value
//! - `ts2id`: Time slot value to Time slot ID
//! - `a2ts`: Annotation ID to time slot id/ref tuple, `(time_slot_ref1, time_slot_ref2)`.
//! - `a2idx`: Annotation ID to `(AnnotationDocument.tiers[idx1], tier.annotations[idx2])`
//! - `t2idx`: Tier ID to `AnnotationDocument.tiers[idx]`

use std::collections::HashMap;

#[derive(Debug, Clone)]
/// Index with mappings for:
/// - Annotation ID to tier ID
/// - Annotation ID to ref annotation ID
/// - Tier ID to list of annotation ID:s
/// - Tier ID to ref tier ID
/// - Time slot ID to time slot value
/// - Time slot value to Time slot ID
/// - Annotation ID to time slot id/ref tuple, `(time_slot_ref1, time_slot_ref2)`.
/// - Annotation ID to `(AnnotationDocument.tiers[idx1], tier.annotations[idx2])`
/// - Tier ID to `AnnotationDocument.tiers[idx]`
pub struct Index {
    /// Key: Annotation ID. Value: Tier ID.
    pub a2t: HashMap<String, String>,
    /// Key: Annotation ID. Value: Ref annotation ID.
    pub a2ref: HashMap<String, String>,
    /// Key: Tier ID. Value: List of Annotation ID:s in tier.
    pub t2a: HashMap<String, Vec<String>>,
    /// Key: Tier ID. Value: Ref Tier ID.
    pub t2ref: HashMap<String, String>,
    /// Key: Time slot ID. Value: Time slot value (optional).
    pub ts2tv: HashMap<String, Option<i64>>,
    /// Key: Time slot value. Value: Time slot ID.
    /// Only contains time slots with a time slot value.
    pub tv2ts: HashMap<i64, String>,
    /// Key: Annotation ID. Value: Time slot id/ref tuple, `(time_slot_ref1, time_slot_ref2)`.
    pub a2ts: HashMap<String, (String, String)>,
    /// Key: Annotation ID. Value: Index in `AnnotationDocument.tiers` (i.e. which tier),
    /// and index in `tier.annotations` (i.e. which annotation), `(idx1, idx2)`.
    /// I.e. `AnnotationDocument.tiers[idx1].annotations[idx2]`.
    pub a2idx: HashMap<String, (usize, usize)>,
    /// Key: Tier ID. Value: Tier index in `AnnotationDocument.tiers`.
    /// I.e. `AnnotationDocument.tiers[idx]`.
    pub t2idx: HashMap<String, usize>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            a2t: HashMap::new(),
            a2ref: HashMap::new(),
            t2a: HashMap::new(),
            t2ref: HashMap::new(),
            ts2tv: HashMap::new(),
            tv2ts: HashMap::new(),
            a2ts: HashMap::new(),
            a2idx: HashMap::new(),
            t2idx: HashMap::new(),
        }
    }
}
