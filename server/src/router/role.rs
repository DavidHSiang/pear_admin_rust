use axum::{
    routing::{get, patch},
    Router,
};

use crate::controller::{
    api::role::{
        create_role, delete_role, get_role, list_role_powers, list_roles, toggle_role_enable,
        update_powers_for_role, update_role,
    },
    page::role::{create_role_page, edit_role_page, list_roles_page, set_powers_for_role_page},
};

pub fn role_api_router() -> Router {
    Router::new()
        .route("/", get(list_roles).post(create_role))
        .route("/:id", get(get_role).put(update_role).delete(delete_role))
        .route("/:id/enable", patch(toggle_role_enable))
        .route(
            "/:role_id/powers",
            get(list_role_powers).put(update_powers_for_role),
        )
}

pub fn role_template_router() -> Router {
    Router::new()
        .route("/add", get(create_role_page))
        .route("/edit/:id", get(edit_role_page))
        .route("/set_powers/:id", get(set_powers_for_role_page))
        .route("/", get(list_roles_page))
}
