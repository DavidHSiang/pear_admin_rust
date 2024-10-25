use axum::{routing::get, Router};

use crate::controller::{
    api::system_log::list_system_logs, page::system_log::list_system_logs_page,
};

pub fn system_log_api_router() -> Router {
    Router::new().route("/", get(list_system_logs))
}

pub fn system_log_template_router() -> Router {
    Router::new().route("/", get(list_system_logs_page))
}
