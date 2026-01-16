use crate::error::AppErrorBuilt;
use derive_builder::UninitializedFieldError;

impl From<UninitializedFieldError> for AppErrorBuilt {
    fn from(value: UninitializedFieldError) -> Self {
        AppErrorBuilt::uninitialized_field_error(format!("some field uninitialized {}", value))
    }
}
