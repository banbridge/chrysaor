use std::fmt::{Display, Formatter};

use crate::error::BizError;

#[derive(Copy, Clone, Debug)]
pub enum BizCode {
    Ok = 0,

    Internal = 1000000,
    InvalidParam = 1000001,
    UnknownAnyhow = 1000002,

    ParamBind = 1010001,
    JsonParse = 1010002,
    JsonSerde = 1010003,
    BcryptFailed = 1010004,

    // auth 相关错误码
    JwtInvalidToken = 1020001, // auth 无效
    Unauthenticated = 1020002, // 未授权
    JwtDecode = 1020003,
    JwtEncode = 1020004,

    // 数据库相关错误吗
    DBCommon = 1030000,       // 数据库错误
    DBNotFound = 1030001,     // 数据库未找到
    DBQueryFailed = 1030002,  // 数据库查询失败
    DBUpdateFailed = 1030003, // 数据库更新失败
    DBInsertFailed = 1030004, // 插入数据库失败
    DBDeleteFailed = 1030005, // 删除数据库失败
}

impl BizCode {
    pub fn status_code(&self) -> u16 {
        match self {
            BizCode::Ok => 200,
            BizCode::Internal => 500,
            BizCode::UnknownAnyhow => 400,

            BizCode::InvalidParam => 400,
            BizCode::ParamBind => 400,
            BizCode::JsonParse => 400,
            BizCode::JsonSerde => 400,
            BizCode::BcryptFailed => 401,

            BizCode::JwtInvalidToken => 401,
            BizCode::Unauthenticated => 403,
            BizCode::JwtDecode => 401,
            BizCode::JwtEncode => 401,

            BizCode::DBCommon => 500,
            BizCode::DBNotFound => 404,
            BizCode::DBQueryFailed => 500,
            BizCode::DBUpdateFailed => 500,
            BizCode::DBInsertFailed => 500,
            BizCode::DBDeleteFailed => 500,
        }
    }
}

impl Display for BizCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BizError {
    pub fn internal(msg: &str) -> Self {
        let biz_code = BizCode::Internal;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn invalid_param(msg: &str) -> Self {
        let biz_code = BizCode::InvalidParam;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn param_bind(msg: &str) -> Self {
        let biz_code = BizCode::ParamBind;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn json_parse(msg: &str) -> Self {
        let biz_code = BizCode::JsonParse;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn json_serde(msg: &str) -> Self {
        let biz_code = BizCode::JsonSerde;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn jwt_invalid_token(msg: &str) -> Self {
        let biz_code = BizCode::JwtInvalidToken;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn bcrypt_failed(msg: &str) -> Self {
        let biz_code = BizCode::BcryptFailed;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn unauthenticated(msg: &str) -> Self {
        let biz_code = BizCode::Unauthenticated;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn unknown_anyhow(err: anyhow::Error) -> Self {
        let biz_code = BizCode::UnknownAnyhow;
        BizError::new(
            biz_code.status_code(),
            format!("{:?}", err).as_str(),
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn jwt_encode(msg: &str) -> Self {
        let biz_code = BizCode::JwtEncode;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn jwt_decode(msg: &str) -> Self {
        let biz_code = BizCode::JwtDecode;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn db_common(msg: &str) -> Self {
        let biz_code = BizCode::DBCommon;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn db_not_found(msg: &str) -> Self {
        let biz_code = BizCode::DBNotFound;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn db_query_failed(msg: &str) -> Self {
        let biz_code = BizCode::DBQueryFailed;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn db_insert_failed(msg: &str) -> Self {
        let biz_code = BizCode::DBInsertFailed;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn db_update_failed(msg: &str) -> Self {
        let biz_code = BizCode::DBUpdateFailed;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }

    pub fn db_delete_failed(msg: &str) -> Self {
        let biz_code = BizCode::DBDeleteFailed;
        BizError::new(
            biz_code.status_code(),
            msg,
            biz_code as u32,
            biz_code.to_string(),
        )
    }
}
