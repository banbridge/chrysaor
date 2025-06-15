use crate::domain::uc::IAdminUc;
use api::admin::v1::req::*;

pub struct AdminUsercase {}

impl AdminUsercase {
    pub fn new() -> Self {
        AdminUsercase {}
    }
}

impl IAdminUc for AdminUsercase {
    fn say_hello(&self, req: HelloReq) -> HelloResp {
        HelloResp {
            msg: "hello".to_string(),
            code: 0,
            data: "uc".to_string(),
        }
    }
}
