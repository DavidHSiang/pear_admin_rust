use axum::{routing::get, Router};

use crate::controller::{
    api::photo::{create_photo, delete_photo, get_photo, list_photos, update_photo},
    page::photo::{create_photo_page, edit_photo_page, list_photos_page},
};

pub fn photo_api_router() -> Router {
    Router::new()
        .route("/", get(list_photos).post(create_photo))
        .route(
            "/:id",
            get(get_photo).put(update_photo).delete(delete_photo),
        )
}

// 增删查
pub fn photo_template_router() -> Router {
    Router::new()
        .route("/add", get(create_photo_page))
        .route("/edit", get(edit_photo_page))
        .route("/", get(list_photos_page))
}
