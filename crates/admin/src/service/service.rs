use api::admin::v1::req::*;

use crate::domain::uc::IAdminUc;

pub struct AdminService {
    uc: Box<dyn IAdminUc>,
}

impl AdminService {
    pub fn new(uc: Box<dyn IAdminUc>) -> Self {
        Self { uc }
    }

    pub fn say_hello(&self, req: HelloReq) -> HelloResp {
        self.uc.say_hello(req)
    }
}
