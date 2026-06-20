use axum::{
    Router,
    routing::{get, post},
};
use rust_api_test::routes::{create_post, list_posts};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/posts", get(list_posts).post(create_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
