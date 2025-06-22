use validator::Validate;
use infra::entity::t_blog_user;

#[derive(Debug, Clone, Validate)]
pub struct ListUserInput{
    pub user_id:  Option<String>,
    pub user_name:  Option<String>,

    pub pagination: common::param::Pagination,
}

#[derive(Debug, Clone)]
pub struct ListUserOutput{
    pub total: i64,
    pub list: Vec<t_blog_user::Model>,
    
}