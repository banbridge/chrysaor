use banbridge_derive::BizError;
use tracing::warn;

pub type AppResult<T> = Result<T, AppErrorBuilt>;

#[derive(Copy, Clone, Debug, BizError)]
pub enum AppError {
    #[detail(code = 0, http_status = 200)]
    Ok,
    #[detail(code = 1000000, http_status = 500, message_zh = "内部通用错误")]
    Internal,
    #[detail(
        code = 1000001,
        http_status = 400,
        message_zh = "请求参数错误，请仔细核对参数有效性"
    )]
    InvalidParam,
    #[detail(code = 1000002, http_status = 500, message_zh = "未知错误")]
    UnknownAnyhow,
    #[detail(code = 1000003, http_status = 404, message_zh = "请求资源不存在")]
    RequestNotFound,
    #[detail(code = 1000004, http_status = 408, message_zh = "请求超时")]
    RequestTimeout,

    #[detail(
        code = 1010000,
        http_status = 400,
        message_zh = "从请求中解析参数错误，请检查对应参数"
    )]
    ParamBind,
    #[detail(
        code = 1010001,
        http_status = 400,
        message_zh = "JSON解析出错，请检查json格式"
    )]
    JsonParse,
    #[detail(
        code = 1010002,
        http_status = 400,
        message_zh = "JSON序列化错误，请检查对应参数"
    )]
    JsonSerde,
    #[detail(code = 1010003, http_status = 500, message_zh = "BCrypt 加密失败")]
    BcryptFailed,

    // auth 相关错误码
    #[detail(code = 1020000, http_status = 401, message_zh = "JWT Token 无效")]
    JwtInvalidToken, // auth 无效
    #[detail(
        code = 1020001,
        http_status = 401,
        message_zh = "用户未登录或Token失效"
    )]
    Unauthenticated, // 未授权
    #[detail(code = 1020002, http_status = 401, message_zh = "Token无效或已过期")]
    JwtDecode,
    #[detail(
        code = 1020003,
        http_status = 401,
        message_zh = "Token编码失败，请检查对应参数"
    )]
    JwtEncode = 1020004,

    // 数据库相关错误吗
    #[detail(code = 1030000, http_status = 500, message_zh = "数据库错误")]
    DBCommon, // 数据库错误
    #[detail(code = 1030001, http_status = 404, message_zh = "数据库记录未找到")]
    DBNotFound, // 数据库未找到
    #[detail(code = 1030002, http_status = 500, message_zh = "数据库查询失败")]
    DBQueryFailed, // 数据库查询失败
    #[detail(code = 1030003, http_status = 500, message_zh = "数据库更新失败")]
    DBUpdateFailed, // 数据库更新失败
    #[detail(code = 1030004, http_status = 500, message_zh = "数据库插入失败")]
    DBInsertFailed, // 插入数据库失败
    #[detail(code = 1030005, http_status = 500, message_zh = "数据库删除失败")]
    DBDeleteFailed, // 删除数据库失败
    #[detail(code = 1030006, http_status = 500, message_zh = "数据库连接失败")]
    DBConnectionFailed,
    #[detail(code = 1030007, http_status = 500, message_zh = "数据库事务开启")]
    DBTransactionBeginFailed,
    #[detail(code = 1030008, http_status = 500, message_zh = "数据库事务提交失败")]
    DBTransactionCommitFailed,
}

impl AppErrorBuilt {
    pub fn print_stack(self) -> Self {
        let stack = backtrace::Backtrace::new();
        warn!("{:#?}", stack);
        self
    }
}
