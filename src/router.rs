use crate::auth::{get_login, logout, post_login, post_register};
use crate::dashboard::{get_links_dashboard, post_links_dashboard};
use crate::redirect::get_redirect;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Extension, Router};
use simple_cookie::SigningKey;

pub fn create_router(pool: sqlx::PgPool, signing_key: SigningKey) -> Router {
    Router::new()
        .route("/", get(|| async { Html("<html><head><title>Main page</title></head><body><ul><li><a href=\"/dashboard\">Dashboard (auth only)</a></li><li><a href=\"/register\">Register</a></li><li><a href=\"/login\">Login</a></li></ul></body></html>") }))
        .route("/register", get(|| async { Html("<html><head><title>Register page</title></head><body><form action=\"/register\" method=\"POST\"><p><label for=\"email\">Your email</label><input type=\"email\" name=\"email\" id=\"email\" required></p><p><label for=\"password\">Your password</label><input type=\"password\" name=\"password\" id=\"password\" required></p><p><label for=\"confirm_password\">Confirm password</label><input type=\"password\" name=\"confirm_password\" id=\"confirm_password\" required></p><p><button type=\"submit\">Register</button></p></form></body></html>") }).post(post_register))
        .route("/login", get(get_login).post(post_login))
        .route("/dashboard", get(get_links_dashboard))
        .route("/create-link", post(post_links_dashboard))
        .route("/redirect/{short_code}", get(get_redirect))
        .route("/logout", get(logout))
        .layer((Extension(pool), Extension(signing_key)))
}
