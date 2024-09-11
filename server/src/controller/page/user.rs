use axum::response::{Html, IntoResponse};

use crate::router::HANDLEBARS;

pub async fn create_user_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/user/add", &()).unwrap();
    Html(rendered)
}

pub async fn list_users_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/user/list", &()).unwrap();
    Html(rendered)
}

pub async fn edit_user_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/user/edit", &()).unwrap();
    Html(rendered)
}
