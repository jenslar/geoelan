use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::*;
use std::path::{Path, PathBuf};

/// Calculate hash, return sha256 hash+bytes read
pub fn checksum(path: &Path) -> std::io::Result<(String, u64)> {
    let mut file = File::open(path)?;
    let hash: Vec<u8>;
    let size: u64;

    let mut hasher = Sha256::new();
    size = copy(&mut file, &mut hasher)?;
    hash = hasher.finalize().to_vec();
    let hex_string = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");

    Ok((hex_string, size))
}

/// Used for any acknowledgement, e.g. overwrite file
pub fn acknowledge(message: &str) -> bool {
    loop {
        print!("(!) {} (y/n): ", message);
        stdout().flush().unwrap();
        let mut overwrite = String::new();
        stdin()
            .read_line(&mut overwrite)
            .expect("Unable to read input");

        return match overwrite.to_lowercase().trim() {
            "y" | "yes" => true,
            "n" | "no" => false,
            _ => {
                println!("Enter y/yes or n/no");
                continue;
            }
        };
    }
}

/// File io etc
pub fn writefile(content: &[u8], path: &PathBuf) -> std::io::Result<()> {
    let write = if path.exists() {
        acknowledge(&format!("{} already exists. Overwrite?", path.display()))
    } else {
        true
    };

    if write {
        let mut outfile = File::create(&path)?;
        outfile.write_all(content)?;
        println!("Wrote {}", path.display());
    } else {
        println!("Aborted writing {}", path.display());
    }

    Ok(())
}
