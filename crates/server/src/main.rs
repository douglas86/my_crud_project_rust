use axum::Router;
use axum::routing::get;

async fn get_handler() -> &'static str {
    "hello world"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
