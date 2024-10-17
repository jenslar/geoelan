//! Functions for generating ELAN-files and selecting tiers.

use eaf_rs::{eaf::{Eaf, Tier}, EafError};
use mp4iter::Mp4;
use std::{io::Write, path::Path};

use crate::text::process_string;

use super::geo::point::EafPoint;

/// Generates an ELAN-file. If points are provided,
/// a tier named "geo" will be created with these inserted as annotations.
///
/// If points are provided, `end` can be used to set maximum time in milliseconds
/// for the final annotation boundary, in case this surpasses the length
/// of the media files.
///
/// VIRB only: `session_start_ms` and `session_end_ms` allows for shifting the ELAN timeline,
/// since relative timestamps in FIT are relative to the start of the FIT-file,
/// which is usually earlier than recording start.
pub fn generate_eaf(
    video_path: &Path, // could do mp4iter::mp4::Mp4::duration from this to get end
    audio_path: &Path,
    points: Option<&[EafPoint]>,
    session_start_ms: Option<i64>,
) -> Result<Eaf, EafError> {
    let mut eaf = if let Some(pts) = points {
        // Generate tier with coordinates is points are passed
        let geo_tier_id = "geo";

        // Annotations in the form (value, start_ms, end_ms)
        let mut annotations: Vec<(String, i64, i64)> = Vec::new();

        // for point in pts.iter() {
        for (i, point) in pts.iter().enumerate() {
            let t = point
                .timestamp
                .to_owned()
                .expect("(!) No relative timestamp for point");
            let mut ts_val1 = t.whole_milliseconds() as i64; // i128 -> i64: i64::MAX = ca 1100hrs so should be ok for video
                                                             // VIRB only (?): FIT-start time is relative to FIT-file start, not session start
                                                             //                Need to substract from each EAF time slot.
            if let Some(start) = session_start_ms {
                ts_val1 -= start;
            }

            let ts_val2 = ts_val1
                + point
                    .duration
                    .expect("no duration for point") // error or pass default string?
                    .whole_milliseconds() as i64; // i128 -> i64 = ca 1100hrs so should be ok for video

            // Set annotation value
            let timestamp = point
                .datetime
                .expect("no datetime for point") // err or default string?
                // .format("%Y-%m-%dT%H:%M:%S%.3f")
                .to_string(); // TODO 200809 check string representation for PrimitiveDateTime
            let annotation_value = format!(
                "LAT:{:.6};LON:{:.6};ALT:{:.1};TIME:{}",
                point.latitude, point.longitude, point.altitude, timestamp
            );

            annotations.push((annotation_value, ts_val1, ts_val2));
        }

        // Set final time slot to "end" if it exceeds media length.
        // NOTE depending on final value of "end" final time slot may
        // get the same value as the next to final one using the
        // expression below.
        if let Some(annot_tuple) = annotations.last_mut() {
            let mut mp4 = Mp4::new(video_path)?;
            // Mp4::duration() returns error for zero length videos
            if let Ok(duration) = mp4.duration(false) {
                let duration_ms = duration.whole_milliseconds() as i64; // i128 as i64 cast should be safe enough for video time spans
                annot_tuple.2 = duration_ms;
            }
        }

        Eaf::from_values(&annotations, Some(geo_tier_id))?
    } else {
        // Generate an empty default eaf if no points passed.
        Eaf::default()
    };

    // Link media files
    eaf.with_media_mut(&[video_path.to_owned(), audio_path.to_owned()]);

    // index + derive not really necessary, since this is only for serializing into xml,
    // no further processing is done
    eaf.index();
    eaf.derive()?;

    Ok(eaf)
}

pub fn select_tier(eaf: &Eaf, no_tokenized: bool) -> std::io::Result<Tier> {
    println!("Select tier:");
    println!("      ID{}Parent              Tokenized  Annotations  Tokens unique/total  Participant     Annotator       Start of first annotation", " ".repeat(19));
    for (i, tier) in eaf.tiers.iter().enumerate() {
        println!(
            "  {:2}. {:21}{:21}{:5}      {:>9}     {:>6} / {:<6}    {:15} {:15} {}",
            i + 1,
            process_string(&tier.tier_id, None, None, None, Some(20)),
            process_string(
                tier.parent_ref.as_deref().unwrap_or("None"),
                None,
                None,
                None,
                Some(20)
            ),
            tier.is_tokenized(),
            tier.len(),
            tier.tokens(None, None, true, true).len(),
            tier.tokens(None, None, false, false).len(),
            process_string(
                tier.participant.as_deref().unwrap_or("None"),
                None,
                None,
                None,
                Some(15)
            ),
            process_string(
                tier.annotator.as_deref().unwrap_or("None"),
                None,
                None,
                None,
                Some(15)
            ),
            tier.annotations
                .first()
                .map(|a| {
                    format!(
                        "'{} ...'",
                        process_string(&a.value().to_string(), None, None, None, Some(30))
                    )
                })
                .unwrap_or("[empty]".to_owned())
        );
    }

    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        match buffer.trim_end().parse::<usize>() {
            Ok(i) => {
                match eaf.tiers.get(i - 1) {
                    // check if selected tier or any parent tier is tokenized
                    Some(t) => {
                        if eaf.is_tokenized(&t.tier_id, true)? && no_tokenized {
                            println!(
                                "(!) '{}' or one of its parents is tokenized. ['ctrl + c' to exit]",
                                t.tier_id
                            );
                        } else {
                            return Ok(t.to_owned());
                        }
                    }
                    None => println!("(!) No such tier. ['ctrl + c' to exit]"),
                }
            }
            Err(_) => println!("(!) Not a number. ['ctrl + c' to exit]"),
        }
    }
}
