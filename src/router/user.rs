use crate::controller::{
    api::user::{create_user, delete_user, get_user, list_users, update_user},
    page::user::{create_user_page, edit_user_page, list_users_page},
};
use axum::{routing::get, Router};

pub fn user_api_router() -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
}

pub fn user_template_router() -> Router {
    Router::new()
        .route("/add", get(create_user_page))
        .route("/edit", get(edit_user_page))
        .route("/", get(list_users_page))
}
