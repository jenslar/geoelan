use std::collections::HashMap;

use eaf_rs::Annotation;
use fit_rs::{FitPoint, GpsMetadata};
use gpmf_rs::GoProPoint;
use time::{ext::NumericalDuration, format_description, Duration, PrimitiveDateTime};

#[derive(Debug, Default, Clone)]
pub struct EafPoint {
    /// Latitude.
    pub latitude: f64,
    /// Longitude.
    pub longitude: f64,
    /// Altitude.
    pub altitude: f64,
    pub heading: Option<f64>,
    pub speed2d: f64,
    /// 3D speed.
    /// FIT logs 3D speed as a vector value [x, y, z],
    /// whereas GoPro logs this as a scalar. Thus,
    /// the FIT 3D speed value here is the dot product.
    pub speed3d: f64,
    /// Full datetime.
    /// FIT: Supported via post-process, via `timestamp_correlation` message.
    /// GPMF: Supported.
    // pub datetime: Option<NaiveDateTime>,
    // pub datetime: Option<NaiveDateTime>,
    pub datetime: Option<PrimitiveDateTime>,
    /// Timestamp relative to start of video.
    /// FIT: Supported.
    /// GPMF: Supported for MP4 GPMF streams only.
    pub timestamp: Option<Duration>,
    /// Duration. C.f. sample rate/sample duration.
    /// Time between logging points.
    /// Whenever downsampling points
    /// `duration` needs to be recaclulated,
    /// especially for VIRB/goPro to EAF.
    /// - FIT: Derived from time between points.
    /// - GPMF: Derived from MP4 `stts` atom (not available for GPMF streams extracted to file via eg FFmpeg).
    pub duration: Option<Duration>,
    /// Description.
    pub description: Option<String>,
}

impl std::fmt::Display for EafPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "  latitude:    {:.6}
  longitude:   {:.6}
  altitude:    {:.6}
  heading:     {}
  speed (2D):  {:.3}
  speed (3D):  {:.3}
  datetime:    {}
  timestamp:   {}s
  duration:    {}s
  description: {}",
            self.latitude,
            self.longitude,
            self.altitude,
            self.heading
                .map(|h| format!("{:.1}", h))
                .as_deref()
                .unwrap_or("NONE"),
            self.speed2d,
            self.speed3d,
            // self.datetime.map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3f").to_string()).as_deref().unwrap_or("NONE"),
            self.datetime
                .map(|dt| dt.to_string())
                .as_deref()
                .unwrap_or("NONE"),
            self.timestamp.map(|t| t.as_seconds_f64()).unwrap_or(0.0),
            // format!("{}s {}ms", t.num_seconds(), t.num_milliseconds() - t.num_seconds() * 1000))
            // .as_deref()
            // .unwrap_or("NONE"),
            self.duration.map(|t| t.as_seconds_f64()).unwrap_or(0.0),
            // self.duration.map(|t|
            //     format!("{}s {}ms", t.num_seconds(), t.num_milliseconds() - t.num_seconds() * 1000))
            //     .as_deref()
            //     .unwrap_or("NONE"),
            self.description.as_deref().unwrap_or("NONE"),
        )
    }
}

/// Convert single VIRB point/`gps_metadata` to `Point`.
/// See the FIT SDK for an explanatiton of the conversion.
impl From<&GpsMetadata> for EafPoint {
    fn from(gps: &GpsMetadata) -> Self {
        let semi2deg = 180.0 / 2.0_f64.powi(31);
        let relative_time = Duration::seconds(gps.timestamp as i64)
            + Duration::milliseconds(gps.timestamp_ms as i64);
        Self {
            latitude: (gps.latitude as f64) * semi2deg,
            longitude: (gps.longitude as f64) * semi2deg,
            altitude: (gps.altitude as f64 / 5.0) - 500.0,
            heading: Some(gps.heading as f64 / 100.0),
            speed2d: gps.speed as f64 / 1000.0,
            speed3d: (gps.velocity[0].pow(2) as f64
                + gps.velocity[1].pow(2) as f64
                + gps.velocity[2].pow(2) as f64)
                .sqrt()
                / 100.0,
            datetime: None, // derived from `timestamp_correlation` message
            timestamp: Some(relative_time),
            // duration: None,
            duration: Some(relative_time), // ????
            description: None,
        }
    }
}

impl From<&FitPoint> for EafPoint {
    /// Convert Single Garmin VIRB point to `Point`.
    /// Note that datetime is always set to `None` for this
    /// conversion. Use `Point::from_fit()` to convert and
    /// set datetime simultaneously.
    fn from(point: &FitPoint) -> Self {
        Self {
            latitude: point.latitude,
            longitude: point.longitude,
            altitude: point.altitude,
            heading: Some(point.heading),
            speed2d: point.speed2d,
            speed3d: point.speed3d,
            datetime: None, // derived from `timestamp_correlation` message
            timestamp: Some(point.time),
            duration: None,
            description: None,
        }
    }
}

impl From<&GoProPoint> for EafPoint {
    /// Convert Single GoPro point to `Point`.
    fn from(point: &GoProPoint) -> Self {
        Self {
            latitude: point.latitude,
            longitude: point.longitude,
            altitude: point.altitude,
            heading: None,
            speed2d: point.speed2d,
            speed3d: point.speed3d,
            datetime: Some(point.datetime),
            // timestamp: point.time.to_owned().map(|ts| ts.to_relative()), // derived from MP4 atom
            timestamp: Some(point.time.to_owned()), // derived from MP4 atom
            duration: None,
            // timestamp: point.time.as_ref().map(|ts| ts.relative), // derived from MP4 atom
            // duration: point.time.as_ref().map(|ts| ts.duration), // derived from MP4 atom
            description: None,
        }
    }
}

// impl TryFrom<&Annotation> for Point {
impl From<&Annotation> for EafPoint {
    /// Convert EAF annotation value to a `Point`.
    /// May fail if annotation value is not in the form
    /// `LAT:55.791765;LON:13.501448;ALT:101.6;TIME:2023-01-25 12:15:45.399`.
    ///
    /// If timevalues are not set for annotation boundaries,,
    /// `Point::timstamp` and `Point::duration` will be set to `None`.
    // fn try_from(annotation: &Annotation) -> Result<Self, Error> {
    fn from(annotation: &Annotation) -> Self {
        let value = annotation.value().to_string();
        // TODO add Annotation.duration() -> milliseconds method
        let (timestamp, duration) = match annotation.ts_val() {
            (Some(t1), Some(t2)) => (Some(t1.milliseconds()), Some((t2 - t1).milliseconds())),
            _ => (None, None),
        };
        // split LAT:55.791765;LON:13.501448;...
        let split = value
            .split(";")
            // split e.g. LAT:55.791765
            .filter_map(|spl| spl.split_once(":"))
            // keep 55.791765
            .map(|spl| spl.1) // TODO do unwrap_or here to get a "" on None?
            // [55.791765, 13.501448, 101.6, 2023-01-25 12:15:45.399]
            .collect::<Vec<_>>();

        // TODO parse time string
        // TODO better to iterate + enumerate and use idx 0 for lat etc
        let (lat, lon, alt, time) = match split.len() {
            4 => (
                split[0].parse::<f64>().unwrap_or_default(),
                split[1].parse::<f64>().unwrap_or_default(),
                split[2].parse::<f64>().unwrap_or_default(),
                split[3].to_owned(),
            ),
            _ => (
                f64::default(),
                f64::default(),
                f64::default(),
                String::default(),
            ),
        };

        Self {
            latitude: lat,
            longitude: lon,
            altitude: alt,
            heading: None,
            datetime: None, // TODO parse str into datetime
            timestamp,
            duration,
            ..Self::default()
        }
    }
}

impl EafPoint {
    /// Returns timestamp as milliseconds.
    pub fn timestamp_ms(&self) -> Option<i64> {
        self.timestamp.map(|t| (t.as_seconds_f64() * 1000.0) as i64)
    }

    /// Returns datetime as formatted string:
    /// `YYYY-MM-DDTHH:mm:ss.fff`
    pub fn datetime_string(&self) -> Option<String> {
        // let format = format_description::parse(
        //     "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour \
        //         sign:mandatory]:[offset_minute]", // no tz info
        // )
        let format = format_description::parse(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]",
        )
        .expect("Failed to create date time format"); // result instead?
        self.datetime.and_then(|dt| dt.format(&format).ok()) // result instead?
    }

    pub fn with_offset_hrs(&self, offset_hrs: i64) -> Self {
        Self {
            datetime: self.datetime.map(|dt| dt + Duration::hours(offset_hrs)),
            ..self.to_owned()
        }
    }

    /// Converts `geoelan::geo::Point` to the corresponding `kml::types::Point`.
    pub fn to_kml_point(&self) -> crate::kml::types::Point {
        crate::kml::types::Point {
            coord: crate::kml::types::Coord::new(
                self.longitude,
                self.latitude,
                Some(self.altitude),
            ),
            extrude: false,
            altitude_mode: ::kml::types::AltitudeMode::ClampToGround,
            attrs: HashMap::new(),
        }
    }

    /// Convert FIT `gps_metadata` (global FIT ID 160) to `Point`. Optionally set datetime, which
    /// is derived from FIT `timestamp_correlation` (global FIT ID 162).
    // pub fn from_fit(point: &GpsMetadata, datetime: Option<NaiveDateTime>) -> Self {
    pub fn from_fit(point: &GpsMetadata, datetime: Option<PrimitiveDateTime>) -> Self {
        // Constant for converting from semicircles to decimal degrees
        let semi2deg = 180.0 / 2.0_f64.powi(31);
        // GpsMetadata relative timestamp as duration
        let t = Duration::seconds(point.timestamp as i64)
            + Duration::milliseconds(point.timestamp_ms as i64);

        Self {
            latitude: (point.latitude as f64) * semi2deg,
            longitude: (point.longitude as f64) * semi2deg,
            altitude: (point.altitude as f64 / 5.0) - 500.0,
            heading: Some(point.heading as f64 / 100.0),
            speed2d: point.speed as f64 / 1000.0,
            speed3d: (point.velocity[0].pow(2) as f64
                + point.velocity[1].pow(2) as f64
                + point.velocity[2].pow(2) as f64)
                .sqrt()
                / 100.0,
            datetime: datetime.map(|dt| dt + t), // derived from `timestamp_correlation` message
            timestamp: Some(t),
            duration: None,
            description: None,
        }
    }

    /// Generate 2D circle polygon (KML: LinearRing).
    /// Vertices are limited to 3-255. Any value
    /// below 3 will be automatically set to 3, anything above 255 will
    /// fail due to `vertices` being `u8`.
    /// Returned as a "closed" `Vec<Point>`, i.e. last point is the
    /// same as the first one.
    pub fn circle(&self, radius: f64, vertices: u8) -> Vec<Self> {
        let mut circle: Vec<Self> = Vec::new();
        let pi = std::f64::consts::PI;

        let vertices = match vertices {
            i if i < 3 => 3,
            i => i,
        };

        for i in 0..vertices {
            let angle = 2.0 * pi * i as f64 / vertices as f64;

            let dx = radius * angle.cos();
            let dy = radius * angle.sin();

            let lat = self.latitude + (180_f64 / pi) * (dy / 6378137_f64);
            let lon = self.longitude
                + (180_f64 / pi) * (dx / 6378137_f64) / (self.latitude * pi / 180_f64).cos();

            circle.push(Self {
                latitude: lat,
                longitude: lon,
                ..self.to_owned()
            })
        }

        // Close the circle/polygon for KML/GeoJSON: final point must be the same as the first one
        circle.push(circle[0].to_owned());

        circle
    }
}
