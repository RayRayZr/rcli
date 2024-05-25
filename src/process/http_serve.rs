use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

struct HTTPServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("http server started at http://localhost:{}", port);
    let state = HTTPServeState { path };
    let router = Router::new()
        .route("/*path", get(handle_index))
        .with_state(Arc::new(state));
    let add = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(add).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn handle_index(
    State(state): State<Arc<HTTPServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        return (StatusCode::NOT_FOUND, format!("{} not found", p.display()));
    }
    let content = match tokio::fs::read_to_string(p).await {
        Ok(content) => content,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("read file error: {}", e),
            )
        }
    };
    (StatusCode::OK, content)
}
