mod user;

use std::fmt;

use anyhow::Error;
use sea_orm::FromQueryResult;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use user::*;

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct EmptyVo {}

pub const CODE_SUCCESS: &str = "0";
pub const CODE_FAIL: &str = "-1";

/// The http interface returns the model structure, providing basic json data structures such as code, msg, and data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + Clone,
{
    pub fn from_result(result: Result<T, Error>, msg: &str) -> Self {
        match result {
            Ok(data) => Self::from(data, msg),
            Err(e) => Self::from_error(e.to_string()),
        }
    }

    pub fn from(data: T, msg: &str) -> Self {
        Self {
            code: Some(CODE_SUCCESS.to_string()),
            msg: Some(msg.to_string()),
            data: Some(data),
        }
    }

    pub fn from_error(error: String) -> Self {
        Self {
            code: Some(CODE_FAIL.to_string()),
            msg: Some(error),
            data: None,
        }
    }

    pub fn json(self) -> axum::Json<RespVO<T>> {
        axum::Json(self)
    }
}

impl<T> fmt::Display for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json_str = serde_json::to_string(self).map_err(|_| fmt::Error)?;
        write!(f, "{}", json_str)
    }
}
