use super::datetime_format;
use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct PhotoPageVo {
    pub id: i32,
    pub name: String,
    pub href: Option<String>,
    pub mime: String,
    pub size: String,
    #[serde(with = "datetime_format")]
    pub create_at: NaiveDateTime,
}
