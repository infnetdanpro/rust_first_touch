use crate::models::{AuthedUser, UserLinkCreate, UserLinksDashBoard};
use axum::response::Html;
use axum::{Extension, Form};
use nanoid::nanoid;

#[axum::debug_handler]
pub async fn get_links_dashboard(
    Extension(authed_user): Extension<AuthedUser>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Html<String>, String> {
    // взять куку, расшифровать, сверить ее через ключ и выдать ошибку, если что не так
    // Get links for user
    match sqlx::query_as!(
        UserLinksDashBoard,
        "SELECT short_code, destination_url, views  FROM links WHERE user_id = $1 ORDER BY id DESC LIMIT 100", // TODO: pagination
        &authed_user.user_id
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
            Ok(Html(html.as_str().parse().unwrap()))
        }
        Err(e) => Err(e.to_string()),
    }
}

#[axum::debug_handler]
pub async fn post_links_dashboard(
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(authed_user): Extension<AuthedUser>,
    Form(create_link): Form<UserLinkCreate>,
) -> Result<Html<String>, String> {
    let short_code = nanoid!(); //=> "Yo1Tr9F3iF-LFHX9i9GvA"
    let _created_link = sqlx::query!(
        "INSERT INTO links (user_id, short_code, destination_url) VALUES ($1, $2, $3) RETURNING id", // TODO: pagination
        &authed_user.user_id,
        short_code,
        &create_link.destination_url
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(Html("<html><head><title>Dashboard</title></head><body><p>Link created</p><p>Go to <a href=\"/dashboard\">dashboard</a></p></body></html>".to_string()))
}
