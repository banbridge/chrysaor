use api::admin_gen::base::PageReqDto;
use api::admin_gen::v1::{ListUserReqDto, ListUserResultDto, LoginReqDto, UserDto};
use faststr::FastStr;
use infra::entity::t_blog_user;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct ListUserInput {
    pub user_id: Option<FastStr>,
    pub username: Option<FastStr>,

    pub pagination: common::param::Pagination,
}

#[derive(Debug, Clone)]
pub struct ListUserOutput {
    pub total: i64,
    pub list: Vec<t_blog_user::Model>,
}

pub struct LoginInput {
    pub username: Option<FastStr>,
    pub user_id: Option<FastStr>,
    pub password: FastStr,
}

impl Validate for LoginInput {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();
        if self.username.is_none() && self.user_id.is_none() {
            errors.add(
                "username or user_id",
                validator::ValidationError::new("username or user_id can not be empty same"),
            )
        }
        if self.password.is_empty() {
            errors.add(
                "password",
                validator::ValidationError::new("password can not be empty"),
            )
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}

impl From<LoginReqDto> for LoginInput {
    fn from(value: LoginReqDto) -> Self {
        Self {
            username: value.username.map(|v| FastStr::from(v)),
            user_id: value.user_id.map(|v| FastStr::from(v)),
            password: value.password.into(),
        }
    }
}

impl From<ListUserReqDto> for ListUserInput {
    fn from(value: ListUserReqDto) -> Self {
        let page = value.page.unwrap_or(PageReqDto {
            page_num: 1,
            page_size: 10,
        });
        Self {
            user_id: value.user_id,
            username: value.username,
            pagination: common::param::Pagination {
                page_num: page.page_num,
                page_size: page.page_num,
            },
        }
    }
}

impl From<ListUserOutput> for ListUserResultDto {
    fn from(value: ListUserOutput) -> Self {
        Self {
            total: value.total,
            row: value
                .list
                .into_iter()
                .map(|item| convert_user_model_to_user_result(item))
                .collect::<Vec<_>>(),
        }
    }
}

pub fn convert_user_model_to_user_result(data: t_blog_user::Model) -> UserDto {
    UserDto {
        user_id: data.user_id.unwrap_or_default(),
        username: data.username.unwrap_or_default(),
        email: data.email.unwrap_or_default(),
        phone: "****".into(),
        age: 0,
    }
}
