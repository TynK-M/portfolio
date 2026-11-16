use axum::Router;
use tower_http::services::ServeDir;

pub async fn run() {
    let app = Router::new().fallback_service(ServeDir::new("docs"));

    const HOST: &str = "127.0.0.1";
    const PORT: usize = 1312;

    let socket = format!("{HOST}:{PORT}");

    let listener = tokio::net::TcpListener::bind(&socket)
        .await
        .expect("Failed to bind development server");

    log::info!("Development server running at http://{socket}");

    axum::serve(listener, app)
        .await
        .expect("Development server failed");
}
