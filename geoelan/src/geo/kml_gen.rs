//! Generate KML files according to `GeoShape` style

use std::collections::HashMap;
use kml::{
    Kml,
    types::{
        AltitudeMode,
        Coord,
        Point,
        Placemark,
        Geometry,
        LineString,
        LinearRing,
        Element
    },
    KmlDocument
};
use time::PrimitiveDateTime;

use super::{
    geoshape::GeoShape,
    kml_styles::{
        Rgba,
        KmlStyle,
        KmlPolyStyle,
        KmlLineStyle,
        KmlStyleType
    }, EafPoint
};

pub fn kml_to_string(doc: &KmlDocument) -> String {
    vec![
        // Add XML declaration
        "<?xml version='1.0' encoding='utf-8'?>".to_owned(),
        Kml::KmlDocument(doc.to_owned()).to_string()
    ].join("")
}

/// Generate KML document from geometries in `element`
pub fn kml_from_placemarks(placemarks: &[Placemark], styles: &[Element]) -> KmlDocument {
    // <kml ...> attributes
    let attr = HashMap::from([
        ("xmlns".to_owned(), "http://www.opengis.net/kml/2.2".to_owned()),
        ("xmlns:gx".to_owned(), "http://www.google.com/kml/ext/2.2".to_owned())
    ]);
    
    let mut elements: Vec<Kml> = Vec::new();

    for style in styles.iter() {
        elements.push(Kml::Element(style.to_owned()))
    }

    for placemark in placemarks.iter() {
        elements.push(Kml::Placemark(placemark.to_owned()))
    }

    let doc = Kml::Document{
        attrs: HashMap::new(), // no attribs at this level
        elements
    };

    KmlDocument{
        version: kml::KmlVersion::V22,
        attrs: attr,
        elements: vec!(doc)
    }
}

/// KML style URL element
fn kml_styleurl(id: &str) -> Element {
    Element{
        name: "styleUrl".to_owned(),
        attrs: HashMap::new(),
        content: Some(format!("#{id}")),
        children: Vec::new()
    }
}

/// KML style definition element
pub fn kml_style(id: &str, geoshape: &GeoShape, color: &Rgba) -> Element {
    let mut style = KmlStyle::default();
    style.id = id.to_owned();

    match &geoshape {
        GeoShape::Circle{..} => {
            let mut poly = KmlPolyStyle::default();
            poly.color = color.to_owned();
            
            // Set line style as well, since it will be used for poly lines
            let mut line = KmlLineStyle::default();
            line.width = 1.0;
            line.color = Rgba::white().with_alpha(40);
            
            style.styles.push(KmlStyleType::KmlLineStyle(line));
            style.styles.push(KmlStyleType::KmlPolyStyle(poly));
        },
        GeoShape::LineAll {..}
        | GeoShape::LineMulti {..} => {
            let mut line = KmlLineStyle::default();
            line.color = color.to_owned();

            style.styles.push(KmlStyleType::KmlLineStyle(line));
        },
        GeoShape::PointAll{..}
        | GeoShape::PointMulti{..}
        | GeoShape::PointSingle{..} => ()
    }

    style.to_element()
}

/// Timestamp for use within Placemark.
/// If `datetime_end` = `None` the result will be:
/// `<TimeStamp><when>2021-06-22T11:40:38.319</when></TimeStamp>`.
/// 
/// Otherwise a timespan is generated:
/// `<TimeSpan><begin>2021-06-22T11:40:34.119</begin><end>2021-06-22T11:40:42.519</end></TimeSpan>`,
/// which is useful for e.g. polylines.
pub fn kml_timestamp(datetime_start: &PrimitiveDateTime, datetime_end: Option<&PrimitiveDateTime>) -> Element {
    let mut name = "TimeStamp".to_owned();
    let children = match datetime_end {
        Some(dt_end) => {
            name = "TimeSpan".to_owned();
            let mut start = Element::default();
            start.name = "begin".to_owned();
            start.content = Some(datetime_start.to_string()); // TODO 220809 check default PrimitiveDateTime.to_string format, maybe not correct
            let mut end = Element::default();
            end.name = "end".to_owned();
            end.content = Some(dt_end.to_string()); // TODO 220809 check default PrimitiveDateTime.to_string format, maybe not correct

            vec!(start, end)
        },
        None => vec!(Element{
            name: "when".to_owned(),
            attrs: HashMap::new(),
            content: Some(datetime_start.to_string()), // TODO 220809 check default PrimitiveDateTime.to_string format, maybe not correct
            children: Vec::new()
        })
    };

    Element{
        name,
        attrs: HashMap::new(),
        content: None,
        children
    }
}

/// For pop-up bubbles in Google Earth etc.
/// No explicit CDATA tag, since it currently is serialized with escapes,
/// but since KML allows for HTML with escaped characters,
/// and quick-xml escapes '<' etc, the CDATA tag shouldn't be needed.
/// Currently works at least in Google Earth Desktop.
pub fn kml_cdata(
    point_start: &EafPoint,
    point_end: Option<&EafPoint>
) -> String {
    let p_start = format!("<tr><td>{} (lat, lon): {}, {}</td></tr>",
        if point_end.is_some() {"Coordinate, start"} else {"Coordinate"},
        point_start.latitude,
        point_start.longitude
    );
    let t_start = match point_start.datetime {
        Some(dt) => format!("<tr><td>{}: {}</td></tr>",
            if point_end.is_some() {"Time, start"} else {"Time"},
            // dt.format("%Y-%m-%dT%H:%M:%S").to_string()
            dt.to_string() // TODO 220809 check default PrimitiveDateTime.to_string format, maybe not correct
        ),
        None => "Not specified".to_owned()
    };
    let p_end = point_end.map(|p| format!("<tr><td>Coordinate, end (lat, lon): {}, {}</td></tr>",
        p.latitude,
        p.longitude
    ));
    let t_end = point_end.and_then(|p| p.datetime)
        .map(|dt| format!("<tr><td>Time, end: {}</td></tr>",
            // dt.format("%Y-%m-%dT%H:%M:%S").to_string())
            dt.to_string()) // TODO 220809 check default PrimitiveDateTime.to_string format, maybe not correct
        );

    let mut content: Vec<String> = vec!(
        "<table>".to_owned(),
        format!("<tr><td>Description: {}</td></tr>", point_start.description.as_deref().unwrap_or("No description"))
    );

    content.push(p_start);
    if let Some(end) = p_end {
        content.push(end)
    }
    content.push(t_start);
    if let Some(end) = t_end {
        content.push(end)
    }

    content.push("</table>".to_owned());

    content.join("")
}

pub fn kml_point(
    point: &EafPoint,
    name: Option<&str>,
    height: Option<&f64>,
    cdata: bool,
    style_url: Option<&str>
) -> Placemark {
    let mut kml_point = Point::new(
        point.longitude,
        point.latitude,
        Some(point.altitude)
    );

    let mut children: Vec<Element> = point.datetime
        .map(|dt| vec!(kml_timestamp(&dt, None)))
        .unwrap_or(Vec::new());
    
    if let Some(style) = style_url {
        children.push(kml_styleurl(style))
    }

    let description = match cdata {
        true => Some(kml_cdata(point, None)),
        false => point.description.to_owned()
    };

    if let Some(h) = height {
        kml_point.coord.z = Some(*h);
        kml_point.extrude = true;
        // LinearString defaults to ClampToGround
        kml_point.altitude_mode = AltitudeMode::RelativeToGround
    }

    Placemark{
        name: name.map(String::from),
        description,
        geometry: Some(Geometry::Point(kml_point)),
        attrs: HashMap::new(),
        children // styles, cdata etc
    }
}

/// Generates KML line string, aka "poly-line", with datetime if set in specified points.
pub fn kml_linestring(
    points: &[EafPoint],
    name: Option<&str>,
    height: Option<&f64>,
    cdata: bool,
    style_url: Option<&str>
) -> Placemark {
    // Get description from first point
    let mut description = points.first()
        .and_then(|p| p.description.to_owned());

    if cdata {
        if let (Some(p1), Some(p2)) = (points.first(), points.last()) {
            description = Some(kml_cdata(p1, Some(p2)));
        }
    }

    let coords: Vec<_> = points.iter()
        .map(|p| Coord::new(
            p.longitude,
            p.latitude,
            Some(p.altitude)))
        .collect();
    
    let mut children: Vec<Element> = match (points.first().and_then(|p| p.datetime), points.last().and_then(|p| p.datetime)) {
        (Some(t1), Some(t2)) => {
            vec!(kml_timestamp(&t1, Some(&t2)))
        },
        _ => Vec::new()
    };

    if let Some(style) = style_url {
        children.push(kml_styleurl(style))
    }

    let mut linestring = LineString::from(coords);

    // Use 'height' as altitude (z) value if set
    if let Some(h) = height {
        linestring.coords.iter_mut()
            .for_each(|c| c.z = Some(*h));
        linestring.extrude = true;
        // LinearString defaults to ClampToGround
        linestring.altitude_mode = AltitudeMode::RelativeToGround
    }

    Placemark{
        name: name.map(String::from),
        description,
        geometry: Some(Geometry::LineString(linestring)),
        attrs: HashMap::new(),
        children // styles, cdata etc
    }
}

/// For geoshape 2D/3D circle.
/// clamp = true set altitude mode to `ClampToGround`,
/// otherwise set to `RelativeToGround`
pub fn kml_linearring(
    center_point: &EafPoint, // not kml crate Point!
    name: Option<&str>,
    radius: f64,
    vertices: u8,
    // extrude: bool,
    height: Option<&f64>,
    // relative: bool,
    cdata: bool,
    style_url: Option<&str>
    // TODO add timestamp (for center coord)
) -> Placemark {
    let mut center = center_point.to_owned();

    if let Some(h) = height {
        center.altitude = *h
    }
    
    // Get description from first point
    let description = match cdata {
        true => Some(kml_cdata(&center, None)),
        false => center.description.to_owned()
    };

    let mut children: Vec<Element> = center.datetime
        .map(|dt| vec!(kml_timestamp(&dt, None)))
        .unwrap_or(Vec::new());

    if let Some(style) = style_url {
        children.push(kml_styleurl(style))
    }
    
    let circle_points = center.circle(radius, vertices);

    let coords: Vec<_> = circle_points.iter()
        .map(|p| Coord::new(p.longitude, p.latitude, Some(p.altitude)))
        .collect();
    
    let mut linearring = LinearRing::from(coords);

    // Use 'height' as altitude (z) value if set
    if let Some(h) = height {
        linearring.coords.iter_mut()
            .for_each(|c| c.z = Some(*h));
        linearring.extrude = true;
        // LinearString defaults to ClampToGround
        linearring.altitude_mode = AltitudeMode::RelativeToGround
    }

    Placemark{
        name: name.map(String::from),
        // description:  if cdata {None} else {description},
        description,
        geometry: Some(Geometry::LinearRing(linearring)),
        attrs: HashMap::new(),
        children // styles, cdata etc
    }
}

pub fn placemarks_from_geoshape(
    points: &[EafPoint],
    geoshape: &GeoShape,
    name: Option<&str>,
    cdata: bool,
    styles: &HashMap<String, (String, Rgba)>,
    count: Option<usize>
) -> Vec<Placemark> {
    let idx = count.unwrap_or(1);
    match geoshape {
        GeoShape::PointAll{height}
        | GeoShape::PointMulti{height}
        | GeoShape::PointSingle{height} => {
            points.iter()
                .enumerate()
                .map(|(i, point)| {
                    let style = point.description.as_deref()
                        .and_then(|s| styles.get(s))
                        .map(|(s, _)| s.as_str());
                    kml_point(
                        point,
                        Some(name.unwrap_or(&format!("{}", idx+i+1))),
                        height.as_ref(),
                        cdata,
                        style
                    )
                })
                .collect()
        },
        GeoShape::LineAll{height}
        | GeoShape::LineMulti{height} => {
            let style = points.first()
                .and_then(|p| p.description.as_deref())
                .and_then(|s| styles.get(s))
                .map(|(s, _)| s.as_str());
            vec!(kml_linestring(
                points,
                Some(name.unwrap_or(&format!("{}", idx+1))),
                height.as_ref(),
                cdata,
                style
            ))
        },
        // GeoShape::Circle2d{radius, vertices}
        // | GeoShape::Circle3d{radius, vertices} => {
        // GeoShape::Circle{radius, vertices, extrude, height} => {
        GeoShape::Circle{radius, vertices, height} => {
            points.iter()
                .enumerate()
                .map(|(i, point)| {
                    let style = point.description.as_deref()
                        .and_then(|s| styles.get(s))
                        .map(|(s, _)| s.as_str());
                    kml_linearring(
                        point,
                        Some(name.unwrap_or(&format!("{}", idx+i))),
                        *radius,
                        *vertices,
                        // *extrude,
                        // false,
                        height.as_ref(),
                        cdata,
                        style
                    )
                    
                })
                .collect()
        },
    }
}
