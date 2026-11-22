use axum::Router;
use tower_http::services::ServeDir;

use crate::config;

pub async fn run() {
    let app = Router::new().fallback_service(ServeDir::new(config::OUTPUT_DIR));

    let socket = format!("{}:{}", config::DEV_HOST, config::DEV_PORT);

    let listener = tokio::net::TcpListener::bind(&socket)
        .await
        .expect("Failed to bind development server");

    log::info!("Development server running at http://{socket}");

    axum::serve(listener, app)
        .await
        .expect("Development server failed");
}
