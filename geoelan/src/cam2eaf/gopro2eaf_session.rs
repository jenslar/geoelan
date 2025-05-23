use gpmf_rs::{DeviceName, GoProSession};

use crate::geo::EafPointCluster;

use super::cam2eaf;

/// Generate EAF from GoPro recording session.
pub fn run(args: &clap::ArgMatches, gopro_session: &GoProSession) -> std::io::Result<()> {
    let time_offset = args.get_one::<isize>("time-offset").unwrap().to_owned(); // clap: has default value
    let fullgps = *args.get_one::<bool>("fullgps").unwrap();
    // let gpsfix = *args.get_one::<u32>("gpsfix").unwrap(); // defaults to 2 (2D lock)
    let gpsfix = *args.get_one::<u32>("gpsfix").expect("Error: GPS fix should default to 3."); // defaults to 3 (3D lock)
    let gpsdop = args.get_one::<f64>("gpsdop");
    let geotier = *args.get_one::<bool>("geotier").unwrap();

    // Get the GPS-data and convert to geo::point::Point:s.
    let mut pointcluster: Option<EafPointCluster> = None;
    if geotier {
        print!("Merging GPMF-data for {} files...", gopro_session.len());
        let gpmf = match gopro_session.gpmf() {
            Ok(g) => g,
            Err(err) => {
                let msg = format!("(!) Failed to merge GPMF data: {err}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
            }
        };
        println!(" Done");
        print!(
            "Extracting GPS data with time offset {} hours... ",
            time_offset
        );

        let downsample_factor =
            // !!! device needs to check for Hero 13 Black as well
            if matches!(gopro_session.device(), Some(&DeviceName::Hero11Black)) && !fullgps {
                // Downsample GPS9 (10Hz) depending on setting
                10
            } else {
                1
            };

        // Extract points, prune those below satellite lock threshold. Defaults to 3D lock.
        let gps = gpmf.gps().prune(Some(gpsfix), gpsdop.copied());
        println!("{} points extracted with filters minimum satellite lock = {}, max DOP = {}",
            gps.len(),
            gpsfix,
            gpsdop.map(|n| n.to_string()).unwrap_or("N/A".to_owned()),
        );
        if gps.is_empty() {
            println!("Try lowering filters for satellite lock level, e.g. '--gpsfix 2' (2D),\nand/or raising dilution of precision, e.g. '--gpsdop 10'");
        }
        let end = match gpmf.duration() {
            Ok(d) => d,
            Err(err) => {
                let msg = format!("(!) Failed to determine duration for session: {err}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, msg));
            }
        };

        pointcluster = Some(if downsample_factor > 1 {
            EafPointCluster::from_gopro(gps.points(), None, &end, Some(time_offset as i64))
                .downsample(downsample_factor, None)
        } else {
            EafPointCluster::from_gopro(gps.points(), None, &end, Some(time_offset as i64))
        });

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
        args,
    )
}
