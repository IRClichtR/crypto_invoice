use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;
use serde::Serialize;
use std::{fs, path::Path, sync::Arc};

use crate::{
    AppState,
    app_error::app_error::AppError,
};

#[derive(Serialize)]
struct FrontendConfig {
    csrf_token: String,
}

#[axum::debug_handler]
pub async fn serve_home(
    State(app_state): State<Arc<AppState>>,
    csrf_token: CsrfToken,
) -> Result<impl IntoResponse, AppError> {
    let index_path = format!("{}/index.html", app_state.vue_dist_path);

    match fs::read_to_string(Path::new(&index_path)) {
        Ok(mut html_content) => {
            // Minimal frontend config
            let frontend_config = FrontendConfig {
                csrf_token: match csrf_token.authenticity_token() {
                    Ok(token) => token,
                    Err(_) => return Err(AppError::ServerError("Failed to retrieve CSRF token".to_string())),
                },
            };
            
            let config_json = serde_json::to_string(&frontend_config)
                .map_err(|e| AppError::ServerError(format!(
                    "Failed to serialize CSRF token: {}", e
                )))?;
            
            // Insert configuration into HTML
            html_content = html_content.replace(
                "<!-- BACKEND_CONFIG -->", 
                &format!("<script>window.CSRF_TOKEN = '{}';</script>", 
                    match csrf_token.authenticity_token() {
                        Ok(token) => token,
                        Err(_) => return Err(AppError::ServerError("Failed to retrieve CSRF token".to_string())),
                    })
            );
            
            // Header configuration
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
            
            headers.insert(
                header::X_CONTENT_TYPE_OPTIONS,
                header::HeaderValue::from_static("nosniff"),
            );
            
            Ok((StatusCode::OK, headers, Html(html_content)))
        },
        Err(e) => {
            // Transformation de l'erreur en AppError
            Err(AppError::ServerError(format!(
                "Failed to read index.html: {}", e
            )))
        }
    }
}

