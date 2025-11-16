# Appimage Installer  
This is a simple appimage installer written in rust. It extracts appimages, copies them to the opt directory, constructs a desktop entry for them and moves the icon to the system.

## Usage
To install an appimage run this command:
```bash
sudo appimage_installer app.AppImage
```

## Installation  

### From Github Releases

1. Download the latest release: `wget https://github.com/haileyydev/appimage-installer/releases/latest/download/appimage-installer`  
2. Make it executable: `sudo chmod +x appimage-installer`
3. Install it system wide: `sudo cp appimage-installer /usr/local/bin`

### Building From Source

1. Download the latest release: `git clone https://github.com/haileyydev/appimage-installer.git`  
2. Enter the directory: `cd appimage-installer`  
3. Compile the project: `cargo build --release`
4. Install it system wide: `sudo cp target/release/appimage-installer /usr/local/bin`