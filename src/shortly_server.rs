use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;

use crate::db_ops::get_record;

#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(root))
        .route("/:base", get(send_to));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\nServer running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Welcome to the Shortly Web Server!\nTo navigate to your url, simply add '/your_short_url' to the address bar"
}

async fn send_to(Path(base): Path<String>) -> impl IntoResponse {
    let long_url = get_record(&base);

    if long_url.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Short URL not found."})),
        ));
    }

    Ok(Redirect::to(&long_url))
}
