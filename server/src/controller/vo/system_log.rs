use super::datetime_format;
use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct SystemLogPageVo {
    pub id: i32,
    pub method: Option<String>,
    pub uid: Option<i32>,
    pub url: Option<String>,
    pub desc: Option<String>,
    pub ip: Option<String>,
    pub success: Option<bool>,
    pub user_agent: Option<String>,
    #[serde(with = "datetime_format")]
    pub create_at: NaiveDateTime,
    pub log_type: i32,
}
