use crate::error::{BizError, BizResult};
use faststr::FastStr;
use std::str::FromStr;

// 判断字符串是否为空
pub fn require_non_empty<T>(s: &T, err_msg: &str) -> BizResult<()>
where
    T: AsRef<str>,
{
    if s.as_ref().is_empty() {
        return Err(BizError::invalid_param(
            FastStr::from_str(err_msg).unwrap_or(FastStr::from_str("param is empty").unwrap()),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_non_empty() {
        let s = "123";
        require_non_empty(&s, "s is empty".into()).unwrap();

        let s1 = String::from("1231");
        require_non_empty(&s1, "is not empty".into()).unwrap();
    }
}
