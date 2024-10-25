use super::datetime_format;
use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct UserPageVo {
    id: i64,
    username: String,
    real_name: String,
    enable: i32,
    #[serde(with = "datetime_format")]
    create_at: NaiveDateTime,
    #[serde(with = "datetime_format")]
    update_at: NaiveDateTime,
    dept_id: i32,
    dept_name: String,
}
