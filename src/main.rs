use crate::utils::installer::install_appimage;
use std::path::Path;
use std::env;

mod utils;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No appimage passed");
        std::process::exit(1);
    }

    let appimage = &args[1];

    let output = install_appimage(Path::new(appimage));
    match output {
        Err(e) => {
            eprintln!("Error while installing appimage: {:?}", e);
        }
        Ok(output) => {
            println!("{}", output.as_str());
        }
    }
}
