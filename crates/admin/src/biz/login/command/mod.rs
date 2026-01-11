mod model;
mod service;

pub use model::*;

use common::jwt::JWTManager;
use std::sync::Arc;

#[rudi::Singleton(async, name="login_command_service", binds=[Self::into_login_command_service])]
#[derive(Clone)]
pub struct LoginCommandService {}

impl LoginCommandService {
    pub fn into_login_command_service(self) -> Arc<Self> {
        Arc::new(self)
    }
}
