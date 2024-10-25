use anyhow::Result;
use captcha_rust::Captcha;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{controller::vo::CaptchaImage, models::user, utils::rand_utils::encrypt_password};

use super::{power::PowerService, role::RoleService};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginParams {
    pub username: String,
    pub password: String,
    pub captcha: String,
    pub uuid: String,
}

pub struct CommonService {}

impl CommonService {
    pub async fn login(txn: &DatabaseTransaction, login_params: LoginParams) -> Result<i32> {
        if login_params.uuid != encrypt_password(&login_params.captcha.to_lowercase(), "") {
            return Err(anyhow::anyhow!("验证码错误"));
        }

        let user = user::Entity::find()
            .filter(user::Column::Username.eq(login_params.username))
            .one(txn)
            .await?;

        let user = match user {
            Some(user) => user,
            None => return Err(anyhow::anyhow!("用户不存在")),
        };
        let pwd = encrypt_password(&login_params.password, &user.salt);
        info!("pwd: {}", pwd);
        if user.password_hash != encrypt_password(&login_params.password, &user.salt) {
            return Err(anyhow::anyhow!("密码错误"));
        }
        Ok(user.id)
    }

    pub async fn captcha() -> Result<CaptchaImage> {
        let captcha = Captcha::new(4, 240, 80);
        let uuid = encrypt_password(&captcha.text.to_lowercase(), "");
        Ok(CaptchaImage {
            captcha_on_off: true,
            uuid,
            img: captcha.base_img,
        })
    }

    pub async fn menu<M>(txn: &DatabaseTransaction, user_id: i32) -> Result<Vec<M>>
    where
        M: sea_orm::FromQueryResult + Send + Sync,
    {
        let role_ids = RoleService::get_role_ids_by_user_id(txn, user_id).await?;

        let menu = PowerService::get_menu_by_role_ids::<M>(txn, role_ids).await?;
        Ok(menu)
    }
}
