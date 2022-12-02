//! Sample-to-size (in bytes) atom (`stsz`).

use std::ops::Range;

/// Sample-to-size (in bytes) atom. FourCC `stsz`.
#[derive(Debug, Default)]
pub struct Stsz(Vec<u32>);
impl Stsz {
    pub fn new(values: Vec<u32>) -> Self {
        Self(values)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter()
    }
    /// Panics if out of bounds.
    pub fn slice(&self, range: Range<usize>) -> &[u32] {
        &self.0[range]
    }
}