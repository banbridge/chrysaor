use validator::Validate;
use api::admin::v1::LoginReq;
use infra::entity::t_blog_user;

#[derive(Debug, Clone, Validate)]
pub struct ListUserInput{
    pub user_id:  Option<String>,
    pub username:  Option<String>,

    pub pagination: common::param::Pagination,
}

#[derive(Debug, Clone)]
pub struct ListUserOutput{
    pub total: i64,
    pub list: Vec<t_blog_user::Model>,
    
}

pub struct LoginInput{
    pub username: Option<String>,
    pub user_id : Option<String>,
    pub password: String,
}

impl Validate for LoginInput {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();
        if self.username.is_none() && self.user_id.is_none() {
           errors.add("username or user_id", validator::ValidationError::new("username or user_id can not be empty same")) 
        }
        if self.password.is_empty() {
            errors.add("password", validator::ValidationError::new("password can not be empty"))
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}

impl From<LoginReq> for LoginInput{
    fn from(value: LoginReq) -> Self {
        Self{
            username: value.username,
            user_id: value.user_id,
            password: value.password,
            
        }
    }
}