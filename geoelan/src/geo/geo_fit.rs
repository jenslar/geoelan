//! GPS related functions for Garmin VIRB.

use fit_rs::{Fit, FitError};
use super::Point;

/// Sets the datetime field in `Point` structs generated from Garmin VIRB FIT data.
/// 
/// GoPro GPMF already generates datetime.
/// 
/// `timestamp`/`253` will be used for e.g. watches that store absolute datetime here
/// and not via a blanket value in `timestamp_correlation` (VIRB).
pub fn set_datetime_fit(points: &mut [Point], fit: &Fit, offset: i64) -> Result<(), FitError> {
    // let t0 = fit.t0(offset, true).unwrap().naive_utc();
    let t0 = fit.t0(offset, true).unwrap();
    
    for point in points.iter_mut() {
        // .timestamp is available for both FIT + GoPro MP4,
        // but is not present in raw GPMF streams. GeoELAN,
        // should only handle GoPro MP4 for EAF generation.
        if let Some(ts) = point.timestamp {
            // increment datetime by current video position.
            point.datetime = Some(t0 + ts);
        } else {
            // fallback that does not include relative video position.
            point.datetime = Some(t0);
        }
    }

    Ok(())
}
