use gpmf_rs::{GoProSession, DeviceName};

use crate::geo::EafPointCluster;

use super::cam2eaf;

/// Generate EAF from GoPro recording session.
pub fn run(args: &clap::ArgMatches, gopro_session: &GoProSession) -> std::io::Result<()> {
    let time_offset = args.get_one::<isize>("time-offset").unwrap().to_owned(); // clap: has default value
    let fullgps = *args.get_one::<bool>("fullgps").unwrap();
    let gpsfix = *args.get_one::<u32>("gpsfix").unwrap(); // defaults to 3 (3D lock)
    let geotier = *args.get_one::<bool>("geotier").unwrap();

    // Get the GPS-data and convert to geo::point::Point:s.
    let mut pointcluster: Option<EafPointCluster> = None;
    if geotier {
        print!("Merging GPMF-data for {} files...", gopro_session.len());
        let gpmf = match gopro_session.gpmf() {
            Ok(g) => g,
            Err(err) => {
                let msg = format!("(!) Failed to merge GPMF data: {err}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
            }
        };
        println!(" Done");
        print!("Extracting GPS data (minimum satellite lock = {}) with time offset {} hours... ",
            gpsfix,
            time_offset
        );

        let downsample_factor = if matches!(gopro_session.device(), Some(&DeviceName::Hero11Black)) && !fullgps { 
            // Downsample GPS9 (10Hz) depending on setting
            10
        } else {
            1
        };

        // Extract points, prune those below satellite lock threshold. Defaults to 3D lock.
        // let mut gps = gpmf.gps(false).prune(gpsfix, None);
        let gps = gpmf.gps().prune(gpsfix, None);
        let end = match gpmf.duration() {
            Ok(d) => d,
            Err(err) => {
                let msg = format!("(!) Failed to determine duration for session: {err}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
            }
        };

        pointcluster = Some(
            if downsample_factor > 1 {
                EafPointCluster::from_gopro(&gps.0, None, &end, Some(time_offset as i64))
                    .downsample(downsample_factor, None)
            } else {
                EafPointCluster::from_gopro(&gps.0, None, &end, Some(time_offset as i64))
            }
        );

        println!("OK");
    }

    let session_hi = gopro_session.mp4();
    let session_lo = gopro_session.lrv();

    // Concatenate clips and generate eaf
    cam2eaf::run(
        &session_hi,
        &session_lo,
        pointcluster.map(|pc| pc.points).as_deref(),
        None,
        None,
        args
    )
}