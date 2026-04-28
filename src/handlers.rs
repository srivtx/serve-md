use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Redirect},
};
use std::sync::Arc;

use crate::{
    markdown::render_markdown,
    resolve::{resolve, ResolveResult},
    server::AppState,
    template::{directory_listing, wrap_html},
};

pub async fn handle_root(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    handle_path(State(state), "/".to_string()).await
}

pub async fn handle_request(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    handle_path(State(state), format!("/{}", path)).await
}

async fn handle_path(State(state): State<Arc<AppState>>, url_path: String) -> impl IntoResponse {
    match resolve(&state.index, &url_path) {
        ResolveResult::Markdown(fs_path) => {
            if let Ok(meta) = tokio::fs::metadata(&fs_path).await {
                if meta.len() > 10 * 1024 * 1024 {
                    return (StatusCode::FORBIDDEN, "Markdown file too large (max 10MB)")
                        .into_response();
                }
            }
            match tokio::fs::read_to_string(&fs_path).await {
                Ok(content) => {
                    let rendered = render_markdown(&content);
                    let html = wrap_html(&rendered, &url_path);
                    Html(html).into_response()
                }
                Err(e) => {
                    let name = fs_path
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_else(|| "unknown".into());
                    tracing::error!("Failed to read file {}: {}", name, e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        ResolveResult::Static(fs_path) => {
            if let Ok(meta) = tokio::fs::metadata(&fs_path).await {
                if meta.len() > 100 * 1024 * 1024 {
                    return (StatusCode::FORBIDDEN, "File too large (max 100MB)").into_response();
                }
            }
            match tokio::fs::read(&fs_path).await {
                Ok(bytes) => {
                    let mime = mime_guess::from_path(&fs_path).first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], bytes).into_response()
                }
                Err(e) => {
                    let name = fs_path
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_else(|| "unknown".into());
                    tracing::error!("Failed to read file {}: {}", name, e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        ResolveResult::Redirect(to) => Redirect::permanent(&to).into_response(),
        ResolveResult::Directory(dir_path) => {
            if let Some(entries) = state.index.dirs.get(&dir_path) {
                let html = directory_listing(&dir_path, entries);
                Html(html).into_response()
            } else {
                StatusCode::NOT_FOUND.into_response()
            }
        }
        ResolveResult::NotFound => StatusCode::NOT_FOUND.into_response(),
    }
}
