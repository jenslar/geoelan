//! File/path related functions, including filtering data on recording session (Garmin VIRB).

use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

pub mod virb;
pub mod gopro;

/// Used for any acknowledgement, e.g. overwrite file.
pub fn acknowledge(message: &str) -> std::io::Result<bool> {
    loop {
        print!("(!) {} (y/n): ", message);
        stdout().flush()?;
        let mut overwrite = String::new();
        let _ = stdin().read_line(&mut overwrite)?;

        return match overwrite.to_lowercase().trim() {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => {
                println!("Enter y/yes or n/no");
                continue;
            }
        };
    }
}

/// Check if unix path is "hidden", i.e. starts with `.`.
/// If `check_components` = `true` the path components
/// (i.e. directories) are also checked.
pub fn is_hidden(path: &Path, check_components: bool) -> bool {
    if check_components {
        return path.iter()
            .any(|component|
                component.to_string_lossy().starts_with(".")
            )
        // for component in path.iter() {
        //     if component.to_string_lossy().starts_with(".") {
        //         return true
        //     }
        // }
    }
    path.file_name().map(|s| s.to_string_lossy().starts_with(".")) == Some(true)
}

/// Check if `path` has file extension `ext`.
pub fn has_extension(path: &Path, ext: &str) -> bool {
    let inpathext = path.extension().map(|o| o.to_ascii_lowercase());
    let matchext = OsString::from(&ext.to_lowercase());
    inpathext == Some(matchext)
}

/// Write file with user confirmation if path exists.
pub fn writefile(content: &[u8], path: &Path) -> std::io::Result<bool> {
    let write = if path.exists() {
        acknowledge(&format!("{} already exists. Overwrite?", path.display()))?
    } else {
        true
    };

    if write {
        let mut outfile = File::create(&path)?;
        outfile.write_all(content)?;
    }

    Ok(write)
}

/// Adds pre/suffix, to existing file stem or changes extension of path and returns the new path.
/// Returns path untouched if no file stem can be extracted.
// !!! TODO change to return option in order to avoid overwriting existing files
pub fn affix_file_name(path: &Path, prefix: Option<&str>, suffix: Option<&str>, extension: Option<&str>) -> PathBuf {
    let prefix = prefix.unwrap_or("");
    let suffix = suffix.unwrap_or("");

    let new_path = match path.file_stem().and_then(|s| s.to_str()) {
        Some(stem) => path.with_file_name(format!("{prefix}{stem}{suffix}")),
        None => path.to_owned()
    };

    // Set specified extension, or ensure return path has the same file ext as inpath
    if let Some(ext) = extension {
        return new_path.with_extension(ext)
    } else if let Some(ext) = path.extension() {
        return new_path.with_extension(ext)
    }
    
    new_path
}
