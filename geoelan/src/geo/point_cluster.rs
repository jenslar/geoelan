//! `PointCluster`. Convenience struct for processing, including downsampling (latitude dependent).

use std::path::Path;

use eaf_rs::Eaf;
use fit_rs::GpsMetadata;
use geojson::GeoJson;
use gpmf_rs::GoProPoint;
use kml::KmlDocument;
use time::{Duration, PrimitiveDateTime};

use crate::files::writefile;

use super::{
    json_gen::{geojson_from_features, geojson_point},
    kml_gen::{kml_from_placemarks, kml_point, kml_to_string},
    EafPoint,
};

/// Point cluster with optional description.
#[derive(Debug, Default, Clone)]
pub struct EafPointCluster {
    pub points: Vec<EafPoint>,
    pub description: Option<String>,
    // pub geoshape: GeoShape
}

impl From<Vec<GoProPoint>> for EafPointCluster {
    fn from(points: Vec<GoProPoint>) -> Self {
        Self {
            points: points.iter().map(|p| EafPoint::from(p)).collect(),
            description: None,
        }
    }
}

impl EafPointCluster {
    pub fn new(points: &[EafPoint], description: Option<&str>) -> Self {
        Self {
            points: points.to_owned(),
            description: description.map(String::from),
        }
    }

    /// Convert a Garmin VIRB point slice to a point cluster.
    pub fn from_virb(
        points: &[GpsMetadata],
        description: Option<&str>,
        t0: &PrimitiveDateTime,
        end: &Duration,
        offset_hrs: Option<i64>,
    ) -> Self {
        let mut cluster = Self::default();

        cluster.description = description.map(String::from);
        cluster.points = points
            .iter()
            .map(|point| EafPoint::from(point).with_offset_hrs(offset_hrs.unwrap_or(0)))
            .collect();

        cluster.set_timedelta(Some(t0), end);

        cluster
    }

    /// Convert a GoPro point slice to a point cluster.
    pub fn from_gopro(
        points: &[GoProPoint],
        description: Option<&str>,
        end: &Duration,
        offset_hrs: Option<i64>,
    ) -> Self {
        let mut cluster = Self::default();

        cluster.description = description.map(String::from);
        cluster.points = points
            .iter()
            .map(|point| EafPoint::from(point).with_offset_hrs(offset_hrs.unwrap_or(0)))
            .collect();

        // 230424 added setting delta for gopro here instead of in gpmf crate, removed duration for gpmf-points
        cluster.set_timedelta(None, end);

        cluster
    }

    /// Use coordinates from an ELAN tier.
    /// Must correspong to the same pattern GeoELAN
    /// uses with the `--geotier` flag:
    /// `"LAT:{:.6};LON:{:.6};ALT:{:.1};TIME:{}"`,
    pub fn _from_eaf(eaf: &Eaf, tier_id: &str) -> Self {
        // LAT:55.481439;LON:13.026942;ALT:9.4;TIME:2021-05-03 13:04:34.571
        // let regex = Regex::new(r"LAT:([0-9]*[.][0-9]+);LON:([0-9]*[.][0-9]+);ALT:([0-9]*[.][0-9]+);TIME:(\d{4}-\d{2}-\d{2}.\d{2}:\d{2}:\d{2}\.\d+)");
        if let Some(tier) = eaf.get_tier(tier_id) {
            let points = tier
                .annotations
                .iter()
                .map(EafPoint::from)
                .collect::<Vec<_>>();
            let description = points.first().and_then(|p| p.description.to_owned());
            EafPointCluster::new(&points, description.as_deref())
        } else {
            EafPointCluster::default()
        }
        // "LAT:<f64>;LON:<f64>;ALT:<f64>;TIME:<String>"
    }

    /// Generate KML object that can be serialized into a string.
    pub fn to_kml(&self, indexed: bool) -> KmlDocument {
        let kml_points: Vec<_> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let name = match indexed {
                    true => Some((i + 1).to_string()),
                    false => None,
                };
                kml_point(p, name.as_deref(), None, false, None)
            })
            .collect();

        kml_from_placemarks(&kml_points, &[])
    }

    pub fn to_kml_string(&self, indexed: bool) -> String {
        kml_to_string(&self.to_kml(indexed))
    }

    /// Write KML to specified path.
    pub fn write_kml(&self, indexed: bool, path: &Path) -> std::io::Result<bool> {
        let string = self.to_kml_string(indexed);
        writefile(&string.as_bytes(), &path)
    }

    /// Generate GeoJson object that can be serialized into a string.
    pub fn to_json(&self, indexed: bool) -> GeoJson {
        let json_points: Vec<_> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let name = match indexed {
                    true => Some(i + 1),
                    false => None,
                };
                geojson_point(p, name)
            })
            .collect();
        geojson_from_features(&json_points)
    }

    pub fn to_json_string(&self, indexed: bool) -> String {
        self.to_json(indexed).to_string()
    }

    /// Write GeoJson to specified path.
    pub fn write_json(&self, indexed: bool, path: &Path) -> std::io::Result<bool> {
        let string = self.to_json_string(indexed);
        writefile(&string.as_bytes(), &path)
    }

    /// Set time offset in hours.
    pub fn offset_hrs(&mut self, offset: i64) -> Self {
        Self {
            points: self
                .points
                .iter()
                .map(|point| EafPoint {
                    datetime: point
                        .datetime
                        .map(|dt| dt + Duration::hours(offset))
                        .to_owned(),
                    ..point.to_owned()
                })
                .collect(),
            ..self.to_owned()
        }
    }

    /// Set time offset in hours.
    pub fn offset_hrs_mut(&mut self, offset: i64) {
        self.points.iter_mut().for_each(|point| {
            // Add time offset
            if let Some(dt) = point.datetime {
                point.datetime = Some(dt + Duration::hours(offset))
            }
        });
    }

    /// Get first point.
    pub fn first(&self) -> Option<&EafPoint> {
        self.points.first()
    }

    /// Get last point.
    pub fn last(&self) -> Option<&EafPoint> {
        self.points.last()
    }

    /// Iterate over points.
    pub fn iter(&self) -> impl Iterator<Item = &EafPoint> {
        self.points.iter()
    }

    /// Mutably iterate over points.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut EafPoint> {
        self.points.iter_mut()
    }

    /// Number of points in cluster.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Downsample points. A `sample_factor` of 10 means 1000 points
    /// will be averaged into 100 points and so on.
    pub fn downsample(&self, sample_factor: usize, min: Option<usize>) -> Self {
        Self {
            points: super::downsample(sample_factor, &self.points, min),
            ..self.to_owned()
        }
    }

    /// Downsample points. A `sample_factor` of 10 means 1000 points
    /// will be averaged into 100 points and so on.
    pub fn downsample_mut(&mut self, sample_factor: usize, min: Option<usize>) {
        self.points = super::downsample(sample_factor, &self.points, min)
    }

    /// Returns date time for first point.
    pub fn start_datetime(&self) -> Option<&PrimitiveDateTime> {
        self.points.first().and_then(|p| p.datetime.as_ref())
    }

    /// Returns date time for last point.
    pub fn end_datetime(&self) -> Option<&PrimitiveDateTime> {
        self.points.last().and_then(|p| p.datetime.as_ref())
    }

    /// Sets time between points ("sample duration", used as
    /// annotation length).
    /// For VIRB datetime is also set.
    pub fn set_timedelta(&mut self, t0: Option<&PrimitiveDateTime>, end: &Duration) {
        // max index for windows(2)
        let max = self.len() - 2;

        let mut delta = Vec::new();

        // 1. Populate timestamp/duration/delta
        //    Can't iter_mut with .windows()?
        for (i, points) in self.points.windows(2).enumerate() {
            if let (Some(t1), Some(t2)) = (
                points.get(0).and_then(|p| p.timestamp),
                points.get(1).and_then(|p| p.timestamp),
            ) {
                delta.push(t2 - t1);
                if i == max {
                    delta.push(end.to_owned() - t2);
                }
            }
        }

        assert_eq!(self.points.len(), delta.len());

        // 2. Set deltas/sample duration,
        //    used as annotation length for a 'geotier'.
        for (point, duration) in self.iter_mut().zip(delta) {
            point.duration = Some(duration);
            // Set datetime for VIRB
            if let (Some(t), Some(ts)) = (t0, point.timestamp) {
                point.datetime = Some(*t + ts);
            }
            // if let Some(timestamp) = point.timestamp {
            //     point.datetime = Some(*t0 + timestamp);
            // }
        }
    }

    /// Returns `true` if the first point in a cluster
    /// has a description and `false` otherwise.
    /// Returns `false` if the cluster is empty.
    fn is_marked(&self) -> bool {
        self.first()
            .map(|p| p.description.is_some())
            .unwrap_or(false)
    }
}
