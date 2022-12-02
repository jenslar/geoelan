//! `PointCluster`. Convenience struct for processing, including downsampling (latitude dependent).

use fit_rs::GpsMetadata;
use gpmf_rs::GoProPoint;
use time::{PrimitiveDateTime, Duration};

/// Point cluster with optional description.
#[derive(Debug, Default, Clone)]
pub struct PointCluster{
    pub points: Vec<super::point::Point>,
    pub description: Option<String>,
    // pub geoshape: GeoShape
}

impl PointCluster {
    pub fn new(points: &[super::point::Point], description: Option<&str>) -> Self {
        Self {
            points: points.to_owned(),
            description: description.map(String::from)
        }
    }

    /// Convert a Garmin VIRB point slice to a point cluster.
    pub fn from_virb(
        points: &[GpsMetadata],
        description: Option<&str>,
        t0: &PrimitiveDateTime,
        end: &Duration
    ) -> Self {
        let mut cluster = Self::default();

        cluster.description = description.map(String::from);
        cluster.points = points.iter()
            .map(|point| crate::geo::point::Point::from(point))
            .collect();
        
        cluster.set_virbtime(t0, end);
        
        cluster
    }

    /// Convert a GoPro point slice to a point cluster.
    pub fn from_gopro(points: &[GoProPoint], description: Option<&str>) -> Self {
        let mut cluster = Self::default();

        cluster.description = description.map(String::from);
        cluster.points = points.iter()
            .map(|point| crate::geo::point::Point::from(point))
            .collect();
        
        cluster
    }

    /// Set time offset in hours.
    pub fn offset_hrs(&mut self, offset: i64) {
        self.points.iter_mut()
            .for_each(|point| {
                // Add time offset 
                if let Some(dt) = point.datetime {
                    point.datetime = Some(dt + Duration::hours(offset))
                }
            });
    }

    /// Get first point.
    pub fn first(&self) -> Option<&crate::geo::point::Point> {
        self.points.first()
    }
    
    /// Get last point.
    pub fn last(&self) -> Option<&crate::geo::point::Point> {
        self.points.last()
    }
    
    /// Iterate over points.
    pub fn iter(&self) -> impl Iterator<Item = &crate::geo::point::Point> {
        self.points.iter()
    }
    
    /// Mutably iterate over points.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut crate::geo::point::Point> {
        self.points.iter_mut()
    }
    
    /// Number of points in cluster.
    pub fn len(&self) -> usize {
        self.points.len()
    }
    
    /// Downsample points. A `sample_factor` of 10 means 1000 points
    /// will be averaged into 100 points and so on.
    pub fn downsample(&mut self, sample_factor: usize, min: Option<usize>) {
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

    /// Sets date time for Garmin VIRB points.
    /// Not needed if source device  is a GoPro camera.
    pub fn set_virbtime(&mut self, t0: &PrimitiveDateTime, end: &Duration) {
        // max index for windows(2)
        let max = self.len() - 2;

        let mut delta = Vec::new();

        // 1. Populate timestamp/duration/delta
        //    Can't iter_mut with .windows()?
        for (i, points) in self.points.windows(2).enumerate() {
            if let (Some(t1), Some(t2)) = (points.get(0).and_then(|p| p.timestamp), points.get(1).and_then(|p| p.timestamp)) {
                delta.push(t2 - t1);
                if i == max {
                    delta.push(end.to_owned() - t2);
                }
            }
        };

        assert_eq!(self.points.len(), delta.len());

        for (point, duration) in self.iter_mut().zip(delta) {
            point.duration = Some(duration);
            if let Some(timestamp) = point.timestamp {
                point.datetime = Some(*t0 + timestamp);
            }
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