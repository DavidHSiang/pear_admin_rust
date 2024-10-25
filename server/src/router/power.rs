use axum::{
    routing::{get, patch},
    Router,
};

use crate::controller::{
    api::power::{
        create_power, delete_power, get_power, list_powers, list_powers_with_top_level,
        toggle_power_enable, update_power,
    },
    page::power::{create_power_page, edit_power_page, list_powers_page},
};

pub fn power_api_router() -> Router {
    Router::new()
        .route("/", get(list_powers).post(create_power))
        .route("/parents", get(list_powers_with_top_level))
        .route(
            "/:id",
            get(get_power).put(update_power).delete(delete_power),
        )
        .route("/:id/enable", patch(toggle_power_enable))
}

pub fn power_template_router() -> Router {
    Router::new()
        .route("/add", get(create_power_page))
        .route("/edit/:id", get(edit_power_page))
        .route("/", get(list_powers_page))
}
