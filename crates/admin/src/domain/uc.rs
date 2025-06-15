use crate::obj::{ListUserReq, ListUserResp};

pub trait IAdminUC: Send + Sync {
    fn get_user_uc(&self) -> &dyn IUserUC;
}

pub trait IUserUC: Send + Sync {
    fn list_user(&self, req: ListUserReq) -> ListUserResp;
}
