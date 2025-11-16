use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};

// extracts an appimage
pub fn extract_appimage(path: &Path) -> Result<PathBuf, String> {
    println!("Extracting appimage: {:?}", &path);

    // check if the appimage exists
    if !path.exists() {
        return Err("Appimage dosent exist".to_string());
    }

    // make the path absolute
    let path_absolute = fs::canonicalize(path).map_err(|e| e.to_string())?;

    // contruct the command
    let mut extract_cmd: Command = Command::new(path_absolute);
    extract_cmd.arg("--appimage-extract");

    // execute the command and get the output
    let output = extract_cmd.output();

    match output {
        // if the command errors
        Err(e) => {
            return Err(e.to_string());
        }

        // if the command dosent error
        Ok(_) => {
            let out_path = PathBuf::from("squashfs-root");

            // check if the output directory exists
            if out_path.exists() {
                println!("Squashfs saved to squashfs-root");
                return Ok(out_path);
            } else {
                return Err("Failed to extract appimage".to_string());
            }
        }
    }
}