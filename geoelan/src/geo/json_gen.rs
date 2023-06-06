//! Generate GeoJSON files according to `GeoShape` style.

use geojson::{
    Feature,
    GeoJson,
    Geometry,
    Value,
    feature::Id, FeatureCollection
};
use serde_json::{Map, to_value, Number};

use super::{geoshape::GeoShape, EafPoint};

/// Generate GeoJSON Feature ID from numerical value.
fn geojson_id(id: usize) -> Id {
    Id::Number(Number::from(id))
}

/// Generate GeoJSON properties from contents in `Point` (not kml or geojson crate point!).
fn geojson_properties(points: &[EafPoint]) -> Map<String, serde_json::Value> {
    let mut properties = Map::new();

    if let Some(descr) = points.first().and_then(|p| p.description.as_ref()) {
        properties.insert(
            String::from("description"),
            to_value(descr).unwrap()
        );
    }

    // Relative timestamp in milliseconds, for syncing
    if let Some(ts) = points.first().and_then(|p| p.timestamp.as_ref()) {
        let mut name = "timestamp";
        if points.len() > 1 {
            name = "timestamp_start"
        }
        properties.insert(
            String::from(name),
            to_value((ts.as_seconds_f64() * 1000.0) as i64).unwrap()
        );
    }
    if points.len() > 1 {
        if let Some(ts) = points.last().and_then(|p| p.timestamp.as_ref()) {
            properties.insert(
                String::from("timestamp_end"),
                // to_value(ts.num_milliseconds()).unwrap()
                to_value((ts.as_seconds_f64() * 1000.0) as i64).unwrap()
            );
        }
    }

    // Absolute timestamp
    if let Some(dt) = points.first().and_then(|p| p.datetime.as_ref()) {
        let mut name = "datetime";
        if points.len() > 1 {
            name = "datetime_start"
        }
        properties.insert(
            String::from(name),
            // to_value(dt.format("%Y-%m-%dT%H:%M:%S").to_string()).unwrap()
            to_value(dt.to_string()).unwrap()
        );
    }
    if points.len() > 1 {
        if let Some(dt) = points.last().and_then(|p| p.datetime.as_ref()) {
            properties.insert(
                String::from("datetime_end"),
                // to_value(dt.format("%Y-%m-%dT%H:%M:%S").to_string()).unwrap()
                to_value(dt.to_string()).unwrap()
            );
        }
    }

    properties
}

/// Generate GeoJSON point from `Point` (not kml or geojson crate point!)
pub fn geojson_point(point: &EafPoint, id: Option<usize>) -> Feature {
    let geometry = Geometry::new(
        Value::Point(vec!(point.longitude, point.latitude))
    );

    let properties = geojson_properties(&[point.to_owned()]);

    Feature {
        bbox: None,
        geometry: Some(geometry),
        id: id.map(geojson_id),
        properties: Some(properties),
        foreign_members: None,
    }
}

/// Generate GeoJSON line string from `Point`s (not kml or geojson crate point!)
pub fn geojson_linestring(points: &[EafPoint], id: Option<usize>) -> Feature {
    let linestring: Vec<Vec<f64>> = points.iter()
        .map(|p| vec!(p.longitude.to_owned(), p.latitude.to_owned()))
        .collect();
    let geometry = Geometry::new(Value::LineString(linestring));

    let properties = geojson_properties(points);

    Feature {
        bbox: None,
        geometry: Some(geometry),
        id: id.map(geojson_id),
        properties: Some(properties),
        foreign_members: None,
    }
}

/// Generate GeoJSON circle (GeoJSON polygon) from `Point` representing centre (not kml or geojson crate point!)
pub fn geojson_circle(center_point: &EafPoint, id: Option<usize>, radius: f64, vertices: u8) -> Feature {
    // Generate points representing a closed circle from center point
    let points = center_point.circle(radius, vertices);

    let polygon_outer: Vec<Vec<f64>> = points.iter()
        .map(|p| vec!(p.longitude.to_owned(), p.latitude.to_owned()))
        .collect();
    
    // Only need a solid polygon, i.e. circle, hence empty inner vec!()
    let geometry = Geometry::new(Value::Polygon(vec!(polygon_outer, vec!())));

    let properties = geojson_properties(&[center_point.to_owned()]);

    Feature {
        bbox: None,
        geometry: Some(geometry),
        id: id.map(geojson_id),
        properties: Some(properties),
        foreign_members: None,
    }
}

pub fn features_from_geoshape(points: &[EafPoint], geoshape: &GeoShape, count: Option<usize>) -> Vec<Feature> {
    let idx = count.unwrap_or(1);
    match geoshape {
        GeoShape::PointAll{..}
        | GeoShape::PointMulti{..}
        | GeoShape::PointSingle{..} => {
            points.iter()
                .enumerate()
                .map(|(i, point)| {
                    geojson_point(
                        point,
                        Some(count.unwrap_or(idx+i))
                    )
                })
                .collect()
        },
        GeoShape::LineAll{..}
        | GeoShape::LineMulti{..} => {
            vec!(geojson_linestring(
                    points,
                    Some(count.unwrap_or(idx))
                ))
        },
        GeoShape::Circle{radius, vertices, ..} => {
            points.iter()
                .enumerate()
                .map(|(i, p)| 
                    geojson_circle(
                        p,
                        Some(count.unwrap_or(idx+i)),
                        *radius,
                        *vertices
                    )
                )
                .collect()
        },
    }
}

pub fn geojson_from_features(features: &[Feature]) -> GeoJson {
    let collection = FeatureCollection{
        bbox: None,
        features: features.to_owned(),
        foreign_members: None
    };

    GeoJson::FeatureCollection(collection)
}

pub fn geojson_from_clusters(clusters: &[Vec<EafPoint>], geoshape: &GeoShape) -> GeoJson {
    let features: Vec<Feature> = clusters.into_iter()
        .enumerate()
        .flat_map(|(i, p)| features_from_geoshape(
                p,
                &geoshape,
                Some(i)
            )
        )
        .collect();

    geojson_from_features(&features)
}
