use sea_orm::Set;
use serde::Deserialize;

use crate::{
    domain::models::admin, service::user::AdminSearchParams,
    utils::auto_service::models::PageParams,
};

#[derive(Debug, Clone, Deserialize)]
pub struct AdminPageDTO {
    page: Option<u64>,
    limit: Option<u64>,
    login_name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

impl From<AdminPageDTO> for AdminSearchParams {
    fn from(value: AdminPageDTO) -> Self {
        Self {
            login_name: value.login_name,
            phone: value.phone,
            email: value.email,
        }
    }
}

impl From<AdminPageDTO> for PageParams {
    fn from(value: AdminPageDTO) -> Self {
        Self {
            page_num: value.page.unwrap_or(1),
            page_size: value.limit.unwrap_or(10),
        }
    }
}

#[allow(unused)] // TODO remove this
#[derive(Debug, Clone, Deserialize)]
pub struct AdminAddDTO {
    login_name: Option<String>,
    real_name: Option<String>,
    password: Option<String>,
    level: Option<i32>,
    role_ids: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    avatar: Option<String>,
    remark: Option<String>,
    salt: Option<String>,
    last_ip: Option<String>,
    last_login: Option<String>,
    status: Option<i32>,
}

impl TryFrom<AdminAddDTO> for admin::ActiveModel {
    type Error = anyhow::Error;

    fn try_from(value: AdminAddDTO) -> Result<Self, Self::Error> {
        if value.login_name.is_none() || value.login_name.as_ref().unwrap().is_empty() {
            return Err(RequestError::ValidationError("login_name".to_string()).into());
        }
        if value.real_name.is_none() || value.real_name.as_ref().unwrap().is_empty() {
            return Err(RequestError::ValidationError("real_name".to_string()).into());
        }
        if value.password.is_none() || value.password.as_ref().unwrap().is_empty() {
            return Err(RequestError::ValidationError("password".to_string()).into());
        }

        let model = Self {
            login_name: Set(value.login_name),
            ..Default::default()
        };
        Ok(model)
    }
}

// 使用 thiserror 定义错误类型
#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    // 校验错误
    #[error("Validation error on field: {0}")]
    ValidationError(String),
}
