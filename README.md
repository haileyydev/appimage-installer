# Appimage Installer  
This is a command line utility written in rust for installing appimages as system wide applications for seamless integration with desktop environments.

## Installation  

You can install appimage installer by either downloading the precompiled binary from github releases or building it from source yourself.

### From Github Releases  

Follow these instructions to download and install the precompiled binary.

1. Download the latest release: `wget https://github.com/haileyydev/appimage-installer/releases/latest/download/appimage-installer`  
2. Make it executable: `sudo chmod +x appimage-installer`
3. Install it system wide: `sudo cp appimage-installer /usr/local/bin`

### Building From Source

Follow this guide to build the app from source

#### Prerequisites
- git
- cargo

#### Instructions

1. Download the latest release: `git clone https://github.com/haileyydev/appimage-installer.git`  
2. Enter the directory: `cd appimage-installer`  
3. Compile the project: `cargo build --release`
4. Install it system wide: `sudo cp target/release/appimage-installer /usr/local/bin`

## Usage
Use this command to install an appimage:
```bash
sudo appimage_installer app.AppImage
```