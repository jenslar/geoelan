//! Geometry output types.

use super::{downsample, EafPoint};

#[derive(Debug)]
/// Output geometry types
/// If the `height` field is set, KML output will use this
/// to extrude to this height in meters relative to the ground.
pub enum GeoShape {
    /// All points included.
    /// Those that intersect with an annotation
    /// timespan inherit the corresponding annotation value
    /// as description.
    PointAll{height: Option<f64>},
    /// Only points that intersect with an annotation
    /// timespan are included. These inherit the
    /// corresponding annotation value
    /// as description.
    PointMulti{height: Option<f64>},
    /// Points that intersect with an annotation
    /// timespan are averaged to a single point,
    /// which inherits the annotation value.
    PointSingle{height: Option<f64>},
    /// All points included, and joined as a
    /// poly line.
    /// Those that intersect with an annotation
    /// timespan inherit the corresponding annotation value
    /// as description.
    LineAll{height: Option<f64>},
    /// Only points that intersect with an annotation
    /// timespan are included for polyline generation.
    /// These inherit the corresponding annotation value
    /// as description.
    LineMulti{height: Option<f64>},
    /// Points that intersect with an annotation
    /// timespan are averaged to a single point,
    /// which inherits the annotation value. A circle is then generated
    /// using the `radius`, `vertices`, and the optional `height` values.
    /// I.e. point selection is exactly the same as for `PointSingle`,
    /// only representation differs.
    Circle{radius: f64, vertices: u8, height: Option<f64>},
}

impl GeoShape {
    pub fn to_string(&self) -> String {
        match self {
            GeoShape::PointAll{..} => "point-all".to_owned(),
            GeoShape::PointMulti{..} => "point-multi".to_owned(),
            GeoShape::PointSingle{..} => "point-single".to_owned(),
            GeoShape::LineAll{..} => "line-all".to_owned(),
            GeoShape::LineMulti{..} => "line-multi".to_owned(),
            GeoShape::Circle{..} => "circle".to_owned(),
        }
    }
}

/// Returns `true` if the first point in a cluster
/// has a description and `false` otherwise.
/// Returns `false` if the cluster is empty.
fn is_marked(point_cluster: &[EafPoint]) -> bool {
    point_cluster
        .first()
        .map(|p| p.description.is_some())
        .unwrap_or(false)
}

/// Filters and downsamples point clusters.
/// Ensures poly-lines will have at least two points,
/// and that any point variants will return at least
/// a single point, regardless of `downsample_factor`.
pub fn filter_downsample(
    point_clusters: &[Vec<EafPoint>],
    downsample_factor: Option<usize>,
    geoshape: &GeoShape,
) -> Vec<Vec<EafPoint>> {
    let sample_factor = downsample_factor.unwrap_or(1);

    // Store last point in cluster to generate continuous lines for 'line-all'
    let mut last_point: Option<EafPoint> = None;

    // 1. Filter out unmarked clusters for some geoshapes
    let filtered_clusters: Vec<Vec<EafPoint>> = match geoshape {
        
        // All points preserved
        GeoShape::PointAll{..} => point_clusters.iter()
            .map(|cluster| downsample(sample_factor, cluster, None))
            .collect(),

        // Discard marked points/points without description
        GeoShape::PointMulti{..} => point_clusters.iter()
            .filter_map(|cluster|
                if is_marked(cluster) {
                    Some(downsample(sample_factor, cluster, None))
                } else {
                    None
                }
            )
            .collect(),
            
        // All points preserved and transformed to polylines.
        // Alters between marked and unmarked events.
        GeoShape::LineAll{..} => point_clusters.iter()
            // min 2 points for line
            .map(|cluster| {
                // Possibly bad way of adding point in previous cluster
                // as first point in current one to generate
                // continuous poly-line.
                let mut downsampled: Vec<EafPoint> = Vec::new();
                let description = cluster.first().and_then(|p| p.description.as_ref());
                if let Some(lp) = last_point.as_mut() {
                    lp.description = description.cloned();
                    downsampled.push(lp.to_owned())
                }
                downsampled.extend(downsample(sample_factor, cluster, Some(2)));
                last_point = downsampled.last().cloned();
                downsampled
            })
            .collect(),
            
        // Discard marked points/points without description,
        // then transform to broken-up polylines.
        GeoShape::LineMulti{..} => point_clusters.iter()
            .filter_map(|cluster|
                if is_marked(cluster) {
                    // minimum of 2 points for polylines
                    Some(downsample(sample_factor, cluster, Some(2)))
                } else {
                    None
                }
            )
            .collect(),
            
        // Discard marked points/points without description,
        // ignore sample factor,
        // and downsample each cluster to single point or
        // polygonal circle (with single point becoming its center).
        GeoShape::PointSingle{..}
        | GeoShape::Circle{..} => point_clusters.iter()
            .filter_map(|cluster|
                if is_marked(cluster) {
                    Some(downsample(cluster.len(), cluster, None))
                } else {
                    None
                }
            )
            .collect()
    };

    filtered_clusters
}
