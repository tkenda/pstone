# PStone

It is a simple static web server that exposes Orthanc's Stone Web Viewer files from front folder, simulates some Orthanc's responses to hack the viewer and redirects the DICOM WADO requests to another endpoint. It can be used with all PACS that supports DICOM WADO standard.

## Build

### Ubuntu (WIP)

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install NodeJS: https://nodejs.org
3. Execute build script: 
    ```
    $ sh build.sh
    ```
4. Use the .deb package from /target/release/pstone_0.1.0.deb. 
    ```
    # dpkg -i pstone_0.1.0.deb
    ```
### Windows

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install NodeJS: https://nodejs.org
3. Install JRSoftware Inno Setup: https://jrsoftware.org/isinfo.php
4. Set the correct Inno Setup path value (INNO_DIR) in /build/.env file.
5. Execute build script: 
    ```
    C:\>build.bat
    ```
6. Use the installer program from /target/release/pstone_0.1.0.exe.

## Usage

The service starts on boot. You can setup application configuration using the config.yaml located in the same folder where the app runs.

Default paths:
- Windows = ``C:\Program Files\PStone``
- Ubuntu = ``/usr/local/pstone``

## License

GNU AFFERO GENERAL PUBLIC LICENSE