use std::io::ErrorKind;

use fit_rs::VirbSession;

use crate::geo::point_cluster::EafPointCluster;

use super::cam2eaf;

/// Generate EAF from VIRB recording session.
pub fn run(args: &clap::ArgMatches, virb_session: &mut VirbSession) -> std::io::Result<()> {
    // Options
    let time_offset: isize = *args.get_one("time-offset").unwrap(); // default: 0
    let mut downsample_factor = match *args.get_one::<bool>("fullgps").unwrap() {
        true => 1,
        false => 10,
    };

    // Parse linked FIT and set start/end time stamps.
    virb_session.process(time_offset as i64)?;

    let mut gpsfail = false;
    let geotier = *args.get_one::<bool>("geotier").unwrap();

    // EXTRACT GPS, DERIVE TIME DATA FROM FIT
    let mut pointcluster: Option<EafPointCluster> = None;
    if let Ok(gps) = virb_session.gps() {
        if gps.is_empty() {
            println!("(!) No logged points for UUID in FIT-file.");
            gpsfail = true;
        } else {
            let (t0, end) = match (virb_session.t0, virb_session.end) {
                (Some(t), Some(e)) => (t, e),
                _ => {
                    let msg = "(!) Failed to determine time values for session.";
                    return Err(std::io::Error::new(ErrorKind::Other, msg));
                }
            };

            // prevent no points in output
            if downsample_factor >= gps.len() {
                downsample_factor = gps.len()
            }

            let mut cluster =
                EafPointCluster::from_virb(&gps, None, &t0, &end, Some(time_offset as i64))
                    .downsample(downsample_factor, None);
            // .offset_hrs(time_offset as i64);

            // Correct point "duration" (time difference between two logged points)
            // to ensure correct annotation duration in EAF.
            // TODO don't call this with new average behaviour that sets timespan/duration
            // TODO differently
            cluster.set_timedelta(Some(&t0), &end);

            pointcluster = Some(cluster);
        }
    } else {
        println!("(!) Failed to extract GPS data.");
        gpsfail = true;
    }

    if geotier && gpsfail {
        eprintln!("(!) No geotier will be created.")
    }

    let session_start_ms = virb_session.start.map(|n| n.whole_milliseconds() as i64);

    let session_hi = virb_session.mp4();
    let session_lo = virb_session.glv();

    // Concatenate clips and generate eaf
    cam2eaf::run(
        &session_hi,
        &session_lo,
        pointcluster.map(|pc| pc.points).as_deref(),
        session_start_ms,
        Some(virb_session.fit_path().as_path()),
        args,
    )
}
