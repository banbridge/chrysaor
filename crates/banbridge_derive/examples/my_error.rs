use banbridge_derive::BizError;

#[derive(BizError, Debug)]
enum BanbridgeError {
    #[detail(code = 1234, http_status = 400, message_zh = "邮箱格式错误")]
    InvalidEmail,
    #[detail(code = 1235, http_status = 400)]
    InvalidPassword,
}

fn main() {
    let code = BanbridgeError::InvalidEmail.get_code();
    println!("{}", code);
    let c = BanbridgeErrorBizBuilder::invalid_email("sa".to_string());

    println!("{:#?}", c);
}

fn print_stack() {
    let a = backtrace::Backtrace::new();
    println!("{:?}", a);
}
