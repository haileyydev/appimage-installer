use crate::utils::extractor::extract_appimage;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use fs_extra::dir;

// parse the contents of a .desktop file
fn parse_desktop_file(content: String) -> HashMap<String, String> {
    // create a hashmap to store them
    let mut desktop_values = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            desktop_values.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    return desktop_values;
}

// install an appimage
pub fn install_appimage(path: &Path) -> Result<String, String> {
    // extract the appimage
    match extract_appimage(path) {
        // if it fails to extract
        Err(e) => {
            return Err(e.to_string());
        }
        // once its extracted
        Ok(squashfs) => {
            // read the directory contents
            // if the read errors, then return the error
            println!("Reading squashfs");

            let squashfs_contents = fs::read_dir(&squashfs).map_err(|e| e.to_string())?;

            let mut desktop_file: Option<PathBuf> = None;

            // look for the desktop file
            for entry_result in squashfs_contents {
                // if it fails to read the entry, return an error
                let entry = entry_result.map_err(|e| e.to_string())?;

                let path = entry.path();
                
                // check if the child is a desktop file
                if path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    desktop_file = Some(path);
                    break;
                }
            }

            // error if the desktop file isnt found
            let desktop_path = match desktop_file {
                Some(path) => path,
                None => {
                    return Err("Desktop file not found".to_string());
                }
            };

            println!("Reading desktop file");

            // read the desktop file and error if it cant
            let desktop_contents = fs::read_to_string(&desktop_path).map_err(|e| e.to_string())?;

            // parse the desktop file
            let parsed_desktop_file = parse_desktop_file(desktop_contents);

            if let Some(name) = parsed_desktop_file.get("Name") {
                println!("Installing {}", name);
                let destination = PathBuf::from(format!("/opt/{}", name));

                println!("Copying {:?} to {:?}", squashfs, destination);

                // create the options for copying
                let mut options = dir::CopyOptions::new();
                options.overwrite = true;
                options.copy_inside = true;

                // copy the squashfs and return an error if it fails
                dir::copy(&squashfs, &destination, &options).map_err(|e| e.to_string())?;

                println!("Removing original squashfs");

                // delete the original squashfs and return an error if it fails
                dir::remove(squashfs).map_err(|e| e.to_string())?;

                println!("Building desktop entry");

                let mut desktop_entry = String::new();
                desktop_entry.push_str("[Desktop Entry]\n");

                // for every entry in the original desktop file copy it to the new one
                for (key, value) in parsed_desktop_file.into_iter() {
                    match key.as_str() {
                        // if its the Exec option we need to change the path
                        "Exec" => {
                            desktop_entry.push_str(format!("Exec={}/{}\n", &destination.display(), value).as_str());
                        }
                        _ => {
                            desktop_entry.push_str(format!("{}={}\n", key, value).as_str());
                        }
                    }
                }
                println!("Copying app icon to system");

                // move the icon to the system icon directory
                let icondirectory = PathBuf::from(format!("{}/usr/share/icons", &destination.display()));
                dir::copy(icondirectory, "/usr/share", &options).map_err(|e| e.to_string())?;
                
                return Ok("Application installed successfully".to_string());

            } else {
                return Err("Application name not found".to_string());
            }
        }
    }
}