use std::net::SocketAddr;
use axum::{routing::get, Router, response::Redirect};

use crate::db_ops::get_url;

#[tokio::main]
pub async fn start_server() {

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/:base", get(send_to))
        .route("/missing", get(missing));
    
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Shortly Web Server!\nTo navigate to your url, simply add '/your_short_url' to the address bar"
}

async fn missing() -> &'static str {
    "Invalid short base: Please verify your unique base."
}

async fn send_to(axum::extract::Path(base): axum::extract::Path<String>) -> Redirect {
    let long_url: String = get_url(&base);
    Redirect::to(&long_url)
}