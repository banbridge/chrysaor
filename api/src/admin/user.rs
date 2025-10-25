use crate::proto_gen::v1::*;
use validator::{Validate, ValidationError, ValidationErrors};

impl Validate for LoginReqDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.password.is_empty() {
            errors.add("Password", ValidationError::new("can not be empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
