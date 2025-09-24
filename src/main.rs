mod auth;
mod dashboard;
mod models;
mod redirect;
mod router;

use crate::router::create_router;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // let signing_key = generate_signing_key();
    let signing_key = [0u8; 32];

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/shortlinker")
        .await
        .expect("failed to connect to database");
    let app = create_router(pool, signing_key);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
