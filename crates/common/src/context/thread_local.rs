use crate::context::{BanbridgeMetainfo, LogId};
use metainfo::METAINFO;

#[allow(unused)]
const LOG_ID_KEY: &str = "B_LOG_ID";

pub fn get_log_id() -> Option<LogId> {
    METAINFO
        .try_with(|tmp| tmp.borrow().get_logid())
        .unwrap_or(None)
}

pub fn get_or_default_log_id() -> LogId {
    get_log_id().unwrap_or_default()
}
