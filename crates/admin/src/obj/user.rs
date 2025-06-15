use common::param;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ListUserReq {
    pub id: Option<String>,
    pub name: Option<String>,

    #[validate(nested)]
    #[serde(flatten)]
    pub page: param::Pagination,
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub email: String,
}

#[derive(Serialize)]
pub struct ListUserResp {
    pub users: Vec<User>,

    pub total: i64,
}

pub struct ListUserOption {
    pub id: String,
    pub name: String,

    pub offset: i32,
    pub limit: i32,
}
