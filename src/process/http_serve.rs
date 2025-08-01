use std::{path::PathBuf, sync::Arc};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serveing {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };

    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else if p.is_dir() {
        let mut content = String::new();
        content.push_str("<html><body><ul>");
        let mut ss = Vec::new();
        match list_files_in_dir(&p, &mut ss).await {
            Err(e) => {
                eprint!("{e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, "读取文件错误".into());
            }
            Ok(_) => {
                for s in &ss {
                    content.push_str(s);
                }
            }
        }
        content.push_str("</ul></body></html>");
        (StatusCode::OK, content)
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

async fn list_files_in_dir(path: &PathBuf, file_path: &mut Vec<String>) -> anyhow::Result<()> {
    let mut entries = tokio::fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            Box::pin(list_files_in_dir(&path, file_path)).await?;
        } else if let Some(name) = path.file_name() {
            let s = format!("<li><a href= \"{}\">{:?}</a></li>", path.display(), name);
            file_path.push(s);
        }
    }
    Ok(())
}
