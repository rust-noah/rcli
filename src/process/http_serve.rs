use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use std::{fs, io, net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };

    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(subpath): Path<String>,
) -> (StatusCode, Html<String>) {
    let file_path = state.path.join(&subpath);

    info!("Reading file {:?}", file_path);
    if !file_path.exists() {
        (
            StatusCode::NOT_FOUND,
            Html(format!(
                "<html><body><p>File {} not found.</p></body></html>",
                file_path.display()
            )),
        )
    } else if file_path.is_dir() {
        let mut entries = fs::read_dir(file_path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .and_then(|iter| {
                iter.map(|entry| entry.map(|e| e.file_name().into_string().unwrap_or_default()))
                    .collect::<Result<Vec<_>, io::Error>>()
            })
            .unwrap_or_else(|_| vec![]);

        entries.sort();
        let list_items = entries
            .iter()
            .map(|entry| format!("<li><a href=\"{0}\">{0}</a></li>", entry))
            .collect::<Vec<_>>()
            .join("");

        return (
            StatusCode::OK,
            Html(format!("<html><body><ul>{}</ul></body></html>", list_items)),
        );
    } else {
        match tokio::fs::read_to_string(file_path).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, Html(content))
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!(
                        "<html><body><p>Error reading file: {:?}</p></body></html>",
                        e
                    )),
                )
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, _) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
    }
}
