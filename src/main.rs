use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    http::{uri::Uri, Request, Response},
    response::{self, IntoResponse, Redirect},
    routing::{any, get, get_service, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use hyper::{client::HttpConnector, Body};
use std::fs::File;
use std::{convert::TryFrom, net::SocketAddr};
use tower_http::services::ServeDir;

pub type Client = hyper::client::Client<HttpConnector, Body>;

mod config;
mod integration;
mod system;

use config::Config;

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Load config.yaml, if not found use default values.
    let config = match File::open("config.yaml") {
        Ok(file) => serde_yaml::from_reader(file).unwrap(),
        Err(err) => {
            println!("{}", err);
            Config::default()
        }
    };

    let addr = SocketAddr::new(config.bind_ip, config.port);

    let rustls = if config.tls.enabled {
        Some(
            RustlsConfig::from_pem_file(&config.tls.certs, &config.tls.key)
                .await
                .unwrap(),
        )
    } else {
        None
    };

    let app = Router::new()
        .route("/system", get(system_handler))
        .route("/tools/lookup", post(lookup_handler))
        .route("/studies/:id/archive", get(archive_handler))
        .nest("/dicom-web/", any(wado_handler))
        .fallback(get_service(ServeDir::new("./front")).handle_error(handle_error))
        .layer(Extension(client))
        .layer(Extension(config));

    println!("reverse proxy listening on {}", addr);

    match rustls {
        Some(t) => {
            axum_server::bind_rustls(addr, t)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        None => {
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    }
}

// Simulates Orthancs PACS response.
async fn system_handler(Extension(config): Extension<Config>) -> Json<system::System> {
    Json(config.system)
}

// Reverse proxy to custom /tools/lookup endpoint.
async fn lookup_handler(
    Extension(config): Extension<Config>,
    Extension(client): Extension<Client>,
    mut request: Request<Body>,
) -> Response<Body> {
    let uri = format!("{}orthanc/tools/lookup", config.pacs_url);
    *request.uri_mut() = Uri::try_from(uri).unwrap();
    client.request(request).await.unwrap()
}

enum ArchiveRes {
    Redirect(String),
    Error(String),
    None,
}

impl IntoResponse for ArchiveRes {
    fn into_response(self) -> response::Response {
        match self {
            ArchiveRes::Redirect(ref t) => Redirect::temporary(t).into_response(),
            ArchiveRes::Error(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
                .into_response(),
            ArchiveRes::None => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Archive not configured!"),
            )
                .into_response(),
        }
    }
}

// Reverse proxy to archive endpoint.
async fn archive_handler(
    Extension(config): Extension<Config>,
    Path(id): Path<String>,
) -> ArchiveRes {
    match config.archive.get_link(&id).await {
        Some(Ok(t)) => ArchiveRes::Redirect(t),
        Some(Err(err)) => ArchiveRes::Error(err),
        None => ArchiveRes::None,
    }
}

// Reverse proxy to PACS wado endpoint.
async fn wado_handler(
    Extension(config): Extension<Config>,
    Extension(client): Extension<Client>,
    mut request: Request<Body>,
) -> Response<Body> {
    let path = request.uri().path();
    let path_query = request
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("{}dicom-web{}", config.pacs_url, path_query);
    (*request.headers_mut()).append("X-WADO-Client", "orthanc".parse().unwrap());
    *request.uri_mut() = Uri::try_from(uri).unwrap();
    client.request(request).await.unwrap()
}

async fn handle_error(error: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}
