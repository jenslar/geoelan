/// Media unique ID, extracted from the
/// `udta` atom of an unedited GoPro MP4 file.
/// Eight values.
/// Can either be the derived from the "raw" `udta`
/// data, or the GPMF stream it also contains.
/// So far the first four values are equal between the
/// "raw" `udta`values and the GPMF stream.
/// The first four values are also equal for clips
/// belonging to the same session so far.
/// The last four values differ between MP4 files,
/// even for those in the same session.
#[derive(Debug, PartialEq)]
pub enum Muid {
    Udta(Vec<u32>),
    Gpmf(Vec<u32>),
    Bytes(Vec<u8>)
}

impl Muid {
    pub fn len(&self) -> usize {
        match &self {
            Self::Udta(muid) | Self::Gpmf(muid) => muid.len(),
            Self::Bytes(muid) => muid.len(),
        }
    }

    /// Returns first four values of MUID. Equal for all
    /// clips in the same session so far.
    /// Panics if input contains fewer than four values.
    pub fn head(&self) -> &[u32] {
        match &self {
            Self::Udta(muid) | Self::Gpmf(muid) => &muid[0..4],
            _ => &[]
        }
    }

    /// Returns last four values of MUID. Differs between
    /// files so far, including those in the same session.
    /// Panics if input contains fewer than eight values.
    pub fn tail(&self) -> &[u32] {
        match &self {
            Self::Udta(muid) | Self::Gpmf(muid) => &muid[4..8],
            _ => &[]
        }
    }
}

impl std::fmt::Display for Muid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Udta(muid) | Self::Gpmf(muid) => write!(f, "{:?}", muid),
            Self::Bytes(muid) => write!(f, "{:?}", muid)
        }
    }
}