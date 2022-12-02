//! Geo-related functions, including generating KML + GeoJSON.

use time::Duration;

pub mod geo_fit;
pub mod geo_gpmf;
pub mod kml_gen;
pub mod json_gen;
pub mod geoshape;
pub mod kml_styles;
pub mod point;
pub mod point_cluster;

pub use geoshape::GeoShape;
pub use point::Point;
pub use point_cluster::PointCluster;

fn average(nums: &[f64]) -> f64 {
    nums.iter().sum::<f64>() / nums.len() as f64
}

/// Downsample points.
/// Clusters points in sizes equal to `sample_factor`
/// Optionally set a minimum number of points to return via `min`.
/// If `sample_factor` results in fewer points than `min`,
/// `min` will be used
pub fn downsample(
    mut sample_factor: usize,
    points: &[point::Point],
    min: Option<usize>
) -> Vec<point::Point> {
    match sample_factor {
        0 => panic!("Sample factor cannot be 0."),
        1 => return points.to_vec(),
        // ensure downsampling will at lest yield a single point
        f if f > points.len() => sample_factor = points.len(),
        _ => ()
    }

    // Int division for checking if downsample factor
    // causes fewer than optionally set min number of points
    if let Some(m) = min {
        // if points.len() / sample_factor < 2 { // shouldn't "< 2" be "< m"?
        if points.len() / sample_factor < m { // 220914 "< m" IS UNTESTED
            // div_ceil will be in upcoming rust version:
            // https://github.com/rust-lang/rfcs/issues/2844
            // sample_factor = points.len().div_ceil(m)
            // sample_factor = (points.len() as f64 / m as f64).ceil() as usize // should this be .floor()?
            // 220914 changed to .floor() since otherwise 
            sample_factor = (points.len() as f64 / m as f64).floor() as usize // .floor() IS UNTESTED
        }
    }

    let initial_sample_factor = sample_factor; // changed if remaining points < sample_factor

    let mut average: Vec<point::Point> = Vec::new();

    // TODO could perhaps iter over points using point.chunks(sample_factor) + remainder?
    for idx in (0..points.len()).step_by(sample_factor) {
        average.push(point_cluster_average(&points[idx..idx + sample_factor]));

        // Need to check step size before last loop, i.e. before last idx+stepsize
        // or risk out of bounds if points.len() % initial_sample_factor != 0,
        // hence 2*samplefactor.
        // Sets sample factor to len of remaining points if < initial samplefactor
        if initial_sample_factor > 1 && 2 * sample_factor > points.len() - idx {
            sample_factor = points.len() - idx - sample_factor;
        }
    }

    average
}

/// Returns latitude dependent average for specified coordinate cluster.
// pub fn point_cluster_average(points: &[Point], text: Option<&str>) -> Point {
pub fn point_cluster_average(points: &[point::Point]) -> point::Point {
    // see: https://carto.com/blog/center-of-points/
    // atan2(y,x) where y = sum((sin(yi)+...+sin(yn))/n), x = sum((cos(xi)+...cos(xn))/n), y, i in radians
    // note that this currently does a f64 conversion/cast from degrees to radians and back to degrees


    let description = points.first()
        .and_then(|p| p.description.to_owned());

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
                .unwrap_or(0)
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
        false => Some(average(&hdg))
    };
    let sp2d_avg = average(&sp2d);
    let sp3d_avg = average(&sp3d);
    let time_avg = Duration::milliseconds(
        time_as_ms.iter().sum::<i64>() / points.len() as i64, // may be off by 1ms since no float+round
    );

    point::Point {
        latitude: lat_avg_deg,
        longitude: lon_avg_deg,
        altitude: alt_avg,
        heading: hdg_avg,
        speed2d: sp2d_avg,
        speed3d: sp3d_avg,
        // TODO find datetime for avg point, not use that of the first
        datetime: points.first().and_then(|p| p.datetime),
        timestamp: Some(time_avg),
        // TODO find duration for avg point, not use that of the first
        duration: points.first().and_then(|p| p.duration),
        description,
    }
}
