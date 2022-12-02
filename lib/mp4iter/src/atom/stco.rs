//! Sample-to-offset atom (`stco`).

use std::ops::Range;

/// Sample-to-offset (in bytes from start of MP4) atom.
/// Each value represents a byte offset for a data chunk
/// in the corresponding track.
#[derive(Debug, Default)]
pub struct Stco(Vec<u32>);
impl Stco {
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