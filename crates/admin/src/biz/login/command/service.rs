use super::LoginCommand;
use crate::biz::login::LoginCommandService;
use common::error::AppResult;

impl LoginCommandService {
    pub fn login(&self, input: LoginCommand) -> AppResult<String> {
        Ok("".to_string())
    }
}
