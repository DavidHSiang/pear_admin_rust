use axum::{
    routing::{get, patch},
    Router,
};

use crate::controller::{
    api::dept::{create_dept, delete_dept, get_dept, list_depts, toggle_dept_enable, update_dept},
    page::dept::{create_dept_page, edit_dept_page, list_depts_page},
};

pub fn dept_api_router() -> Router {
    Router::new()
        .route("/", get(list_depts).post(create_dept))
        .route("/:id", get(get_dept).put(update_dept).delete(delete_dept))
        .route("/:id/enable", patch(toggle_dept_enable))
}

pub fn dept_template_router() -> Router {
    Router::new()
        .route("/add", get(create_dept_page))
        .route("/edit/:id", get(edit_dept_page))
        .route("/", get(list_depts_page))
}
