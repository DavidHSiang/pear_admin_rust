use axum::response::{Html, IntoResponse};

use crate::router::HANDLEBARS;

pub async fn list_system_logs_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("system/system_log/list", &()).unwrap();
    Html(rendered)
}
