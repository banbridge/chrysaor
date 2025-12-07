use faststr::FastStr;

#[derive(Debug)]
pub struct Ctx {
    pub user_id: FastStr,
    pub log_id: FastStr,
}
