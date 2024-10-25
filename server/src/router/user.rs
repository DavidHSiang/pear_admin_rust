use crate::controller::{
    api::user::{create_user, delete_user, get_user, list_users, toggle_user_enable, update_user},
    page::user::{create_user_page, edit_user_page, list_users_page},
};
use axum::{
    routing::{get, patch},
    Router,
};

pub fn user_api_router() -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/:id/enable", patch(toggle_user_enable))
}

pub fn user_template_router() -> Router {
    Router::new()
        .route("/add", get(create_user_page))
        .route("/edit/:id", get(edit_user_page))
        .route("/", get(list_users_page))
}
