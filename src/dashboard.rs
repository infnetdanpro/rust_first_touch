use crate::models::{UserLinkCreate, UserLinksDashBoard};
use axum::http::HeaderMap;
use axum::response::Html;
use axum::{Extension, Form};
use nanoid::nanoid;
use simple_cookie::{SigningKey, decode_cookie};

pub async fn get_links_dashboard(
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(signing_key): Extension<SigningKey>,
    headers: HeaderMap,
) -> Result<Html<String>, String> {
    // взять куку, расшифровать, сверить ее через ключ и выдать ошибку, если что не так
    if let Some(cookie_headers) = headers.get("cookie") {
        let cookie_headers = cookie_headers.to_str().unwrap();
        for cookie_pair in cookie_headers.split(';') {
            if let Some((name, value)) = cookie_pair.split_once("=") {
                if name.trim() == "PHPSESSID" {
                    let decoded = decode_cookie(signing_key, "user_id", value.trim())
                        .map_err(|e| format!("Failed to decode cookie: {:?}", e))?;

                    let arr: [u8; 4] = decoded
                        .as_slice()
                        .try_into()
                        .map_err(|_| "Cookie data must be 4 bytes for i32".to_string())?;
                    let user_id = i32::from_le_bytes(arr);

                    // Get links for user
                    match sqlx::query_as!(
                        UserLinksDashBoard,
                        "SELECT short_code, destination_url, views  FROM links WHERE user_id = $1 ORDER BY id DESC LIMIT 100", // TODO: pagination
                        &user_id
                    )
                    .fetch_all(&pool)
                    .await
                    {
                        Ok(user_links) => {
                            let mut html = String::from(
                                "<html><head><title>Dashboard</title></head><body><p><form action=\"/create-link\" method=\"post\"><label for=\"destination_url\">Create new link:<input name=\"destination_url\"></input><button type=\"submit\">Create</button></form></p><hr><h1>Your Links</h1><table border='1'><tr><th>Short Code</th><th>Destination URL</th><th>Views</th></tr>",
                            );

                            for link in user_links {
                                html.push_str(&format!(
                                    "<tr><td><a href=\"/redirect/{}\">/redirect/{}</a></td><td>{}</td><td>{}</td></tr>",
                                    &link.short_code,
                                    &link.short_code,
                                    &link.destination_url,
                                    &link.views.unwrap().to_string()
                                ));
                            }
                            html.push_str("</table></body></html>");
                            return Ok(Html(html.as_str().parse().unwrap()));
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
        }
    }
    Err("Unauthorized".to_string())
}

#[axum::debug_handler]
pub async fn post_links_dashboard(
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(signing_key): Extension<SigningKey>,
    headers: HeaderMap,
    Form(create_link): Form<UserLinkCreate>,
) -> Result<Html<String>, String> {
    if let Some(cookie_headers) = headers.get("cookie") {
        let cookie_headers = cookie_headers.to_str().unwrap();
        for cookie_pair in cookie_headers.split(';') {
            if let Some((name, value)) = cookie_pair.split_once("=") {
                if name.trim() == "PHPSESSID" {
                    let decoded = decode_cookie(signing_key, "user_id", value.trim())
                        .map_err(|e| format!("Failed to decode cookie: {:?}", e))?;

                    let arr: [u8; 4] = decoded
                        .as_slice()
                        .try_into()
                        .map_err(|_| "Cookie data must be 4 bytes for i32".to_string())?;
                    let user_id = i32::from_le_bytes(arr);

                    let short_code = nanoid!(); //=> "Yo1Tr9F3iF-LFHX9i9GvA"
                    // Get links for user
                    let _created_link = sqlx::query!(
                        "INSERT INTO links (user_id, short_code, destination_url) VALUES ($1, $2, $3) RETURNING id", // TODO: pagination
                        &user_id,
                        &short_code,
                        &create_link.destination_url
                    )
                    .fetch_one(&pool)
                    .await
                    .map_err(|e| e.to_string())?;

                    return Ok(Html("<html><head><title>Dashboard</title></head><body><p>Link created</p><p>Go to <a href=\"/dashboard\">dashboard</a></p></body></html>".to_string()));
                }
            }
        }
    }
    Err("Unauthorized".to_string())
}
