use std::env;

use sea_orm::ActiveValue::NotSet;
use serde::Deserialize;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use validator::Validate;

use crate::auto_service;
use crate::models::photo;
use crate::utils::auto_service::prelude::*;

auto_service!(PhotoService, photo, {});

#[derive(Debug, Deserialize, Validate)]
pub struct MergePhotoModel {
    id: Option<i32>,
    name: String,
    mime: String,
    size: usize,
    data: Vec<u8>,
}

impl MergePhotoModel {
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn new(name: String, mime: String, size: usize, data: Vec<u8>) -> Self {
        Self {
            id: None,
            name,
            mime,
            size,
            data,
        }
    }
}

impl From<MergePhotoModel> for photo::ActiveModel {
    fn from(model: MergePhotoModel) -> Self {
        let href = format!("/static/upload/{}", model.name);
        Self {
            id: model.id.map_or_else(|| NotSet, Set),
            name: Set(model.name),
            href: Set(href),
            mime: Set(model.mime),
            size: Set(model.size.to_string()),
            ..Default::default()
        }
    }
}

impl PhotoService {
    pub async fn page<M>(
        txn: &DatabaseTransaction,
        page_params: impl Into<PageParams>,
    ) -> Result<PageData<M>>
    where
        M: FromQueryResult + Send + Sync,
    {
        Self::_page::<M, NoneSearchParams>(txn, page_params, NoneSearchParams {}).await
    }

    pub async fn delete(txn: &DatabaseTransaction, photo_id: i32) -> Result<()> {
        let photo: photo::Model = Self::get_by_id(txn, photo_id).await?;
        // href 路径是相对路径，所以需要拼接 href内容类似于 /static/upload/1617291580000.jpg
        // project_root 是项目的根目录
        let current_dir = env::current_dir()?;
        let file_path = format!("{}{}", current_dir.to_string_lossy(), photo.href);

        info!("delete_photo: photo_id={}, href={}", photo_id, file_path);
        fs::remove_file(file_path).await?;
        Self::_delete_by_id(txn, photo_id).await
    }

    pub async fn add(
        txn: &DatabaseTransaction,
        create_params: impl TryInto<MergePhotoModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let create_model: MergePhotoModel = create_params.try_into().map_err(Into::into)?;
        create_model.validate()?;

        // 通过数据库判断文件是否存在
        let photo: Option<photo::Model> = photo::Entity::find()
            .filter(photo::Column::Name.eq(create_model.name.clone()))
            .one(txn)
            .await?;

        if photo.is_some() {
            return Err(anyhow::anyhow!("文件已存在"));
        }

        let mut file = File::create(format!("./static/upload/{}", create_model.name))
            .await
            .unwrap();
        file.write_all(&create_model.data).await.unwrap();

        Self::_add(txn, create_model).await
    }

    pub async fn update(
        txn: &DatabaseTransaction,
        update_params: impl TryInto<MergePhotoModel, Error: Into<anyhow::Error>>,
    ) -> Result<()> {
        let update_model: MergePhotoModel = update_params.try_into().map_err(Into::into)?;
        update_model.validate()?;

        let photo: Option<photo::Model> = photo::Entity::find()
            .filter(photo::Column::Name.eq(update_model.name.clone()))
            .one(txn)
            .await?;

        if let Some(photo) = photo {
            if photo.id != update_model.id.unwrap() {
                return Err(anyhow::anyhow!("文件已存在"));
            }
        }

        let photo: photo::Model = Self::get_by_id(txn, update_model.id.unwrap()).await?;
        let old_file_path = format!("./static/upload/{}", photo.name);
        fs::remove_file(old_file_path).await?;

        let mut file = File::create(format!("./static/upload/{}", update_model.name))
            .await
            .unwrap();
        file.write_all(&update_model.data).await.unwrap();

        Self::_update(txn, update_model).await
    }
}
