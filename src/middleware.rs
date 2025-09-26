use axum::Extension;
use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use simple_cookie::{SigningKey, decode_cookie};

pub async fn auth_middleware(
    headers: HeaderMap,
    Extension(signing_key): Extension<SigningKey>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match extract_user_id(&headers, &signing_key) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(next.run(req).await)
        }
        Err(status) => Err(status),
    }
}

fn extract_user_id(headers: &HeaderMap, signing_key: &SigningKey) -> Result<i32, StatusCode> {
    if let Some(cookie_headers) = headers.get("cookie") {
        let cookie_headers = cookie_headers
            .to_str()
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        for cookie_pair in cookie_headers.split(';') {
            if let Some((name, value)) = cookie_pair.split_once("=") {
                if name.trim() == "PHPSESSID" {
                    let decoded = decode_cookie(*signing_key, "user_id", value.trim())
                        .map_err(|_| StatusCode::UNAUTHORIZED)?;

                    let arr: [u8; 4] = decoded
                        .as_slice()
                        .try_into()
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                    let user_id = i32::from_le_bytes(arr);
                    return Ok(user_id);
                }
            }
        }
        Err(StatusCode::UNAUTHORIZED)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
