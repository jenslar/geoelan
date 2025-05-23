//! File/path related functions, including filtering data on recording session (Garmin VIRB).

use std::ffi::OsString;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub mod gopro;
pub mod virb;

/// Used for any confirmation, e.g. overwrite file.
pub fn confirm(message: &str) -> std::io::Result<bool> {
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

/// Check if `path` has file extension `ext`.
pub fn has_extension(path: &Path, ext: &str) -> bool {
    let inpathext = path.extension().map(|o| o.to_ascii_lowercase());
    let matchext = OsString::from(&ext.to_lowercase());
    inpathext == Some(matchext)
}

pub fn has_extension_any(path: &Path, exts: &[&str]) -> bool {
    exts.iter().any(|ext| has_extension(path, ext))
}

/// Write file with user confirmation if path exists.
pub fn writefile(content: &[u8], path: &Path) -> std::io::Result<bool> {
    let write = if path.exists() {
        confirm(&format!("{} already exists. Overwrite?", path.display()))?
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
pub fn affix_file_name(
    path: &Path,
    prefix: Option<&str>,
    suffix: Option<&str>,
    extension: Option<&str>,
    substitute_space: Option<&str>
) -> PathBuf {
    let prefix = prefix.unwrap_or("");
    let suffix = suffix.unwrap_or("");

    let new_path = match path.file_stem().and_then(|s| s.to_str()) {
        // Some(stem) => path.with_file_name(file_name),
        Some(stem) => {
            let mut file_name = format!("{prefix}{stem}{suffix}");
            if let Some(c) = substitute_space {
                file_name = file_name.replace(" ", c);
            }
            path.with_file_name(file_name)
        },
        None => path.to_owned(),
    };

    // Set specified extension, or ensure return path has the same file ext as inpath
    if let Some(ext) = extension {
        return new_path.with_extension(ext);
    } else if let Some(ext) = path.extension() {
        return new_path.with_extension(ext);
    }

    new_path
}

pub fn filename_startswith(path: &Path, pattern: &str) -> bool {
    path.file_name()
        .and_then(|f| f.to_str())
        .map(|s| s.starts_with(pattern))
        .unwrap_or(false)
}

pub fn paths(dir: &Path, ext: &[&str]) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|result| {
            if let Ok(entry) = result {
                let p = entry.path();
                // let e = p.extension().and_then(|e| e.to_ascii_lowercase().to_str());
                if let Some(e) = p
                    .extension()
                    .map(|e| e.to_string_lossy().to_ascii_lowercase())
                {
                    if ext.contains(&e.as_str()) {
                        Some(p.to_owned())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

pub enum Units {
    Bytes(u64),
    Kilo(f64),
    Mega(f64),
    Giga(f64),
    Tera(f64),
}

impl From<u64> for Units {
    fn from(value: u64) -> Self {
        match value as f64 {
            _z @ ..1e3 => Self::Bytes(value),
            z @ 1e3..1e6 => Self::Kilo(z / 1e3),
            z @ 1e6..1e9 => Self::Mega(z / 1e6),
            z @ 1e9..1e12 => Self::Giga(z / 1e9),
            z @ 1e12..1e15 => Self::Tera(z / 1e12),
            z => Self::Tera(z / 1e12),
        }
    }
}

impl std::fmt::Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Units::Bytes(n) => write!(f, "{n} bytes"),
            Units::Kilo(fl) => write!(f, "{fl:.2}KB", ),
            Units::Mega(fl) => write!(f, "{fl:.2}MB", ),
            Units::Giga(fl) => write!(f, "{fl:.2}GB", ),
            Units::Tera(fl) => write!(f, "{fl:.2}TB", ),
        }
    }
}