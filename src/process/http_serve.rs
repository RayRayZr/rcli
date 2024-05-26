use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;

struct HTTPServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    info!("http server started at http://localhost:{}", port);
    let state = HTTPServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_gzip()
        .precompressed_zstd()
        .precompressed_deflate();

    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(handle_index))
        .with_state(Arc::new(state));
    let add = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(add).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

enum Response {
    Text(String),
    Html(Html<String>),
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        match self {
            Response::Text(s) => s.into_response(),
            Response::Html(s) => s.into_response(),
        }
    }
}

async fn handle_index(
    State(state): State<Arc<HTTPServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, Response) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            Response::Text(format!("{} not found", p.display())),
        );
    }
    if p.is_dir() {
        let mut dirs = match tokio::fs::read_dir(p).await {
            Ok(dirs) => dirs,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Response::Text(format!("read dir error: {}", e)),
                )
            }
        };
        let mut arr = Vec::new();
        while let Ok(Some(entry)) = dirs.next_entry().await {
            let path = entry.path();
            arr.push(format!(
                "<li><a href=\"{}\">{}</a></li>",
                path.to_string_lossy().to_string().split_off(1),
                path.to_string_lossy()
            ));
        }
        let body = arr.join("\n");
        return (StatusCode::OK, Response::Html(Html(body)));
    }
    let content = match tokio::fs::read_to_string(p).await {
        Ok(content) => content,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::Text(format!("read file error: {}", e)),
            )
        }
    };
    (StatusCode::OK, Response::Text(content))
}
