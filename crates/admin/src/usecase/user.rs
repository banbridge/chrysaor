use super::{ListUserInput, ListUserOutput, LoginInput};
use crate::data::AdminRepo;
use crate::domain;
use crate::util::jwt::{JWT, Principal};
use async_trait::async_trait;
use common::crypt;
use common::error::{BizError, BizResult};
use sea_orm::Iden;
use std::sync::Arc;

pub struct UserUsecase {
    admin_repo: Arc<AdminRepo>,
    jwt: Arc<JWT>,
}

impl UserUsecase {
    pub fn new(admin_repo: Arc<AdminRepo>, jwt: Arc<JWT>) -> Arc<Self> {
        Arc::new(UserUsecase { admin_repo, jwt })
    }
}

#[async_trait]
impl domain::IUserUC for UserUsecase {
    async fn list_user(&self, _req: ListUserInput) -> BizResult<ListUserOutput> {
        let output = ListUserOutput {
            total: 0,
            list: vec![],
        };

        Ok(output)
    }

    async fn login(&self, req: LoginInput) -> BizResult<String> {
        let user = self
            .admin_repo
            .user_repo()
            .get_user(req.username, req.user_id)
            .await?;

        let check = crypt::verify_password(
            user.password.unwrap_or_default().as_str(),
            req.password.as_str(),
        )?;
        if !check {
            return Err(BizError::invalid_param("username or password is incorrect"));
        }

        let token = self.jwt.encode(&Principal {
            id: user.user_id.unwrap_or_default().to_string(),
            username: user.username.unwrap_or_default().to_string(),
        })?;
        Ok(token)
    }
}
