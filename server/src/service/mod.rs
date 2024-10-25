use regex::Regex;
use validator::ValidationError;

pub mod common;
pub mod dept;
pub mod dict_data;
pub mod dict_type;
pub mod monitor;
pub mod photo;
pub mod power;
pub mod role;
pub mod role_power;
pub mod system_log;
pub mod user;
pub mod user_role;

lazy_static::lazy_static! {
    static ref NO_SPACE_REGEX: Regex = Regex::new(r"^\S+$").unwrap();
}

// 自定义密码验证函数
fn validate_password(password: &str) -> Result<(), ValidationError> {
    // 长度必须大于8位
    if password.len() <= 8 {
        return Err(ValidationError::new("长度必须大于8位"));
    }
    // 必须包含大写字母
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("需包含大写字母"));
    }
    // 必须包含小写字母
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("需包含小写字母"));
    }
    // 必须包含数字
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("需包含数字"));
    }
    // 必须包含特殊符号
    let special_chars = "!@#$%^&*()-_=+[]{}|;:'\",.<>/?`~";
    if !password.chars().any(|c| special_chars.contains(c)) {
        return Err(ValidationError::new("需包含特殊符号"));
    }
    Ok(())
}
