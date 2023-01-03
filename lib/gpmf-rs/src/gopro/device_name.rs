//! GoPro device name (`DVNM`).

/// GoPro camera model. Set in GPMF struct for convenience.
/// Does not yet include all previous models, hence `Other<String>`
// #[derive(Debug, Clone, Eq, Hash)]
// #[derive(Debug, Clone, PartialEq, Ord)]
#[derive(Debug, Clone)]
pub enum DeviceName {
    Hero5Black,  // DVNM not confirmed
    Hero6Black,  // DVNM not confirmed
    Hero7Black,  // DVNM "Hero7 Black" or "HERO7 Black" (MP4 GoPro MET udta>minf atom)
    Hero8Black,  // probably "Hero7 Black", but not confirmed
    Hero9Black,  // DVNM "Hero9 Black" or "HERO9 Black" (MP4 GoPro MET udta>minf atom)
    Hero10Black, // DVNM "Hero10 Black" or "HERO10 Black" (MP4 GoPro MET udta>minf atom)
    Hero11Black, // DVNM "Hero11 Black" or "HERO11 Black" (MP4 GoPro MET udta>minf atom)
    Fusion,
    GoProMax,
    GoProKarma,  // DVNM "GoPro Karma v1.0" + whichever device is connected e.g. hero 5.
    // other identifiers? Silver ranges etc?
    Other(String), // for models not yet included as enum
}

impl DeviceName {
    pub fn from_str(model: &str) -> Self {
        match model.trim() {
            "Hero5 Black" | "HERO5 Black" => DeviceName::Hero5Black, // correct device name?
            "Hero6 Black" | "HERO6 Black" => DeviceName::Hero6Black, // correct device name?
            "Hero7 Black" | "HERO7 Black" => DeviceName::Hero7Black,
            "Hero8 Black" | "HERO8 Black" => DeviceName::Hero8Black,
            "Hero9 Black" | "HERO9 Black" => DeviceName::Hero9Black,
            "Hero10 Black" | "HERO10 Black" => DeviceName::Hero10Black,
            "Hero11 Black" | "HERO11 Black" => DeviceName::Hero11Black,
            "Fusion" | "FUSION" => DeviceName::Fusion,
            "GoPro Max" => DeviceName::GoProMax,
            "GoPro Karma v1.0" => DeviceName::GoProKarma,
            s => DeviceName::Other(s.to_owned()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceName::Hero5Black => "Hero5 Black".to_owned(), // correct device name?
            DeviceName::Hero6Black => "Hero6 Black".to_owned(), // correct device name?
            DeviceName::Hero7Black => "Hero7 Black".to_owned(),
            DeviceName::Hero8Black => "Hero8 Black".to_owned(),
            DeviceName::Hero9Black => "Hero9 Black".to_owned(),
            DeviceName::Hero10Black => "Hero10 Black".to_owned(),
            DeviceName::Hero11Black => "Hero11 Black".to_owned(),
            DeviceName::Fusion => "Fusion".to_owned(),
            DeviceName::GoProMax => "GoPro Max".to_owned(),
            DeviceName::GoProKarma => "GoPro Karma v1.0".to_owned(), // only v1.0 so far
            DeviceName::Other(s) => s.to_owned(),
        }
    }

    // Get documented sample frequency for a specific device
    // pub fn freq(&self, fourcc: FourCC) {
    //     match self {

    //     }
    // }
}
