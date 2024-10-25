use regex::Regex;
use serde::Deserialize;
use validator::ValidationError;

use crate::utils::auto_service::models::PageParams;

impl From<PageParamsOption> for PageParams {
    fn from(option: PageParamsOption) -> Self {
        Self {
            page_num: option.page.unwrap_or(1),
            page_size: option.limit.unwrap_or(10),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PageParamsOption {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

lazy_static::lazy_static! {
    static ref NO_SPACE_REGEX: Regex = Regex::new(r"^\S+$").unwrap();
}

// 自定义密码验证函数
#[allow(unused)]
fn validate_password(password: &str) -> Result<(), ValidationError> {
    // 长度必须大于8位
    if password.len() <= 8 {
        return Err(ValidationError::new("length"));
    }
    // 必须包含大写字母
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("uppercase"));
    }
    // 必须包含小写字母
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("lowercase"));
    }
    // 必须包含数字
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("numeric"));
    }
    // 必须包含特殊符号
    let special_chars = "!@#$%^&*()-_=+[]{}|;:'\",.<>/?`~";
    if !password.chars().any(|c| special_chars.contains(c)) {
        return Err(ValidationError::new("special_char"));
    }
    Ok(())
}
