use crate::error::{BizError, BizResult};

pub fn crypt_password(password: &str) -> BizResult<String> {
    let hash_paas = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|err| BizError::bcrypt_failed(err.to_string().into()))?;
    Ok(hash_paas)
}

pub fn verify_password(hash_password: &str, password: &str) -> BizResult<bool> {
    bcrypt::verify(password, hash_password)
        .map_err(|err| BizError::bcrypt_failed(err.to_string().into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "123456";
        let hash_password = "$2b$12$ruJjTvVQzE7m.bPDJFmwfeeTEXdeZZk6ZYFRbyRnT3YZH41XBnGEG";
        println!("hash_password: {}", hash_password);
        let result = verify_password(&hash_password, password).unwrap();
        println!("result: {}", result);
    }
}
