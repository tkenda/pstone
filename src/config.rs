use serde::{Serialize, Deserialize};
use std::net::IpAddr;

use crate::integration::Archive;
use crate::system::System;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Pacs {
    Proteus
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Tls {
    pub enabled: bool,
    pub certs: String,
    pub key: String,
}

impl Default for Tls {
    fn default() -> Self {
        Self {
            enabled: false,
            certs: "".to_string(),
            key: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    #[serde(default = "default_bind_ip")]
    pub bind_ip: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_pacs")]
    pub pacs: Pacs,
    #[serde(default = "default_pacs_url")]
    pub pacs_url: String,
    #[serde(default)]
    pub archive: Archive,
    #[serde(default)]
    pub system: System,
    #[serde(default)]
    pub tls: Tls,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_ip: "0.0.0.0".parse().unwrap(),
            port: 3000,
            pacs: Pacs::Proteus,
            pacs_url: "http://127.0.0.1:8080/api/v1/".to_string(),
            archive: Archive::default(),
            system: System::default(),
            tls: Tls::default(),
        }
    }
}

fn default_bind_ip() -> IpAddr {
    Config::default().bind_ip
}

fn default_port() -> u16 {
    Config::default().port
}

fn default_pacs() -> Pacs {
    Config::default().pacs
}

fn default_pacs_url() -> String {
    Config::default().pacs_url
}

