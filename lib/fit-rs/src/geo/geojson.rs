use geojson::{Feature, Value, Geometry, Position, PointType, LineStringType};

use crate::GpsMetadata;

// pub enum FeatureType {
//     Point,
//     LineString
// }

// impl From<GpsMetadata> for Position {
//     /// Generates GeoJSON `Position` with single point geometry.
//     fn from(gps: GpsMetadata) -> Self {
//         gps.to_decimal().to_vec()
//     }
// }

impl From<GpsMetadata> for PointType {
    fn from(gps: GpsMetadata) -> Self {
        gps.to_decimal().to_vec()
    }
}

// impl From<Vec<GpsMetadata>> for LineStringType {
//     fn from(gps: Vec<GpsMetadata>) -> Self {
//         gps.iter()
//             .map(|p| p.to_decimal().to_vec())
//             .collect()
//     }
// }