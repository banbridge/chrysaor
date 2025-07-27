use api::admin::PageReq;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    #[validate(range(min = 1))]
    pub page_num: i64,
    #[validate(range(min = 1, max = 200))]
    pub page_size: i64,
}
