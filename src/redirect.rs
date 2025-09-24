use crate::models::LinkRedirect;
use axum::Extension;
use axum::extract::Path;
use axum::response::{Html, IntoResponse, Redirect, Response};

pub async fn get_redirect(
    Extension(pool): Extension<sqlx::PgPool>,
    Path(short_code): Path<String>,
) -> Response {
    // get a link from DB
    match sqlx::query_as!(
        LinkRedirect,
        "SELECT destination_url FROM links WHERE short_code = $1",
        &short_code
    )
    .fetch_one(&pool)
    .await
    {
        Ok(link_db) => {
            let _updated_link = sqlx::query!(
                "UPDATE links SET views = views + 1 WHERE short_code = $1;",
                &short_code
            )
            .execute(&pool)
            .await;

            Redirect::permanent(&link_db.destination_url).into_response()
        }
        Err(_) => {
            let mut resp = String::from("");
            resp.push_str(&format!("<p>Error: {} is not found</p>", &short_code));
            Html(resp).into_response()
        }
    }
}
