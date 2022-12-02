//! Time-to-sample atom (`stts`). Time-to-sample table only.

use std::ops::Range;

/// Time-to-sample atom (`stts`). Time-to-sample table only.
/// 
/// Layout: `Vec<(sample_count, sample_duration)>`.
/// 
/// Time-to-sample table:
/// - Sample count: A 32-bit integer that specifies the number of consecutive samples that have the same duration.
/// - Sample duration: A 32-bit integer that specifies the duration of each sample. (presumably in milliseconds)
/// 
/// https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFChap2/qtff2.html
#[derive(Debug, Default)]
// pub struct Stts(Vec<(u32, u32)>); // (sample_count, sample_duration)
pub struct Stts(Vec<u32>); // Instead of more compact (count, duration), 'duration' is duplicated 'count' number of times to allow easy "zip-itering" over stts + stsz in parallel.
impl Stts {
    pub fn new(values: Vec<u32>) -> Self {
        Self(values)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter()
    }
    pub fn slice(&self, range: Range<usize>) -> &[u32] {
        &self.0[range]
    }
}