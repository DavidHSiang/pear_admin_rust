use axum::response::{Html, IntoResponse};

use crate::router::HANDLEBARS;

pub async fn create_photo_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/photo/add", &()).unwrap();
    Html(rendered)
}

pub async fn list_photos_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/photo/list", &()).unwrap();
    Html(rendered)
}

pub async fn edit_photo_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/photo/edit", &()).unwrap();
    Html(rendered)
}
