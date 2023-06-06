//! Extract and georeference ELAN-annotations, and export as KML + GeoJSON.

use std::{path::PathBuf, collections::{HashSet, HashMap}, io::ErrorKind};

use time::Duration;
use eaf_rs::Eaf;
use kml::types::{Placemark, Element};

use crate::{
    geo::{
        geoshape::{GeoShape, filter_downsample},
        kml_gen::{placemarks_from_geoshape, kml_from_placemarks, kml_to_string, kml_style},
        json_gen::geojson_from_clusters,
        kml_styles::Rgba,
        EafPoint
    },
    elan::select_tier,
    files
};
mod gopro2points;
mod virb2points;


pub fn run(args: &clap::ArgMatches) -> std::io::Result<()> {

    // clap: required arg
    let eaf_path = args.get_one::<PathBuf>("eaf").unwrap().to_owned();
    let use_geotier = *args.get_one::<bool>("geotier").unwrap();
    let fit_present = args.contains_id("fit");
    let gpmf_present = args.contains_id("gpmf");

    // Parse EAF early in case 'geotier' is set.
    let eaf = Eaf::de(&eaf_path, true)?;

    // Extract points from either VIRB, GoPro, or annotation data.
    let mut points = match (fit_present, gpmf_present, use_geotier) {
        (true, false, false) => virb2points::run(args)?,
        (false, true, false) => gopro2points::run(args)?,
        (false, false, true) => {
            print!("[GEO TIER] ");
            let geotier = select_tier(&eaf, true)?;

            // Try to parse annotations into coordinates.
            // Will use default values if parsing fails.
            geotier.iter()
                .map(|annotation| EafPoint::from(annotation))
                .collect::<Vec<_>>()
        },
        _ => {
            let msg = "(!) Can only specify one of 'gpmf', 'fit', 'geotier'";
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        }
    };

    if points.is_empty() {
        let msg = "(!) No points to process.";
        return Err(std::io::Error::new(ErrorKind::Other, msg))
    }

    let time_offset = *args.get_one::<isize>("time-offset").unwrap(); // clap default: 0

    // clap: default 1
    let downsample_factor = args.get_one::<usize>("downsample-factor")
        .unwrap().to_owned();
    if downsample_factor == 0 {
        let msg = "(!) 'downsample' can not be 0.";
        return Err(std::io::Error::new(ErrorKind::Other, msg))
    }

    // clap: default 1
    let radius = args.get_one::<f64>("radius").unwrap().to_owned();
    if !(radius > 0.0) {
        let msg = "(!) 'radius' must be a positive float.";
        return Err(std::io::Error::new(ErrorKind::Other, msg))
    }

    // clap default: 40, range: 3 .. 255 (min value checked later)
    let vertices = args.get_one::<u8>("vertices").unwrap().to_owned();

    // clap: default 0.0
    //       if > 0.0 KML-files will use height to extrude
    //       relative to ground
    let height: Option<f64> = args.get_one("height").cloned();
    if let Some(h) = &height {
        if !(h > &0.0) {
            let msg = "(!) 'height' must be a positive float.";
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        }
    }

    // clap: default 'point-all'
    let geoshape_arg = args.get_one::<String>("geoshape").unwrap();
    let geoshape = match geoshape_arg.as_str() {
        // TODO 220627 change extrude to all shapes to take height then use height.is_some() to set extrude
        "point-all" => GeoShape::PointAll{height},
        "point-multi" => GeoShape::PointMulti{height},
        "point-single" => GeoShape::PointSingle{height},
        "line-all" => GeoShape::LineAll{height},
        "line-multi" => GeoShape::LineMulti{height},
        "circle" => GeoShape::Circle{radius, vertices, height},
        // Final branch should never be reached, since clap sets default to 'points-all'
        // and checks valid values.
        shape => {
            let msg = format!("(!) Invalid 'geoshape' value '{shape}'.");
            return Err(std::io::Error::new(ErrorKind::Other, msg))
        },
    };

    // Important: Cluster points BEFORE downsampling,
    //            since will otherwise risk not having points corresponding
    //            to annotation time spans, short ones especially.


    print!("[CONTENT TIER] ");
    let tier = select_tier(&eaf, true)?;

    print!("Mapping annotation values and downsampling points...");
    // For performance reasons outer iteration is points,
    // since these usually outnumber number of annotations in a tier.
    for point in points.iter_mut() {
        
        // Add offset hours to datetime
        point.datetime = point.datetime.map(|dt| dt + Duration::hours(time_offset as i64));
        
        // Map annotation value to point.description if
        // the point's relative timestamp is within
        // the annotation's time span.
        if let Some(t_point) = point.timestamp_ms() {
            tier.annotations.iter()
                .find(|a| {
                    if let (Some(t_annot_start), Some(t_annot_end)) = a.ts_val() {
                        // TODO 1a. Fix edge cases for annotations short enough not to be "contained" by a point "time span".
                        // TODO 2a. Include points that are logged close to annotation start/end, but at what thresh hold?
                        // TODO 2b. 2a may introduce edge cases for back-to-back annotations so perhaps not?
                        // TODO 1a + 1b. VIRB, logs at 10Hz so threshold < 100ms? GoPro logs at 1Hz (clusters) so threshold < 1000ms?
                        t_point > t_annot_start && t_point < t_annot_end // point logged within annotation boundaries
                    } else {
                        false
                    }
                })
                .map(|a| point.description = Some(a.value().to_string()));
        }
    }

    // 'group_by()' is exactly what is needed but it's unstable/nightly only,
    // see issue #80552: https://github.com/rust-lang/rust/issues/80552
    // let point_clusters = points.group_by(|p1, p2| p1.description == p2.description)

    let mut point_clusters: Vec<Vec<EafPoint>> = Vec::new();
    if points.len() > 1 {
        // Add first point to point_slice as comparison
        let mut point_slice = vec!(points[0].to_owned());

        // Start iterating from point two and on
        // for comparison with last point in point_slice
        points.iter().skip(1)
            .for_each(|pt| {
                if let Some(p) = point_slice.last() {
                    if p.description == pt.description {
                        point_slice.push(pt.to_owned())
                    } else {
                        point_clusters.push(point_slice.to_owned());
                        point_slice = vec!(pt.to_owned())
                    }
                }
            });
        
        // Push final point_slice
        if !point_slice.is_empty() {
            point_clusters.push(point_slice.to_owned());
        }
    }

    let downsampled_clusters = filter_downsample(&point_clusters, Some(downsample_factor), &geoshape);
    println!(" Done.");

    println!("Resulting point clusters with downsample factor {downsample_factor} and geoshape '{}':", geoshape.to_string());
    // For comparing original point count with downsampled result.
    let before_after: Vec<(usize, usize)> = point_clusters.as_slice().iter()
        .zip(downsampled_clusters.as_slice())
        .map(|(bef, aft)| (bef.len(), aft.len()))
        .collect();
    // Keeping track of unique annotation values for generating
    // KML style ID so that for poly-lines, lines with the same
    // description get the same colour.
    let mut unique_annotations: HashSet<String> = HashSet::new();

    for (i, cluster) in downsampled_clusters.iter().enumerate() {

        // Compile unique annotations to generate KML styles
        // where lines with the same description get the same colour.
        let description = cluster.first().and_then(|p| p.description.as_deref());
        if let Some(descr) = description {
            unique_annotations.insert(descr.to_owned());
        }
        
        // indeces should exist and match, compare points before, after downsample
        let (before, after) = before_after.get(i).map(|(bef, aft)| (bef, aft)).unwrap_or((&0, &0));

        println!("{:4}. {:5} -> {:5} points. Description: {}",
            i + 1,
            before,
            after,
            description.unwrap_or("NONE")
        )
    }

    println!("Generating KML and GeoJSON...");
    // KML-only: Substitute basic Placemark description with HTML CDATA 
    let cdata = *args.get_one::<bool>("cdata").unwrap();
    // Generate KML styles via unique annotation values
    let kml_style_id: HashMap<String, (String, Rgba)> = unique_annotations.iter()
        .enumerate()
        .map(|(i, s)| (s.to_owned(), (format!("style{}", i+1), Rgba::random(None))))
        .collect();
    let mut kml_styles: Vec<Element> = kml_style_id.iter()
        .map(|(_, (id, color))| kml_style(id, &geoshape, color))
        .collect();
    kml_styles.sort_by_key(|e| e.name.to_owned());

    // Generate KML
    let placemarks: Vec<Placemark> = downsampled_clusters.iter()
        .enumerate()
        .flat_map(|(i, p)| placemarks_from_geoshape(
            p,
            &geoshape,
            None,
            cdata,
            &kml_style_id,
            Some(i+1)
        ))
        .collect();
    let kml = kml_from_placemarks(&placemarks, &kml_styles);

    // Serialize to KML v2.2. No line breaks/indentation.
    let kml_doc = kml_to_string(&kml);
    let kml_path = files::affix_file_name(&eaf_path, None, Some(geoshape_arg), Some("kml"));

    match files::writefile(&kml_doc.as_bytes(), &kml_path) {
        Ok(true) => println!("Wrote {}", kml_path.display()),
        Ok(false) => println!("User aborted writing KML-file"),
        Err(err) => return Err(err),
    }

    // Generate GeoJSON
    let geojson = geojson_from_clusters(&downsampled_clusters, &geoshape);

    // Serialize GeoJSON. Not indented (= smaller size for web use).
    let geojson_doc = geojson.to_string();
    let geojson_path = files::affix_file_name(&eaf_path, None, Some(geoshape_arg), Some("geojson"));

    match files::writefile(&geojson_doc.as_bytes(), &geojson_path) {
        Ok(true) => println!("Wrote {}", geojson_path.display()),
        Ok(false) => println!("User aborted writing JSON-file"),
        Err(err) => return Err(err),
    }


    // Print results
    let first_point = downsampled_clusters.first().and_then(|c| c.first());
    let first_annotated_point = downsampled_clusters.iter()                 // iter outer vec
        .find(|c| c.first().and_then(|p| p.description.as_ref()).is_some()) // find first point with descr in inner vec
        .and_then(|c| c.first());                                           // return first item in inner vec
    let first_annotation = tier.first();
    let georefed_annotations = downsampled_clusters.iter()
        .filter_map(|c| c.first().and_then(|p| p.description.to_owned()))
        .collect::<Vec<String>>();

    if let Some(annotation) = first_annotation {
        println!("Relative time stamps:");
        print!("  First annotation:   ");
        if let (Some(t1), Some(t2)) = annotation.ts_val() {
            println!("    {t1:8} ms - {t2:8} ms '{}'", annotation.value())
        } else {
            println!("(!) No relative time set for annotation:\n    {annotation:?}")
        }
    }
    
    if let (Some(point), Some(point_annot)) = (first_point, first_annotated_point) {
        print!("  First processed point:  ");
        if let Some(t) = point.timestamp_ms() {
            print!("{t:8} ms")
        } else {
            print!("(!) No relative time set for point:\n    {point}")
        }
        println!(" (not first point in GPS log)");
        print!("  First annotated point:  ");
        if let (Some(t), Some(txt)) = (point_annot.timestamp_ms(), point_annot.description.as_ref()) {
            println!("{t:8} ms '{txt}'")
        } else {
            println!("(!) No relative time set for point:\n    {point}")
        }
    }

    println!("Annotations:");
    println!("  Geo-referenced:        {:4} annotations", georefed_annotations.len());
    println!("  Discarded:             {:4} annotations (preceed GPS logging start time)",
        tier.len() - georefed_annotations.len()
    );

    Ok(())
}