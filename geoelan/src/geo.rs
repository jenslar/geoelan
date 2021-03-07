#![allow(dead_code)]
use fit_rs::structs::{GpsMetadata, Point};

// FIT conversions
// input values + processing according to FIT SDK Profile.xlsx
// pub fn degrees(semicircle: i32) -> f64 {
//     (semicircle as f64) * (180.0 / 2.0_f64.powi(31))
// }

// pub fn altitude(alt: u32) -> f64 {
//     (alt as f64 / 5.0) - 500.0 // scale 5, offset 500
// }

// pub fn heading(hdg: u16) -> f32 {
//     hdg as f32 / 100.0 // scale 100
// }

// pub fn velocity(vel: &[i16]) -> f32 {
//     // NOTE: returns scalar/vector sum value for now
//     if vel.len() != 3 {
//         panic!("Invalid Velocity length. Expected 3, got {}", vel.len());
//     }
//     let vel_x = (vel[0] as i32)
//         .checked_pow(2)
//         .unwrap_or_else(|| panic!("Buffer overflow for velocity X: {}", vel[0]));
//     let vel_y = (vel[1] as i32)
//         .checked_pow(2)
//         .unwrap_or_else(|| panic!("Buffer overflow for velocity Y: {}", vel[1]));
//     let vel_z = (vel[2] as i32)
//         .checked_pow(2)
//         .unwrap_or_else(|| panic!("Buffer overflow for velocity Z: {}", vel[2]));
//     (vel_x as f32 + vel_y as f32 + vel_z as f32).sqrt() / 100.0
// }

// pub fn speed(spe: u32) -> f32 {
//     spe as f32 / 1000.0
// }

pub fn downsample(mut sample_factor: usize, gps: &[GpsMetadata]) -> Vec<Point> {
    let initial_sample_factor = sample_factor; // changed if remaining points < sample_factor
    let mut points: Vec<Point> = Vec::new();

    for idx in (0..gps.len()).step_by(sample_factor) {
        // check against initial value, but use dynamic one for steps
        match sample_factor {
            1 => points.push(gps[idx].to_point()),
            _ => {
                let gps2points: Vec<_> = gps[idx..idx + sample_factor]
                    .iter()
                    .map(|p| p.to_point().to_owned())
                    .collect();
                points.push(point_cluster_average(&gps2points, None))
            }
        };

        // need to check step size before last loop, i.e. before last idx+stepsize
        // or risk out of bounds if gps.len() % initial_sample_factor != 0, hence 2*samplefactor
        if initial_sample_factor > 1 && 2 * sample_factor > gps.len() - idx {
            sample_factor = gps.len() - idx - sample_factor;
        }
    }

    points
}

pub fn point_cluster_average(points: &[Point], text: Option<&String>) -> Point {
    // see: https://carto.com/blog/center-of-points/
    // atan2(y,x) where y = sum((sin(yi)+...+sin(yn))/n), x = sum((cos(xi)+...cos(xn))/n), y, i in radians
    // note that this currently does a f64 conversion/cast from degrees to radians and back to degrees

    let deg2rad = std::f64::consts::PI / 180.0; // inverse for radians to degress

    let mut lon_rad_sin: Vec<f64> = Vec::new(); // sin values
    let mut lon_rad_cos: Vec<f64> = Vec::new(); // cos values
    let mut lat_rad: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut alt: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut spe: Vec<f64> = Vec::new(); // arithmetic average ok
                                        // let mut vel: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut hdg: Vec<f64> = Vec::new(); // arithmetic average ok
    let mut time_as_ms: Vec<i64> = Vec::new();

    for pt in points.iter() {
        lon_rad_sin.push((pt.longitude * deg2rad).sin()); // get the sin values immediately
        lon_rad_cos.push((pt.longitude * deg2rad).cos()); // get the cos values immediately
        lat_rad.push(pt.latitude * deg2rad); // arithmetic avg ok, only converts to radians
        alt.push(pt.altitude);
        spe.push(pt.speed);
        // vel.push(pt.velocity as f64);
        hdg.push(pt.heading);
        time_as_ms.push(pt.time.num_milliseconds());
    }

    // AVERAGING LATITUDE DEPENDANT LONGITUDES
    let lon_rad_sin_sum: f64 = lon_rad_sin.into_iter().sum::<f64>() / points.len() as f64;
    let lon_rad_cos_sum: f64 = lon_rad_cos.into_iter().sum::<f64>() / points.len() as f64;
    let lon_avg_deg = f64::atan2(lon_rad_sin_sum, lon_rad_cos_sum) / deg2rad; // degrees
    let lat_avg_deg = lat_rad.into_iter().sum::<f64>() / points.len() as f64 / deg2rad; // degrees
    let alt_avg = alt.into_iter().sum::<f64>() / points.len() as f64;
    let spe_avg = spe.into_iter().sum::<f64>() / points.len() as f64;
    // let vel_avg = vel.into_iter().sum::<f64>() / points.len() as f64;
    let hdg_avg = hdg.into_iter().sum::<f64>() / points.len() as f64;
    let time_avg = chrono::Duration::milliseconds(
        (time_as_ms.into_iter().sum::<i64>() as f64 / points.len() as f64) as i64,
    );

    Point {
        latitude: lat_avg_deg,
        longitude: lon_avg_deg,
        altitude: alt_avg,
        speed: spe_avg,
        // velocity: vel_avg,
        heading: hdg_avg,
        time: time_avg,
        text: text.map(String::from),
    }
}
