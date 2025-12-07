use crate::error::AppResult;

// 判断字符串是否为空
pub fn require_non_empty<T>(s: &T, err_msg: &str) -> AppResult<()>
where
    T: AsRef<str>,
{
    if s.as_ref().is_empty() {
        return Err(AppErrorBuilt::invalid_param(err_msg.to_string()));
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
