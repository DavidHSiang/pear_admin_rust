use axum::{
    routing::{get, post},
    Router,
};

use crate::controller::{
    api::common::{captcha, configs, login, menu, monitor_polling},
    page::index::{index, login_page, logout, monitor, welcome},
};

pub fn common_api_router() -> Router {
    Router::new()
        .route("/configs", get(configs))
        .route("/menu", get(menu))
        .route("/captcha", get(captcha))
        .route("/login", post(login))
        .route("/monitor_polling", get(monitor_polling))
}

pub fn common_template_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/welcome", get(welcome))
        .route("/login", get(login_page))
        .route("/logout", get(logout))
        .route("/monitor", get(monitor))
}
