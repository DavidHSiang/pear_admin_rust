use axum::{
    extract::{Multipart, Path, Query},
    response::IntoResponse,
};
use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
    controller::{
        dto::PageParamsOption,
        vo::{PhotoPageVo, RespVO},
    },
    db,
    service::photo::{MergePhotoModel, PhotoService},
};

pub async fn list_photos(Query(page_params): Query<PageParamsOption>) -> impl IntoResponse {
    info!("list_photos");
    let txn = db!().begin().await.unwrap();
    let result = PhotoService::page::<PhotoPageVo>(&txn, page_params).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn create_photo(mut multipart: Multipart) -> impl IntoResponse {
    let txn = db!().begin().await.unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
        // let field_name = field.name().unwrap().to_string();
        let name = field.file_name().unwrap().to_string();
        let mime = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap().to_vec();
        let size = data.len();

        let create_model = MergePhotoModel::new(name, mime, size, data);

        let result = PhotoService::add(&txn, create_model).await;
        if result.is_err() {
            txn.rollback().await.unwrap();
            return RespVO::from_result(result, "上传失败").json();
        }
    }
    txn.commit().await.unwrap();
    RespVO::from((), "上传成功").json()
}

pub async fn get_photo(Path(photo_id): Path<i32>) -> impl IntoResponse {
    info!("get_photo: photo_id={}", photo_id);
    let txn = db!().begin().await.unwrap();
    let result = PhotoService::get_by_id::<PhotoPageVo>(&txn, photo_id).await;
    txn.commit().await.unwrap();
    RespVO::from_result(result, "查询成功").json()
}

pub async fn update_photo(
    Path(photo_id): Path<i32>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let txn = db!().begin().await.unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
        // let field_name = field.name().unwrap().to_string();
        let name = field.file_name().unwrap().to_string();
        let mime = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap().to_vec();
        let size = data.len();

        let mut update_model = MergePhotoModel::new(name, mime, size, data);
        update_model.set_id(photo_id);

        PhotoService::update(&txn, update_model).await.unwrap();
    }
    txn.commit().await.unwrap();
    RespVO::from((), "上传成功").json()
}

pub async fn delete_photo(Path(photo_ids): Path<String>) -> impl IntoResponse {
    info!("delete_photo: photo_id={}", photo_ids);
    let ids: Vec<i32> = photo_ids
        .split(',')
        .filter_map(|id| id.parse::<i32>().ok())
        .collect();

    let txn = db!().begin().await.unwrap();

    for photo_id in &ids {
        let result = PhotoService::delete(&txn, *photo_id).await;
        if result.is_err() {
            txn.rollback().await.unwrap();
            return RespVO::from_result(result, "删除失败").json();
        }
    }

    txn.commit().await.unwrap();
    RespVO::from((), "删除成功").json()
}
