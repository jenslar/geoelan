use std::{path::Path, ffi::OsStr};

pub(crate) fn has_extension(path: &Path, ext: &str) -> bool {
    if let Some(path_ext) = path.extension() {
        return path_ext.to_ascii_lowercase() == OsStr::new(&ext).to_ascii_lowercase()
    }
    false
}

pub (crate) fn extension_as_str(path: &Path) -> Option<&str> {
    path.extension().and_then(|s| s.to_str())
}