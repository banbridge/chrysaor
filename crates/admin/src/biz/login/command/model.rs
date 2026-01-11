use common::error::AppErrorBuilt;
use derive_builder::Builder;
use validator::Validate;

#[derive(Debug, Builder, Validate)]
#[builder(
    build_fn(error = "AppErrorBuilt")  // 关键：指定自定义错误类型
)]
pub struct LoginCommand {
    #[validate(length(min = 1, max = 10))]
    key: String,
    #[validate(length(min = 1))]
    password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::error::AppResult;

    #[test]
    fn test_login_command() {
        let login_command = get_login();

        println!("user: {:?}", login_command);
    }

    fn get_login() -> AppResult<LoginCommand> {
        let rs = LoginCommandBuilder::default()
            .key("key".to_string())
            .password("".to_string())
            .build()?;

        let _ = rs
            .validate()
            .map_err(|e| AppErrorBuilt::validate_param_failed(format!("{}", e)))?;

        Ok(rs)
    }
}
