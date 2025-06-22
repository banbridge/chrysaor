mod thread_local;
mod metainfo;

pub use thread_local::*;
pub use metainfo::*;

pub type LogId = faststr::FastStr;