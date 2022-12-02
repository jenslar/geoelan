use std::collections::HashSet;
use std::io::Cursor;
use std::path::{PathBuf, Path};

use jpegiter::{Jpeg, JpegTag};
use rayon::prelude::{IntoParallelRefMutIterator, IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::{FourCC, Timestamp, Stream};
use crate::{
    Gps,
    GoProPoint,
    ContentType,
    GpmfError,
    files::match_extension,
    gopro::Dvid
};

/// Core GPMF struct.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Gpmf {
    /// GPMF streams.
    pub streams: Vec<Stream>,
    /// Path/s to the GoPro MP4 source/s
    /// the GPMF data was extracted from.
    pub source: Vec<PathBuf>
}

impl Gpmf {
    /// GPMF from file. Either an unedited GoPro MP4-file,
    /// JPEG-file (WIP, currently n/a),
    /// or a "raw" GPMF-file, extracted via FFmpeg.
    /// Relative timestamps for all data loads is exclusive
    /// to MP4, since these are derived from MP4 timing.
    /// 
    /// ```
    /// use gpmf_rs::Gpmf;
    /// use std::path::Path;
    /// fn main() -> std::io::Result<()> {
    ///     let path = Path::new("PATH/TO/GOPRO.MP4");
    ///     let gpmf = Gpmf::new(&path)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(path: &Path) -> Result<Self, GpmfError> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| GpmfError::InvalidFileType {path: path.to_owned()})?;

        match ext.as_ref() {
            "mp4" => Self::from_mp4(path),
            // TODO GPMF embedded as EXIF
            // "jpg" => (),
            // Possibly "raw" GPMF-file
            _ => Self::from_file(path)
        }
    }

    /// Returns the embedded GPMF streams in a GoPro MP4 file.
    pub fn from_mp4(path: &Path) -> Result<Self, GpmfError> {
        let mut mp4 = mp4iter::Mp4::new(path)?;

        // TODO 220812 REGRESSION CHECK: DONE.
        // TODO        Mp4::offsets() 2-3x slower with new code (4GB file), though in microsecs 110-200us old vs 240-600us new.
        // 1. Extract position/byte offset, size, and time span for GPMF chunks.
        let offsets = mp4.offsets("GoPro MET")?;
        
        // Faster than a single, serial iter so far.
        // 2. Read data at MP4 offsets and generate timestamps serially
        let mut timestamps: Vec<Timestamp> = Vec::new();
        let mut cursors = offsets.iter()
            .map(|o| {
                // Create timestamp
                let timestamp = timestamps.last()
                    .map(|t| Timestamp {
                        relative: t.relative + o.duration,
                        duration: o.duration,
                    }).unwrap_or(Timestamp {
                        relative: 0,
                        duration: o.duration
                    });
                timestamps.push(timestamp);

                // Read and return data at MP4 offsets
                mp4.read_at(o.position as u64, o.size as u64)
                    .map_err(|e| GpmfError::Mp4Error(e))
            })
            .collect::<Result<Vec<_>, GpmfError>>()?;

        assert_eq!(timestamps.len(), cursors.len(), "Timestamps and cursors differ in length for GPMF");

        // 3. Parse each data chunk/cursor into Vec<Stream>.
        let streams = cursors.par_iter_mut().zip(timestamps.par_iter())
            .map(|(cursor, t)| {
                let stream = Stream::new(cursor, None)
                    .map(|mut strm| {
                        // 1-2 streams. 1 for e.g. Hero lineup, 2 for Karma drone (1 for drone, 1 for attached cam)
                        strm.iter_mut().for_each(|s| s.set_time(t));
                        strm
                    });
                stream
            })
            .collect::<Result<Vec<_>, GpmfError>>()? // Vec<Vec<Stream>>, need to flatten
            .par_iter()
            .flatten_iter() // flatten will mix drone data with cam data, perhaps bad idea
            .cloned()
            .collect::<Vec<_>>();

        Ok(Self{
            streams,
            source: vec![path.to_owned()]
        })
    }

    /// Returns the embedded GPMF stream in a GoPro photo, JPEG only.
    pub fn from_jpg(path: &Path) -> std::io::Result<()> {
        // Find and extract EXIf chunkg with GPMF, then .from_cursor()
        // println!("reading as JPEG");
        let mut jpeg = Jpeg::new(path)?;
        let exif = jpeg.exif()?;
        println!("{exif:?}");
        // println!("iterating over JPEG");
        // while let Ok(segment) = jpeg.next() {
        //     match segment.tag {
        //         JpegTag::APP1 => println!("{:?}", segment),
        //         JpegTag::SOS => {
        //             println!("START OF SCAN, BREAKING LOOP");
        //             break
        //         },
        //        _ => ()
        //     }
        // }

        Ok(())
    }

    /// Returns GPMF from a "raw" GPMF-file,
    /// e.g. the "GoPro MET" track extracted from a GoPro MP4 with FFMpeg.
    pub fn from_file(path: &Path) -> Result<Self, GpmfError> {
        // TODO do a buffered read instead of arbitrary max size value?
        let max_size = 50_000_000_u64; // max in-memory size set to 50MB
        let size = path.metadata()?.len();

        if size > max_size {
            return Err(GpmfError::MaxFileSizeExceeded{
                max: max_size,
                got: size,
                path: path.to_owned()
            })
        }

        let mut cursor = Cursor::new(std::fs::read(path)?);
        let streams = Stream::new(&mut cursor, None)?;

        Ok(Self{
            streams,
            source: vec![path.to_owned()]
        })
    }

    /// GPMF from byte slice.
    pub fn from_slice(slice: &[u8]) -> Result<Self, GpmfError> {
        let mut cursor = Cursor::new(slice.to_owned());
        Self::from_cursor(&mut cursor)
    }

    /// GPMF from `Cursor<Vec<u8>>`.
    pub fn from_cursor(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, GpmfError> {
        Ok(Self{
            // streams: Stream::new(cursor)?,
            streams: Stream::new(cursor, None)?,
            source: vec![]
        })
    }

    pub fn print(&self) {
        self.iter().enumerate()
            .for_each(|(i, s)|
                s.print(Some(i+1), Some(self.len()))
            )
    }

    /// Returns number of `Streams`.
    pub fn len(&self) -> usize {
        self.streams.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Stream> {
        self.streams.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Stream> {
        self.streams.iter_mut()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = Stream> {
        self.streams.into_iter()
    }

    /// Returns first DEVC stream
    pub fn first(&self) -> Option<&Stream> {
        self.streams.first()
    }

    /// Returns last DEVC stream
    pub fn last(&self) -> Option<&Stream> {
        self.streams.last()
    }

    /// Find streams with specified FourCC.
    pub fn find(&self, fourcc: &FourCC, recurse: bool) {
        unimplemented!("TODO: IMPLEMENT OPTIONALLY RECIRSIVE FOURCC SEARCH")
        // self.iter()
        //     .map(|stream| {
        //         match stream.streams {
        //             StreamType::Nested(s) => {

        //             }
        //             StreamType::Values(v)
        //         }
        //     })
    }

    /// Move multiple `Stream`s from `streams` to `self.streams`.
    pub fn append(&mut self, streams: &mut Vec<Stream>) {
        self.streams.append(streams)
    }

    /// Add multiple `Stream`s to `self.streams`.
    pub fn extend(&mut self, streams: &[Stream]) {
        self.streams.extend(streams.to_owned())
    }

    /// Merges two GPMF streams, returning the merged stream,
    /// leaving `self` untouched.
    /// Assumed that specified `gpmf` follows after
    /// `self` chronologically.
    pub fn merge(&self, gpmf: &Self) -> Self {
        let mut merged = self.to_owned();
        merged.merge_mut(&mut gpmf.to_owned());
        merged
    }

    /// Merges two GPMF streams in place.
    /// Assumed that specified `gpmf` follows after
    /// `self` chronologically.
    pub fn merge_mut(&mut self, gpmf: &mut Self) {
        if let Some(ts) = self.last_timestamp() {
            // adds final timestamp of previous gpmf to all timestamps
            gpmf.add_time(&ts);
        }

        // Use append() instead?
        // https://github.com/rust-lang/rust-clippy/issues/4321#issuecomment-929110184
        self.extend(&gpmf.streams);
        self.source.extend(gpmf.source.to_owned());
    }

    /// Filters direct child nodes based on `StreamType`. Not recursive.
    pub fn filter(&self, content_type: &ContentType) -> Vec<Stream> {
        self.iter()
            .flat_map(|s| s.filter(content_type))
            .collect()
    }

    /// Filters direct child nodes based on `StreamType` and returns an iterator. Not recursive.
    pub fn filter_iter<'a>(
        &'a self,
        content_type: &'a ContentType,
    ) -> impl Iterator<Item = Stream> + 'a {
        self.iter()
            .flat_map(move |s| s.filter(content_type))
    }

    /// Returns all unique free text stream descriptions, i.e. `STNM` data.
    /// The hierarchy is `DEVC` -> `STRM` -> `STNM`.
    pub fn names(&self) -> Vec<String> {
        // TODO perhaps use HashSet instead?
        let mut names: Vec<String> = Vec::new();
        for (i1, devc_stream) in self.streams.iter().enumerate() {
            names.extend(
            devc_stream.find_all(&FourCC::STRM)
                .iter()
                .enumerate()
                // .inspect(|(i2, s)| println!("{i1}|{i2} {:?} {} {:?}", s.fourcc(), s.len(), s.name()))
                .filter_map(|(_, s)| {
                    let n = s.name();
                    // println!("{n:?}");
                    n
                })
                // .map(|(_, s)| s.name())
                .collect::<Vec<_>>()
            )
        }

        names.sort();
        names.dedup();

        names
    }

    /// Returns summed duration of MP4 sources (longest track).
    /// Raises error if sources are not MP4-files
    /// (e.g. if source is a raw `.gpmf` extracted via FFmpeg).
    pub fn duration(&self) -> Result<time::Duration, GpmfError> {
        // self.source.iter()
        //     .map(|path| mp4iter::Mp4::new(path)?.duration())
        //     .fold(time::Duration::ZERO, |acc| mp4iter::Mp4::new(path)?.duration())
        // TODO perhaps use fold()?
        let mut duration = time::Duration::ZERO;
        for path in self.source.iter() {
            duration += mp4iter::Mp4::new(path)?.duration()?;
        }
        Ok(duration)
    }

    pub fn duration_ms(&self) -> Result<i64, GpmfError> {
        Ok((self.duration()?.as_seconds_f64() * 1000.0) as i64)
    }

    /// Add time to all `DEVC` timestamps
    pub fn add_time(&mut self, time: &Timestamp) {
        self.iter_mut()
            .for_each(|devc|
                devc.time = devc.time.to_owned().map(|t| t.add(time))
            )
    }

    /// Returns first `Timestamp` in GPMF stream.
    pub fn first_timestamp(&self) -> Option<&Timestamp> {
        self.first()
        .and_then(|devc| devc.time.as_ref())
    }
    
    /// Returns last `Timestamp` in GPMF stream.
    pub fn last_timestamp(&self) -> Option<&Timestamp> {
        self.last()
            .and_then(|devc| devc.time.as_ref())
    }

    // /// Device name. Extracted from first `Stream`.
    // pub fn device_name_old(&self) -> Option<String> {
    //     self.streams
    //         .first()
    //         .and_then(|s| s.device_name())
    // }

    /// Device name. Extracted from first `Stream`.
    pub fn device_name(&self) -> Vec<String> {
        let names_set: HashSet<String> = self.streams.iter()
            .filter_map(|s| s.device_name())
            .collect();
        
        let mut names = Vec::from_iter(names_set);
        names.sort();

        names
    }

    /// Device id. Extracted from first `Stream`.
    pub fn device_id(&self) -> Option<Dvid> {
        self.streams
            .first()
            .and_then(|s| s.device_id())
    }

    /// Returns all GPS streams as Vec<Point>`. Each returned point is a processed,
    /// linear average of `GPS5` (should be accurate enough for the 18Hz GPS,
    /// but implementing a latitude dependent longitude average is a future possibility).
    pub fn gps(&self) -> Gps {
        Gps(self.filter_iter(&ContentType::Gps)
            .flat_map(|s| GoProPoint::new(&s)) // TODO which Point to use?
            .collect::<Vec<_>>())
    }

    // /// Extract custom data in MP4 `udta` container.
    // /// GoPro stores some device settings and info here,
    // /// including a mostly undocumented GPMF-stream.
    // pub fn meta(&self) -> Result<(), GpmfError> {
    //     if self.source.iter().any(|p| !match_extension(p, "mp4")) {
    //         return Err(GpmfError::InvalidFileType{expected_ext: String::from("mp4")})
    //     }
    //     for path in self.source.iter() {
    //         let gpmeta = GoProMeta::new(path)?;
    //     }

    //     Ok(())
    // }

    // /// Derive starting time, i.e. the absolute timestamp for first `DEVC`.
    // /// Can only be determined if the GPS was turned on and logged data.
    // /// 
    // /// Convenience method that simply subtracts first `Point`'s `Point.time.instant` from `Point.datetime`.
    // /// 
    // /// Note that this will filter on Gps streams again,
    // /// so if you already have a `Gps` struct use `Gps::t0()`,
    // /// or do the calucation yourself from `Vec<Point>`.
    // pub fn t0(&self) -> Option<NaiveDateTime> {
    //     self.gps().t0()
    // }
}
