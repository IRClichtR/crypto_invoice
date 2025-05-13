use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse}
};
use axum_csrf::CsrfToken;
use std::{fs, path::Path, sync::Arc};

use crate::{
    app_error::app_error::AppError, 
    config::app_config::get_serializable_frontend_config, 
    AppState
};

/// Serves the home page with injected frontend configuration
///
/// This function reads the index.html file, injects the backend configuration
/// (including the CSRF token) and returns the HTML page to the client.
#[axum::debug_handler]
pub async fn serve_home(
    State(app_state): State<Arc<AppState>>,
    csrf_token: CsrfToken,
) -> Result<impl IntoResponse, AppError> {
    // Build the complete path to the index.html file
    let index_path = format!("{}/index.html", app_state.vue_dist_path);
    eprintln!("Index path: {}", index_path);
    
    // Read the HTML file content
    let mut html_content = fs::read_to_string(Path::new(&index_path))
        .map_err(|e| AppError::ServerError(format!(
            "Failed to read index.html: {}", e
        )))?;
    
    // Extract the CSRF token
    let token = csrf_token.authenticity_token()
        .map_err(|_| AppError::ServerError("Failed to retrieve CSRF token".to_string()))?;
    
    // Get the frontend configuration with the CSRF token
    let frontend_config = get_serializable_frontend_config(
        &app_state.config.frontend, 
        token
    );
    
    // Serialize the configuration to JSON
    let config_json = serde_json::to_string(&frontend_config)
        .map_err(|e| AppError::ServerError(format!(
            "Failed to serialize frontend config: {}", e
        )))?;
    
    // Inject the configuration into the HTML by replacing the placeholder
    html_content = html_content.replace(
        "<!-- BACKEND_CONFIG -->", 
        &format!("<script>window.BACKEND_CONFIG = {};</script>", config_json)
    );
    
    // Configure HTTP headers for the response
    let headers = create_security_headers()?;
    
    // Return the complete response
    Ok((StatusCode::OK, headers, Html(html_content)))
}

/// Creates security headers for HTML responses
fn create_security_headers() -> Result<HeaderMap, AppError> {
    let mut headers = HeaderMap::new();
    
    // Set the content type
    headers.insert(
        header::CONTENT_TYPE, 
        "text/html; charset=utf-8".parse()
            .map_err(|_| AppError::ServerError("Invalid content-type header value".to_string()))?
    );
    
    // Add X-Content-Type-Options header to prevent MIME sniffing
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    
    // Add other security headers
    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );
    
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline';"),
    );
    
    Ok(headers)
}
