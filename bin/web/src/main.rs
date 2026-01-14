use std::{net::SocketAddr, path::PathBuf};

use axum::{Router, routing::get_service};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    // Get configuration from cargo-leptos environment variables
    let addr_str =
        std::env::var("LEPTOS_SITE_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "target/site".to_string());

    let addr: SocketAddr = addr_str.parse().expect("Invalid LEPTOS_SITE_ADDR");

    // Try multiple possible locations for index.html:
    // 1. "index.html" - when run from bin/web/ directory (cargo-leptos watch)
    // 2. "bin/web/index.html" - when run from workspace root
    let index_candidates = ["index.html", "bin/web/index.html"];
    let index_file = index_candidates
        .iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
        .unwrap_or_else(|| PathBuf::from("index.html"));

    println!("Serving static files from {site_root} at http://{addr}");
    println!("Using index file: {index_file:?}");

    // wasm-bindgen generates JS that requests <name>_bg.wasm, but cargo-leptos
    // outputs <name>.wasm. We handle this mismatch by serving the actual wasm file
    // at both paths.
    let wasm_file = PathBuf::from(&site_root).join("pkg/longtime.wasm");

    // For SPA routing, return index.html for any path that doesn't match a static file.
    // We use ServeFile as the fallback which returns 200 OK (not 404).
    let serve_dir = ServeDir::new(&site_root).fallback(ServeFile::new(&index_file));

    let app = Router::new()
        // Handle the wasm-bindgen expected filename
        .route(
            "/pkg/longtime_bg.wasm",
            get_service(ServeFile::new(&wasm_file)),
        )
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve application");
}
