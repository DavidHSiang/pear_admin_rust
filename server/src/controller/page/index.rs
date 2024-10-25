use axum::response::{Html, IntoResponse, Redirect};
use tower_sessions::Session;
use tracing::info;

use crate::{router::HANDLEBARS, service::monitor::MonitorService};

pub async fn index() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("index", &()).unwrap();
    Html(rendered)
}

pub async fn welcome() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("welcome", &()).unwrap();
    Html(rendered)
}

pub async fn login_page() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let rendered = handlebars.render("login", &()).unwrap();
    Html(rendered)
}

pub async fn logout(session: Session) -> impl IntoResponse {
    info!("logout");
    session.clear().await;
    Redirect::to("/login").into_response()
}

pub async fn monitor() -> impl IntoResponse {
    let handlebars = HANDLEBARS.clone();
    let sysinfo = MonitorService::system_info().await;
    let rendered = handlebars.render("monitor", &sysinfo).unwrap();
    Html(rendered)
}
