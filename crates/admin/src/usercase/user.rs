use crate::domain;
use crate::obj::{ListUserReq, ListUserResp};

pub struct UserUsecase {}

impl domain::IUserUC for UserUsecase {
    fn list_user(&self, _req: ListUserReq) -> ListUserResp {
        ListUserResp {
            users: vec![],
            total: 10,
        }
    }
}
