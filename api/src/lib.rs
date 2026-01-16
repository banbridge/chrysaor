// @generated
pub mod admin {
    #[cfg(feature = "admin-v1")]
    // @@protoc_insertion_point(attribute:admin.v1)
    pub mod v1 {
        include!("admin/v1/admin.v1.rs");
        // @@protoc_insertion_point(admin.v1)
    }
}
#[cfg(feature = "base")]
// @@protoc_insertion_point(attribute:base)
pub mod base {
    include!("base/base.rs");
    // @@protoc_insertion_point(base)
}
pub mod buf {
    #[cfg(feature = "buf-validate")]
    // @@protoc_insertion_point(attribute:buf.validate)
    pub mod validate {
        include!("buf/validate/buf.validate.rs");
        // @@protoc_insertion_point(buf.validate)
    }
}