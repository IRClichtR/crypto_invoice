use crate::{
    AppState,
    routes::home::serve_home,
};
use tower_http::{services::ServeDir, cors::CorsLayer};
use hyper::header;
use std::sync::Arc;
use axum::{Router, routing::get};
use axum_csrf::{CsrfConfig, CsrfLayer};
use tower_cookies::CookieManagerLayer;

pub fn create_app_routes(
    app_state: Arc<AppState>,
    csrf_config: CsrfConfig,
    cors_config: CorsLayer,
) -> Router {
    // Create router
    let app = Router::new()
        .route("/", get(serve_home))
        // other routes to be added here
        .nest_service(
            "/assets", ServeDir::new(format!("{}/assets", app_state.vue_dist_path))
        )
        .layer(CookieManagerLayer::new())
        .layer(CsrfLayer::new(csrf_config.clone()))
        .layer(
            tower_http::set_header::SetResponseHeaderLayer::if_not_present(
                header::X_CONTENT_TYPE_OPTIONS,
                header::HeaderValue::from_static("nosniff"),
            )
        )
        .layer(cors_config)
        // .layer(from_fn(utils::server_utils::restrict_origin))
        .with_state(app_state);

    // Return the configured router
    app
}