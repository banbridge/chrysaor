use super::{ListUserInput, ListUserOutput, LoginInput};
use crate::data::AdminRepo;
use crate::domain::{self, IUserUC};
use crate::util::jwt::{JWT, Principal};
use async_trait::async_trait;
use common::crypt;
use common::error::{BizError, BizResult};
use faststr::FastStr;
use std::sync::Arc;

#[derive(Clone)]
#[rudi::Singleton(async,name = "user_uc", binds=[Self::into_user_uc])]
pub struct UserUsecase {
    #[di(name = "admin_repo")]
    admin_repo: Arc<AdminRepo>,
    #[di(name = "jwt")]
    jwt: Arc<JWT>,
}

impl UserUsecase {
    fn into_user_uc(self) -> Arc<dyn IUserUC> {
        Arc::new(self)
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

    async fn login(&self, req: LoginInput) -> BizResult<FastStr> {
        let user = self
            .admin_repo
            .user_repo
            .get_user(req.username, req.user_id)
            .await?;

        let check = crypt::verify_password(
            user.password.unwrap_or_default().as_str(),
            req.password.as_str(),
        )?;
        if !check {
            return Err(BizError::invalid_param(FastStr::from(
                "username or password is incorrect",
            )));
        }

        let token = self.jwt.encode(&Principal {
            id: user.user_id.unwrap_or_default().to_string(),
            username: user.username.unwrap_or_default().to_string(),
        })?;
        Ok(token)
    }
}
