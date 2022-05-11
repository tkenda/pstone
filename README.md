# PStone

It is a simple static web server that exposes Orthanc's Stone Web Viewer files from front folder, simulates some Orthanc's responses to hack the viewer and redirects the DICOM WADO requests to another endpoint. It can be used with any PACSs that supports DICOM WADO standard.

## Build

### Ubuntu (WIP)

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install NodeJS: https://nodejs.org
3. Execute build script: 
    ```
    $ sh build.sh
    ```
4. Use the .deb package from ``/target/release/pstone_0.1.0.deb``. 
    ```
    # dpkg -i pstone_0.1.0.deb
    ```
### Windows

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install NodeJS: https://nodejs.org
3. Install JRSoftware Inno Setup: https://jrsoftware.org/isinfo.php
4. Set the correct Inno Setup path value ``INNO_DIR`` in ``/build/.env`` file.
5. Execute build script: 
    ```
    C:\>build.bat
    ```
6. Use the installer program from ``/target/release/pstone_0.1.0.exe``.

## Usage

The service starts on boot. You can setup application using the config.yaml located in the same folder where the app runs.

Default Path:
- Windows = ``C:\Program Files\PStone``
- Ubuntu = ``/usr/local/pstone``

Setup ``config.yaml``:

```yaml
# IP address of the server.
bindIp: "0.0.0.0"
# Port of the server.
port: 3000
# PACS type. (Only available Proteus PACS)
pacs: proteus
# PACS WADO endpoint.
pacsUrl: "http://127.0.0.1:8080/api/v1/"
# DICOM study download option. (Only available for Proteus PACS)
archive:
    proteus:
        token: "XXXXXXX"
# Data sent to Stone Web Viewer to identify PACS.
system:
    apiVersion: 16
    checkRevisions: false
    databaseBackendPlugin: ~
    dicomAet: "ORTHANC"
    dicomPort: 4242
    httpPort: 8042
    isHttpServerSecure: true
    name: "Orthanc Gateway"
    pluginEnabled: true
    storageAreaPlugin: ~
    version: "1.10.1"
# TLS for server
tls:
    enabled: false
    certs: ""
    key: ""
```

## License

This code is licensed under the [<b>Affero General Public License</b>](https://raw.githubusercontent.com/tkenda/pstone/main/LICENSE) the same as [<b>Orthanc Web Viewer</b>](https://www.orthanc-server.com/static.php?page=stone-web-viewer)'s license.