use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::ArchiveTraits;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArchiveProteus {
    #[serde(default = "default_sign_url")]
    pub sign_url: String,
    #[serde(default = "default_public_url")]
    pub public_url: String,
    #[serde(default = "default_duration")]
    pub duration: u32,
    pub token: String,
}

impl Default for ArchiveProteus {
    fn default() -> Self {
        Self {
            sign_url: "http://127.0.0.1:8080/api/v1/dicom/zip/%ID%/duration/%DURATION%/link"
                .to_string(),
            public_url: "http://127.0.0.1:8080".to_string(),
            token: String::default(),
            duration: 300,
        }
    }
}

fn default_sign_url() -> String {
    ArchiveProteus::default().sign_url
}

fn default_public_url() -> String {
    ArchiveProteus::default().public_url
}

fn default_duration() -> u32 {
    ArchiveProteus::default().duration
}

#[derive(Debug, Deserialize)]
struct Link {
    link: String,
    status: String,
}

#[async_trait]
impl ArchiveTraits for ArchiveProteus {
    async fn execute(&self, id: &str) -> Result<String, String> {
        match Client::builder()
            .danger_accept_invalid_certs(true)
            .referer(false)
            .build()
            .unwrap()
            .get(
                self.sign_url
                    .replace("%ID%", id)
                    .replace("%DURATION%", &format!("{}", self.duration)),
            )
            .bearer_auth(&self.token)
            .send()
            .await
        {
            Ok(res) if res.status().is_success() => match res.json::<Link>().await {
                Ok(t) if t.status == "done" => Ok(format!("{}{}", self.public_url, t.link)),
                Ok(t) => Err(format!("Invalid status response from Proteus. {:?}", t)),
                Err(err) => Err(format!("Could not parse JSON response. {}", err)),
            },
            Ok(_) => Err("Invalid HTTP status response from Proteus.".to_string()),
            Err(err) => Err(format!("{}", err)),
        }
    }
}
