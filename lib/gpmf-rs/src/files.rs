use std::{path::Path, ffi::OsStr};

pub(crate) fn match_extension(path: &Path, ext: &str) -> bool {
    if let Some(path_ext) = path.extension() {
        return path_ext.to_ascii_lowercase() == OsStr::new(&ext).to_ascii_lowercase()
    }
    false
}
