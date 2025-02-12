//! Geo-related functions, including generating KML + GeoJSON.

use time::Duration;

pub mod geo_fit;
pub mod geo_gpmf;
pub mod geoshape;
pub mod json_gen;
pub mod kml_gen;
pub mod kml_styles;
pub mod point;
pub mod point_cluster;

pub use point::EafPoint;
pub use point_cluster::EafPointCluster;

fn average(nums: &[f64]) -> f64 {
    nums.iter().sum::<f64>() / nums.len() as f64
}

/// Downsample points.
/// Clusters points in sizes equal to `sample_factor`,
/// then downsamples each sub-cluster to a single point.
/// Optionally set a minimum number of points to return via `min`.
/// If `sample_factor` results in fewer points than `min`,
/// `min` will be used in its place.
pub fn downsample(
    mut sample_factor: usize,
    points: &[point::EafPoint],
    min: Option<usize>,
) -> Vec<point::EafPoint> {
    match sample_factor {
        0 => panic!("Sample factor cannot be 0."), // avoid division by 0
        1 => return points.to_vec(),
        // ensure downsampling will at lest yield a single point
        f if f > points.len() => sample_factor = points.len(),
        _ => (),
    }

    // Int division for checking if downsample factor
    // causes fewer than optionally set min number of points
    if let Some(m) = min {
        // if points.len() / sample_factor < 2 { // shouldn't "< 2" be "< m"?
        if points.len() / sample_factor < m {
            // 220914 "< m" IS UNTESTED
            // div_ceil will be in upcoming rust version:
            // https://github.com/rust-lang/rfcs/issues/2844
            // sample_factor = points.len().div_ceil(m)
            // sample_factor = (points.len() as f64 / m as f64).ceil() as usize // should this be .floor()?
            // 220914 changed to .floor() since otherwise
            sample_factor = (points.len() as f64 / m as f64).floor() as usize // .floor() IS UNTESTED
        }
    }

    // let initial_sample_factor = sample_factor; // changed if remaining points < sample_factor

    // let mut average: Vec<point::EafPoint> = Vec::new();

    // // splits into even chunks, with possible remainder.len() < sample_factor
    // for cluster in points.chunks(sample_factor) {
    //     average.push(point_cluster_average(&cluster));
    // }

    points
        .chunks(sample_factor)
        .map(|c| point_cluster_average(c))
        .collect::<Vec<_>>()

    // TODO could perhaps iter over points using point.chunks(sample_factor) + remainder?
    // for idx in (0..points.len()).step_by(sample_factor) {
    //     average.push(point_cluster_average(&points[idx..idx + sample_factor]));

    //     // Need to check step size before last loop, i.e. before last idx+stepsize
    //     // or risk out of bounds if points.len() % initial_sample_factor != 0,
    //     // hence 2*samplefactor.
    //     // Sets sample factor to len of remaining points if < initial samplefactor
    //     if initial_sample_factor > 1 && 2 * sample_factor > points.len() - idx {
    //         sample_factor = points.len() - idx - sample_factor;
    //     }
    // }

    // average
}

/// Returns latitude dependent average for specified coordinate cluster.
// pub fn point_cluster_average(points: &[Point], text: Option<&str>) -> Point {
pub fn point_cluster_average(points: &[point::EafPoint]) -> point::EafPoint {
    // see: https://carto.com/blog/center-of-points/
    // atan2(y,x) where y = sum((sin(yi)+...+sin(yn))/n), x = sum((cos(xi)+...cos(xn))/n), y, i in radians
    // note that this currently does a f64 conversion/cast from degrees to radians and back to degrees

    let description = points.first().and_then(|p| p.description.to_owned());
    let ts_first = points.first().and_then(|p| p.timestamp);
    // Note: if points have no duration set, resulting eaf
    //       will have incorrect annotation boundaries on the geotier.
    let dur_total: Duration = points.iter().filter_map(|p| p.duration).sum();

    let deg2rad = std::f64::consts::PI / 180.0; // inverse for radians to degress

    let mut lon_rad_sin: Vec<f64> = Vec::new(); // sin values
    let mut lon_rad_cos: Vec<f64> = Vec::new(); // cos values
    let mut lat_rad: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut alt: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut hdg: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut sp2d: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut sp3d: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut time_as_ms: Vec<i64> = Vec::new();

    for pt in points.iter() {
        lon_rad_sin.push((pt.longitude * deg2rad).sin()); // get the sin values immediately
        lon_rad_cos.push((pt.longitude * deg2rad).cos()); // get the cos values immediately
        lat_rad.push(pt.latitude * deg2rad); // arithmetic avg ok, only converts to radians
        alt.push(pt.altitude);
        if let Some(h) = pt.heading {
            hdg.push(h)
        }
        sp2d.push(pt.speed2d);
        sp3d.push(pt.speed3d);
        time_as_ms.push(
            pt.timestamp
                .map(|t| (t.as_seconds_f64() * 1000.0) as i64)
                .unwrap_or(0),
        );
    }

    // AVERAGING LATITUDE DEPENDANT LONGITUDES
    let lon_rad_sin_sum = average(&lon_rad_sin);
    let lon_rad_cos_sum = average(&lon_rad_cos);
    let lon_avg_deg = f64::atan2(lon_rad_sin_sum, lon_rad_cos_sum) / deg2rad; // -> degrees
    let lat_avg_deg = average(&lat_rad) / deg2rad; // -> degrees
    let alt_avg = average(&alt);
    let hdg_avg = match hdg.is_empty() {
        true => None,
        false => Some(average(&hdg)),
    };
    let sp2d_avg = average(&sp2d);
    let sp3d_avg = average(&sp3d);
    // let time_avg = Duration::milliseconds(
    //     time_as_ms.iter().sum::<i64>() / points.len() as i64, // may be off by 1ms since no float+round
    // );
    // TODO untested, added 230107
    // TODO no longer used 230403
    // let time_avg = {
    //     let sum = time_as_ms.iter().sum::<i64>() as f64;
    //     let avg = sum / points.len() as f64;
    //     Duration::milliseconds(avg.round() as i64)
    // };

    point::EafPoint {
        latitude: lat_avg_deg,
        longitude: lon_avg_deg,
        altitude: alt_avg,
        heading: hdg_avg,
        speed2d: sp2d_avg,
        speed3d: sp3d_avg,
        // Use datetime for first point in cluster to represent the start
        // of the timestamp for averaged points. (rather than average datetime)
        datetime: points.first().and_then(|p| p.datetime),
        // timestamp: should be start of first point not average,
        // so that timestamp + duration = timespan within which all averaged points were logged
        timestamp: ts_first, // TODO test! hero11 then virb (remove set_timedelta for virb)
        // timestamp: Some(time_avg), // OLD
        // duration: should be sum of all durations
        // so that timestamp + duration = timespan within which all averaged points were logged
        duration: Some(dur_total), // TODO test! hero11 then virb (remove set_timedelta for virb)
        // duration: points.first().and_then(|p| p.duration), // OLD
        description,
    }
}

/// Calculate the great circle distance in kilometers between two points
/// on earth's surface (specified in decimal degrees)
pub fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let deg2rad = std::f64::consts::PI / 180.0; // inverse for radians to degress

    // convert decimal degrees to radians
    let (lon1, lat1, lon2, lat2) = (
        lon1 * deg2rad,
        lat1 * deg2rad,
        lon2 * deg2rad,
        lat2 * deg2rad,
    );

    // haversine formula
    let dlon = lon2 - lon1;
    let dlat = lat2 - lat1;

    // let a = sin((dlat)/2)^2 + cos(lat1) * cos(lat2) * sin(dlon/2)^2;
    let a = (dlat / 2.).sin().powi(2) + lat1.cos() * lat2.cos() * (dlon / 2.).sin().powi(2);
    let c = 2. * a.sqrt().asin();
    let r = 6371.; // Radius of earth in kilometers. Use 3956 for miles

    c * r
}
