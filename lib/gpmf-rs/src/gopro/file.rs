//! Structs for locating and working with MP4-files belonging to the same recording session.

use std::path::{Path, PathBuf};

use crate::{
    GpmfError,
    Gpmf,
};

use super::{
    RecordingType,
    GoProMeta
};

/// Represents an original, unedited GoPro MP4-file,
/// or a raw, binary file of GPMF-data extracted
/// from a GoPro Mp4-file with e.g. FFmpeg.
#[derive(Debug, Clone, PartialEq)]
pub struct GoProFile {
    /// High-resolution MP4 path
    pub mp4_path: PathBuf,
    /// Low-resolution MP4 path
    pub lrv_path: PathBuf,
    pub sequence: u8,
    pub recording_type: RecordingType,
    pub file_id: Option<String>,
    /// MP4 `udta` container.
    /// Contains custom fields for GoPro MP4-files,
    /// and GPMF-data.
    pub gpmf: Gpmf, // should just use Option<Gpmf>
    // pub meta: Option<Udta>, // single MP4 files only
    // pub muid: Option<>, 
    pub(crate) parsed: bool
}

impl GoProFile {
    pub fn new(path: &Path, parse: bool, _debug: bool) -> Result<Self, GpmfError> {
        let mut gopro = Self::default();
        
        gopro.mp4_path = path.to_owned();
        
        let (sequence, file_id, recording_type) = Self::parse_filename(path)?;
        
        gopro.sequence = sequence;
        gopro.recording_type = recording_type;
        gopro.file_id = file_id;
        
        if parse {
            gopro.gpmf = Gpmf::new(path)?;
            gopro.parsed = true;
        }

        Ok(gopro)
    }

    pub fn is_gopro(path: &Path) -> Result<bool, GpmfError> {
        let mut mp4 = mp4iter::Mp4::new(path)?;
        while let Ok(hdlr) = mp4.hdlr() {
            if &hdlr.component_name == "GoPro MET" {
                return Ok(true)
            }
        }
        Ok(false)
    }

    pub fn defult_with_path(path: &Path) -> Self {
        let mut gopro = Self::default();
        gopro.mp4_path = path.to_owned();
        gopro
    }

    /// Parses and returns order in session,
    /// file ID from file name (last four characters),
    /// and recording type (looping or chaptered).
    /// See <https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US>.
    pub fn parse_filename(path: &Path) -> Result<(u8, Option<String>, RecordingType), std::num::ParseIntError> {
        
        let ext = path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());
        
        // File stem as Vec<char>
        let filestem: Vec<char> = match ext.as_deref() {
            Some("mp4") => {
                path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.chars().collect::<Vec<_>>())
                .unwrap_or_default()
            },
            _ => Vec::new(),
        };

        let (mut sequence, mut file_id, mut recording_type) = (u8::default(), None, RecordingType::Unknown);

        if filestem.len() == 8 {
            match filestem[2..4].iter().collect::<String>().parse::<u8>() {
                // Chaptered video:
                // GH010026 -> 01 = chapter, 0026 = id
                Ok(num) => {
                    sequence = num;
                    file_id = Some(filestem[4..8].iter().collect::<String>()); // e.g. "1234"
                    recording_type = RecordingType::Chaptered;
                },
                // Looping video
                // GHAA0001 -> AA = id, 0001 = loop nr
                Err(_) => {
                    sequence = filestem[4..8].iter().collect::<String>().parse::<u8>()?;
                    file_id = Some(filestem[2..4].iter().collect::<String>()); // e.g. "AA"
                    recording_type = RecordingType::Looping;
                }
            }
        }

        Ok((sequence, file_id, recording_type))
    }

    // pub fn meta(&self) -> Result<(), GpmfError> {
    //     let mut mp4 = mp4iter::Mp4::new(&self.mp4_path)?;
    //     let udta = mp4.udta()?;

    //     Ok(())
    // }

    /// Extract custom data in MP4 `udta` container.
    /// GoPro stores some device settings and info here,
    /// including a mostly undocumented GPMF-stream.
    pub fn meta(&self) -> Result<GoProMeta, GpmfError> {
        GoProMeta::new(&self.mp4_path)
    }

    // /// Print custom GoPro metadata in MP4 `udta` atom.
    // pub fn print_meta(&self) {
    //     if let Some(udta) = &self.meta {
    //             udta.fields.iter().for_each(|f| println!("{:?}", f));
    //             udta.streams.iter().for_each(|s| s.print(None, None))
    //     }
    // }

    // /// Returns unique media ID/`MUID` as vector of eight unsigned integers.
    // /// Uses the raw `udta` field in GoPro MP4 `udta` atom.
    // /// Call `muid_gpmf()` for the GPMF-stream embedded within the `udta`atom instead.
    // /// Note that the last four values in the vector are not equal for
    // /// the "raw" `udta` field and the equivalent GPMF-stream.
    // // pub fn muid_udta(&self) -> Option<Vec<u32>> {
    // pub fn muid_udta(&self) -> Option<Muid> {
    //     let basetype = self.meta.as_ref()
    //         .and_then(|meta| meta.find_udta(&FourCC::MUID))
    //         .and_then(|field| field.to_basetype(b'L', &Endian::Little).ok())?;
        
    //     let vec: Option<Vec<u32>> = basetype.as_ref().into();

    //     vec.map(|v| Muid::Udta(v))
    // }

    // /// Returns unique media ID/`MUID` as vector of eight unsigned integers.
    // /// Uses GPMF-stream in GoPro MP4 `udta` atom.
    // /// Call `muid_udta()` for the raw `udta` field instead.
    // /// Note that the last four values in the vector are not equal for
    // /// the "raw" `udta` field and the equivalent GPMF-stream.
    // // pub fn muid_gpmf(&self) -> Option<Vec<u32>> {
    // pub fn muid_gpmf(&self) -> Option<Muid> {
    //     let muid_values = self.meta.as_ref()
    //         .map(|udta| udta.find_gpmf(&FourCC::MUID, true))
    //         .and_then(|streams| streams.get(0).cloned())
    //         .and_then(|stream| stream.values());
        
    //     muid_values.map(|basetypes| {
    //         let vec: Vec<u32> = basetypes.iter()
    //             .filter_map(|basetype| {
    //                 let num: Option<u32> = basetype.into();
    //                 num
    //             })
    //             .collect();

    //         vec
    //     })
    //     .map(|v| Muid::Gpmf(v))
    // }
    
    // pub fn muid_bytes(&self) -> Option<Muid> {
    //     let basetype = self.meta.as_ref()
    //         .and_then(|meta| meta.find_udta(&FourCC::MUID))
    //         .and_then(|field| field.to_basetype(b'B', &Endian::Little).ok())?;
    
    //     let vec: Option<Vec<u8>> = basetype.as_ref().into();

    //     vec.map(|v| Muid::Bytes(v))
    // }
}

impl Default for GoProFile {
    fn default() -> Self {
        Self {
            mp4_path: PathBuf::default(),
            lrv_path: PathBuf::default(),
            sequence: u8::default(),
            recording_type: RecordingType::Unknown,
            file_id: None,
            // meta: None,
            gpmf: Gpmf::default(),
            parsed: false
        }
    }
}