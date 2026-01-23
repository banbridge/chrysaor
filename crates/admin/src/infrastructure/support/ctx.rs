use super::UserClaim;
use faststr::FastStr;
use getset::{Getters, Setters};

#[derive(Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub", set_with = "pub")]
pub struct Ctx {
    user_id: FastStr,
}

impl From<UserClaim> for Ctx {
    fn from(user: UserClaim) -> Self {
        Self {
            user_id: user.user_id().clone(),
        }
    }
}
