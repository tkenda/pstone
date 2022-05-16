use axum::{
    body::Body,
    extract::{Extension, Path},
    http::{HeaderMap, Method, Request},
    http::{HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{self, IntoResponse, Redirect},
    routing::{any, get, get_service, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use reqwest::{header, Client};
use std::fs::File;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod config;
mod integration;
mod system;

use config::Config;

#[tokio::main]
async fn main() {
    // Load config.yaml, if not found use default values.
    let config = match File::open("config.yaml") {
        Ok(file) => serde_yaml::from_reader(file).unwrap(),
        Err(err) => {
            println!("{}", err);
            Config::default()
        }
    };

    let client = Client::builder()
        .danger_accept_invalid_certs(config.pacs_invalid_certs)
        .referer(false)
        .build()
        .unwrap();

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

    let server_name = config.server_name.to_owned();

    let app = Router::new()
        .route("/system", get(system_handler))
        .route("/tools/lookup", post(lookup_handler))
        .route("/studies/:id/archive", get(archive_handler))
        .nest("/dicom-web/", any(wado_handler))
        .fallback(get_service(ServeDir::new("./front")).handle_error(handle_error))
        .layer(Extension(client))
        .layer(Extension(config))
        .layer(middleware::from_fn(move |req, next| {
            filter_redirects(server_name.clone(), req, next)
        }));

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

async fn send(
    url: String,
    request: Request<Body>,
    client: Client,
) -> (StatusCode, HeaderMap, Vec<u8>) {
    let action = match request.method() {
        &Method::GET => client.get(url),
        &Method::HEAD => client.head(url),
        &Method::POST => client.post(url),
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::default(),
                format!("Invalid request method.").as_bytes().to_vec(),
            )
        }
    };

    let headers = request.headers().to_owned();

    match action
        .header("X-WADO-Client", "orthanc")
        .header(
            header::REFERER,
            headers
                .get(header::REFERER)
                .unwrap_or(&HeaderValue::from_static("")),
        )
        .header(
            header::ACCEPT,
            headers
                .get(header::ACCEPT)
                .unwrap_or(&HeaderValue::from_static("")),
        )
        .body(request.into_body())
        .send()
        .await
    {
        Ok(t) => (
            StatusCode::OK,
            t.headers().to_owned(),
            t.bytes().await.unwrap().to_vec(),
        ),
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::default(),
                format!("{}", err).as_bytes().to_vec(),
            )
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
    request: Request<Body>,
) -> impl IntoResponse {
    let url = format!("{}orthanc/tools/lookup", config.pacs_url);
    send(url, request, client).await
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
    request: Request<Body>,
) -> impl IntoResponse {
    let path_and_query = request
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(request.uri().path());

    let url = format!("{}dicom-web{}", config.pacs_url, path_and_query);
    send(url, request, client).await
}

async fn handle_error(error: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", error),
    )
}

fn host_from_header<B>(req: &Request<B>) -> Option<&str> {
    let host_parts = req
        .headers()
        .get(header::HOST)
        .map(|t| t.to_str().unwrap().split(":").collect::<Vec<&str>>())
        .unwrap_or(vec![]);

    if host_parts.is_empty() {
        return None;
    }

    let uri_parts = host_parts[0].split("/").collect::<Vec<&str>>();

    if uri_parts.is_empty() {
        None
    } else {
        Some(uri_parts[0])
    }
}

fn host_from_uri<B>(req: &Request<B>) -> Option<&str> {
    req.uri().host()
}

async fn filter_redirects<B>(
    server_name: Option<String>,
    req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    match server_name {
        Some(ref sn) => {
            let host = match host_from_uri(&req) {
                Some(t) => Some(t),
                None => host_from_header(&req),
            };

            match host {
                Some(t) if t == sn => Ok(next.run(req).await),
                Some(t) if t == "127.0.0.1" => Ok(next.run(req).await),
                Some(t) if t == "localhost" => Ok(next.run(req).await),
                Some(_) | None => Err(StatusCode::MISDIRECTED_REQUEST),
            }
        }
        None => Ok(next.run(req).await),
    }
}
