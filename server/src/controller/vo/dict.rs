use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct DictTypePageVo {
    pub id: i32,
    pub type_name: String,
    pub type_code: String,
    pub description: Option<String>,
    pub enable: i32,
    pub create_at: String,
    pub update_at: String,
}

#[derive(Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct DictDataPageVo {
    pub id: i32,
    pub data_label: String,
    pub data_value: String,
    pub type_code: String,
    pub is_default: i32,
    pub enable: i32,
    pub remark: Option<String>,
    pub create_at: String,
    pub update_at: String,
}
