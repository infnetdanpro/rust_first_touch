mod auth;
mod dashboard;
mod models;
mod redirect;
mod router;
mod middleware;

use crate::router::create_router;
use simple_cookie::SigningKey;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // let signing_key = generate_signing_key();
    let signing_key = std::env::var("SIGNING_KEY")
        .map(|s| s.into_bytes())
        .unwrap_or_else(|_| vec![0; 32]);

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/shortlinker".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let app = create_router(pool, SigningKey::try_from(signing_key).unwrap());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
