//! MP4 offset. Values are derived from `stco`, `stts`, and `stsz` atoms.

#[derive(Debug)]
pub struct Offset {
    /// Offset in bytes from start of file.
    pub position: u32,
    /// Size of GPMF-chunk in bytes.
    pub size: u32,
    /// Duration in milliseconds,
    /// equal to the GPMF-chunk's "duration"
    /// within the `mdat` atom.
    pub duration: u32
}
impl Offset {
    pub fn new(position: u32, size: u32, duration: u32) -> Self {
        Self{position, size, duration}
    }
}

// pub struct Offsets(Vec<Offset>);

// impl Offsets {
//     pub fn time(&self) -> Vec<u32> {
//         self.0.iter().fold(0, |acc|)
//     }
// }