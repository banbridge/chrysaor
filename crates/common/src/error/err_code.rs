use crate::error::BizError;

#[derive(Copy, Clone)]
pub enum BizCode {
    Ok = 0,

    Internal = 1000000,
    InvalidParam = 1000001,
    UnknownAnyhow = 1000002,

    ParamBind = 1010001,
    JsonParse = 1010002,
    JsonSerde = 1010003,

    // auth 相关错误码
    JwtInvalidToken = 1020001, // auth 无效
    Unauthenticated = 1020002, // 未授权
    JwtDecode = 1020003,
    JwtEncode = 1020004,
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

            BizCode::JwtInvalidToken => 401,
            BizCode::Unauthenticated => 403,
            BizCode::JwtDecode => 401,
            BizCode::JwtEncode => 401,
        }
    }
}

impl BizError {
    pub fn internal(msg: &str) -> Self {
        let biz_code = BizCode::Internal;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn invalid_param(msg: &str) -> Self {
        let biz_code = BizCode::InvalidParam;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn param_bind(msg: &str) -> Self {
        let biz_code = BizCode::ParamBind;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn json_parse(msg: &str) -> Self {
        let biz_code = BizCode::JsonParse;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn json_serde(msg: &str) -> Self {
        let biz_code = BizCode::JsonSerde;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn jwt_invalid_token(msg: &str) -> Self {
        let biz_code = BizCode::JwtInvalidToken;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn unauthenticated(msg: &str) -> Self {
        let biz_code = BizCode::Unauthenticated;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn unknown_anyhow(err: anyhow::Error) -> Self {
        let biz_code = BizCode::UnknownAnyhow;
        BizError::new(
            biz_code.status_code(),
            format!("{:?}", err).as_str(),
            biz_code as u32,
        )
    }

    pub fn jwt_encode(msg: &str) -> Self {
        let biz_code = BizCode::JwtEncode;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }

    pub fn jwt_decode(msg: &str) -> Self {
        let biz_code = BizCode::JwtDecode;
        BizError::new(biz_code.status_code(), msg, biz_code as u32)
    }
}
