use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct System {
    #[serde(default = "default_api_version")]
    api_version: u32,
    #[serde(default = "default_check_revisions")]
    check_revisions: bool,
    #[serde(default = "default_database_backend_plugin")]
    database_backend_plugin: Option<String>,
    #[serde(default = "default_database_version")]
    database_version: u32,
    #[serde(default = "default_dicom_aet")]
    dicom_aet: String,
    #[serde(default = "default_dicom_port")]
    dicom_port: u32,
    #[serde(default = "default_http_port")]
    http_port: u32,
    #[serde(default = "default_is_http_server_secure")]
    is_http_server_secure: bool,
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_plugin_enabled")]
    plugin_enabled: bool,
    #[serde(default = "default_storage_area_plugin")]
    storage_area_plugin: Option<String>,
    #[serde(default = "default_version")]
    version: String,
}

impl Default for System {
    fn default() -> Self {
        Self {
            api_version: 16,
            check_revisions: false,
            database_backend_plugin: None,
            database_version: 6,
            dicom_aet: "ORTHANC".to_string(),
            dicom_port: 4242,
            http_port: 8042,
            is_http_server_secure: true,
            name: "Orthanc Gateway".to_string(),
            plugin_enabled: true,
            storage_area_plugin: None,
            version: "1.10.1".to_string(),
        }
    }
}

fn default_api_version() -> u32 {
    System::default().api_version
}

fn default_check_revisions() -> bool {
    System::default().check_revisions
}

fn default_database_backend_plugin() -> Option<String> {
    System::default().database_backend_plugin
}

fn default_database_version() -> u32 {
    System::default().database_version
}

fn default_dicom_aet() -> String {
    System::default().dicom_aet  
}

fn default_dicom_port() -> u32 {
    System::default().dicom_port
}

fn default_http_port() -> u32 {
    System::default().http_port
}

fn default_is_http_server_secure() -> bool {
    System::default().is_http_server_secure
}

fn default_name() -> String {
    System::default().name
}

fn default_plugin_enabled() -> bool {
    System::default().plugin_enabled
}

fn default_storage_area_plugin() -> Option<String> {
    System::default().storage_area_plugin
}

fn default_version() -> String {
    System::default().version
}