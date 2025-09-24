use crate::models::{LogIn, SignUp, UserDb};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::{Extension, Form};
use bcrypt::{DEFAULT_COST, hash, verify};
use simple_cookie::{SigningKey, encode_cookie};

pub async fn post_register(
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(signing_key): Extension<SigningKey>,
    Form(signup): Form<SignUp>,
) -> Result<Response, String> {
    if signup.password != signup.confirm_password {
        return Err("Passwords do not match".to_string());
    }
    let hashed_pwd = hash(&signup.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        r#"INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id"#,
        &signup.email,
        &hashed_pwd
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let encoded = encode_cookie(signing_key, "user_id", result.id.to_le_bytes().to_vec());

    let cookie_value = format!(
        "PHPSESSID={}; HttpOnly; Secure; SameSite=Strict; Max-Age=86400",
        encoded
    );

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(
            Html("Registration successful! You are now logged in.")
                .into_response()
                .into_body(),
        )
        .unwrap();
    let (mut parts, body) = response.into_parts();
    parts.headers.extend(headers);
    Ok(Response::from_parts(parts, body))
}

pub async fn get_login() -> Html<&'static str> {
    Html(
        "<html><head><title>Login page</title></head><body><form action=\"/login\" method=\"post\"><p><label for=\"email\">Email</label><input type=\"email\" name=\"email\"></input></p><p><label for=\"password\">Password</label><input type=\"password\" name=\"password\"></input></p><p><button type=\"submit\">Login</button></p></form></body></html>",
    )
}

pub async fn post_login(
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(signing_key): Extension<SigningKey>,
    Form(login): Form<LogIn>,
) -> Result<Response, StatusCode> {
    let user_db = sqlx::query_as!(
        UserDb,
        "SELECT id, password FROM users WHERE email = $1",
        &login.email,
    )
    .fetch_one(&pool)
    .await;

    let user_db = match user_db {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            // Пользователь не найден
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(_) => {
            // Другие ошибки базы данных
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let valid = match verify(login.password, &user_db.password) {
        Ok(is_valid) => is_valid,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let encoded = encode_cookie(signing_key, "user_id", user_db.id.to_le_bytes().to_vec());

    let cookie_value = format!(
        "PHPSESSID={}; HttpOnly; Secure; SameSite=Strict; Max-Age=86400",
        encoded
    );

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Html("Login successful!").into_response().into_body())
        .unwrap();
    let (mut parts, body) = response.into_parts();
    parts.headers.extend(headers);
    Ok(Response::from_parts(parts, body))
}

pub async fn logout(Extension(_pool): Extension<sqlx::PgPool>) -> Result<Response, String> {
    let mut headers = HeaderMap::new();
    let cookie_value = "PHPSESSID=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; expires=Thu, 01 Jan 1970 00:00:00 GMT";
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Html("You are unlogged").into_response().into_body())
        .unwrap();
    let (mut parts, body) = response.into_parts();
    parts.headers.extend(headers);
    Ok(Response::from_parts(parts, body))
}
