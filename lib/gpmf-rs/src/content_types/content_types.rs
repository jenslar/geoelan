/// Stream type, mostly for internal use.
/// This will have to be updated if new stream types are added or STNM free text descriptions change.
#[derive(Debug, Clone)]
pub enum ContentType {
    Accelerometer,           // Hero 7, 9
    AgcAudioLevel,           // Hero 9
    AverageLuminance,        // Hero 7
    CameraOrientation,       // Hero 9
    ExposureTime,            // Hero 7, 9
    FaceCoordinates,         // Hero 7, 9
    Gps,                     // Hero 7, 9
    GravityVector,           // Hero 9
    Gyroscope,               // Hero 7, 9
    ImageUniformity,         // Hero 7, 9
    ImageOrientation,        // Hero 9
    LrvFrameSkip,            // Hero 9
    MicrophoneWet,           // Hero 9
    MrvFrameSkip,            // Hero 9
    PredominantHue,          // Hero 7
    SceneClassification,     // Hero 7
    SensorGain,              // Fusion
    SensorIso,               // Hero 7, 9
    SensorReadOutTime,       // Hero 7
    WhiteBalanceRgbGains,    // Hero 7, 9
    WhiteBalanceTemperature, // Hero 7, 9
    WindProcessing,          // Hero 9
    Other(String),
}

impl ContentType {
    /// Returns stream name (`STNM`) specified in gpmf documentation as a string slice.
    pub fn to_str(&self) -> &str {
        match self {
            // Confirmed for Hero 7, 9. DOES NOT WORK FOR HERO5, STNM IS: Accelerometer (up/down, right/left, forward/back)
            ContentType::Accelerometer => "Accelerometer",
            // Confirmed for Hero 9 (' ,' typo exists in GPMF)
            ContentType::AgcAudioLevel => "AGC audio level[rms_level ,peak_level]",
            // Confirmed for Hero 7
            ContentType::AverageLuminance => "Average luminance",
            // Confirmed for Hero 9
            ContentType::CameraOrientation => "CameraOrientation",
            // Confirmed for Hero 7, 9, Fusion
            ContentType::ExposureTime => "Exposure time (shutter speed)",
            // Confirmed for Hero 7, 9
            ContentType::FaceCoordinates => "Face Coordinates and details",
            // Confirmed for Hero 7, 9, Fusion
            ContentType::Gps => "GPS (Lat., Long., Alt., 2D speed, 3D speed)",
            // Confirmed for Hero 9
            ContentType::GravityVector => "Gravity Vector",
            // Confirmed for Hero 7, 9
            ContentType::Gyroscope => "Gyroscope",
            // Confirmed for Hero 7, 9
            ContentType::ImageUniformity => "Image uniformity",
            // Confirmed for Hero 9
            ContentType::ImageOrientation => "ImageOrientation",
            // Confirmed for Hero 9
            ContentType::LrvFrameSkip => "LRV Frame Skip",
            // Confirmed for Hero 9
            ContentType::MicrophoneWet => "Microphone Wet[mic_wet, all_mics, confidence]",
            // Confirmed for Hero 9
            ContentType::MrvFrameSkip => "MRV Frame Skip",
            // Confirmed for Hero 7
            ContentType::PredominantHue => "Predominant hue[[hue, weight], ...]",
            // Confirmed for Hero 7
            ContentType::SceneClassification => "Scene classification[[CLASSIFIER_FOUR_CC,prob], ...]",
            // Confirmed for Fusion
            ContentType::SensorGain => "Sensor gain",
            // Confirmed for Hero 7, 9
            ContentType::SensorIso => "Sensor ISO",
            // Confirmed for Hero 7
            ContentType::SensorReadOutTime => "Sensor read out time",
            // Confirmed for Hero 7, 9
            ContentType::WhiteBalanceRgbGains => "White Balance RGB gains",
            // Confirmed for Hero 7, 9
            ContentType::WhiteBalanceTemperature => "White Balance temperature (Kelvin)",
            // Confirmed for Hero 9
            ContentType::WindProcessing => "Wind Processing[wind_enable, meter_value(0 - 100)]",
            ContentType::Other(s) => s,
        }
    }

    /// Returns enum corresponding to stream name (`STNM`) specified in gpmf stream.
    /// If no results are returned despite the data being present,
    /// try using `ContentType::Other(String)` instead. Gpmf data can only be identified
    /// via its stream name free text description (`STNM`), which may differ between devices
    /// for the same kind of data.
    pub fn from_str(stream_type: &str) -> ContentType {
        match stream_type {
            // Hero 7, 9 | Fusion
            "Accelerometer" | "Accelerometer (up/down, right/left, forward/back)" => {
                ContentType::Accelerometer
            }
            // Hero 9 (comma spacing is correct)
            "AGC audio level[rms_level ,peak_level]" => ContentType::AgcAudioLevel,
            // Hero 7
            "Average luminance" => ContentType::AverageLuminance,
            // Hero 9
            "CameraOrientation" => ContentType::CameraOrientation,
            // Hero 7, 9, Fusion
            "Exposure time (shutter speed)" => ContentType::ExposureTime,
            // Hero 7, 9
            "Face Coordinates and details" => ContentType::FaceCoordinates,
            // Hero 7, 9
            "GPS (Lat., Long., Alt., 2D speed, 3D speed)" => ContentType::Gps,
            // Hero 9
            "Gravity Vector" => ContentType::GravityVector,
            // Hero 7, 9 | Fusion
            "Gyroscope" | "Gyroscope (z,x,y)" => ContentType::Gyroscope,
            // Hero 7, 9
            "Image uniformity" => ContentType::ImageUniformity,
            // Hero 9
            "ImageOrientation" => ContentType::ImageOrientation,
            // Hero 9
            "LRV Frame Skip" => ContentType::LrvFrameSkip,
            // Hero 9
            "Microphone Wet[mic_wet, all_mics, confidence]" => ContentType::MicrophoneWet,
            // Hero 9
            "MRV Frame Skip" => ContentType::MrvFrameSkip,
            // Hero 7
            "Predominant hue[[hue, weight], ...]" => ContentType::PredominantHue,
            // Hero 7
            "Scene classification[[CLASSIFIER_FOUR_CC,prob], ...]" => {
                ContentType::SceneClassification
            }
            // Fusion
            "Sensor gain (ISO x100)" => ContentType::SensorGain,
            // Hero 7, 9
            "Sensor ISO" => ContentType::SensorIso,
            // Hero 7
            "Sensor read out time" => ContentType::SensorReadOutTime,
            // Hero 7, 9
            "White Balance RGB gains" => ContentType::WhiteBalanceRgbGains,
            // Hero 7, 9
            "White Balance temperature (Kelvin)" => ContentType::WhiteBalanceTemperature,
            // Hero 9
            "Wind Processing[wind_enable, meter_value(0 - 100)]" => ContentType::WindProcessing,
            // Other
            s => ContentType::Other(s.to_owned()),
        }
    }
}
