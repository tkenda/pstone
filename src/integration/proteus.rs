use async_trait::async_trait;
use hyper::client::HttpConnector;
use hyper::{body, Body, Method, Request};
use serde::{Deserialize, Serialize};

use super::ArchiveTraits;

pub type Client = hyper::client::Client<HttpConnector, Body>;

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
            sign_url: "http://127.0.0.1:8080/api/v1/dicom/zip/%ID%/duration/%DURATION%/link".to_string(),
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
        let client = Client::new();

        let req = Request::builder()
            .method(Method::GET)
            .uri(
                self.sign_url
                    .replace("%ID%", id)
                    .replace("%DURATION%", &format!("{}", self.duration)),
            )
            .header("authorization", format!("Bearer {}", self.token))
            .body(Body::empty())
            .unwrap();

        let res = match client.request(req).await {
            Ok(t) => t,
            Err(err) => return Err(err.to_string()),
        };

        if res.status().is_success() {
            match body::to_bytes(res).await {
                Ok(body) => match serde_json::from_slice::<Link>(&body) {
                    Ok(t) if t.status == "done" => Ok(format!("{}{}", self.public_url, t.link)),
                    Ok(_) => Err("Invalid status response from Proteus".to_string()),
                    Err(err) => Err(format!("Could not parse JSON response. {}", err)),
                },
                Err(err) => Err(format!("Invalid body response from Proteus. {}", err)),
            }
        } else {
            Err("Invalid HTTP status response from Proteus.".to_string())
        }
    }
}
