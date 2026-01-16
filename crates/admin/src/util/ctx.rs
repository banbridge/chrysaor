use super::User;
use faststr::FastStr;
use getset::{Getters, Setters};

#[derive(Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub", set_with = "pub")]
pub struct Ctx {
    user_id: FastStr,
}

impl From<User> for Ctx {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id().clone(),
        }
    }
}
