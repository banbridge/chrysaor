use std::cell::RefCell;

thread_local! {
pub static LOG_ID: RefCell<Option<super::LogId>> = RefCell::new(None);

pub static METAINFO: RefCell<Option<super::MetaInfo>> = RefCell::new(None);
}
