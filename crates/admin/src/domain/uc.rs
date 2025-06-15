use api::admin::v1::req::*;

pub trait IAdminUc {
    fn say_hello(&self, req: HelloReq) -> HelloResp;
}
