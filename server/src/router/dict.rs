use axum::{
    routing::{get, patch},
    Router,
};

use crate::controller::{
    api::{
        dict_data::{
            create_dict_data, delete_dict_data, get_dict_data, list_dict_data,
            toggle_dict_data_enable, update_dict_data,
        },
        dict_type::{
            create_dict_type, delete_dict_type, get_dict_type, list_dict_types,
            toggle_dict_type_enable, update_dict_type,
        },
    },
    page::dict::{
        create_dict_data_page, create_dict_type_page, edit_dict_data_page, edit_dict_type_page,
        list_dict_page,
    },
};

pub fn dict_api_router() -> Router {
    Router::new()
        .nest("/type", dict_type_api_router())
        .nest("/data", dict_data_api_router())
}

pub fn dict_template_router() -> Router {
    Router::new()
        .route("/", get(list_dict_page))
        .nest("/type", dict_type_template_router())
        .nest("/data", dict_data_template_router())
}

pub fn dict_type_api_router() -> Router {
    Router::new()
        .route("/", get(list_dict_types).post(create_dict_type))
        .route(
            "/:id",
            get(get_dict_type)
                .put(update_dict_type)
                .delete(delete_dict_type),
        )
        .route("/:id/enable", patch(toggle_dict_type_enable))
}

pub fn dict_type_template_router() -> Router {
    Router::new()
        .route("/add", get(create_dict_type_page))
        .route("/edit/:id", get(edit_dict_type_page))
}

pub fn dict_data_api_router() -> Router {
    Router::new()
        .route("/", get(list_dict_data).post(create_dict_data))
        .route(
            "/:id",
            get(get_dict_data)
                .put(update_dict_data)
                .delete(delete_dict_data),
        )
        .route("/:id/enable", patch(toggle_dict_data_enable))
}

pub fn dict_data_template_router() -> Router {
    Router::new()
        .route("/add/:type_code", get(create_dict_data_page))
        .route("/edit/:id", get(edit_dict_data_page))
}
