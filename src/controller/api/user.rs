use axum::response::IntoResponse;

pub async fn list_users() -> impl IntoResponse {
    "All users"
}

pub async fn create_user() -> impl IntoResponse {
    "Create a user"
}

pub async fn get_user() -> impl IntoResponse {
    "Get a specific user"
}

pub async fn update_user() -> impl IntoResponse {
    "Update a user"
}

pub async fn delete_user() -> impl IntoResponse {
    "Delete a user"
}
