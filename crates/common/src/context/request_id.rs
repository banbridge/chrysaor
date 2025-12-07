use metainfo::{METAINFO, MetaInfo};

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub const REQUEST_ID_TRACE_KEY: &str = "request_id";

pub type LogId = faststr::FastStr;

pub trait BanbridgeMetainfo {
    fn get_logid(&self) -> Option<LogId>;
    fn set_logid(&mut self, s: LogId);
}

impl BanbridgeMetainfo for MetaInfo {
    fn get_logid(&self) -> Option<LogId> {
        self.get_faststr::<LogId>().cloned()
    }
    fn set_logid(&mut self, s: LogId) {
        self.insert_faststr::<LogId>(s)
    }
}

pub fn get_logid() -> Option<LogId> {
    METAINFO.with(|v| v.borrow().get_logid())
}

pub fn get_or_defalue_logid() -> LogId {
    get_logid().unwrap_or(String::from("-").parse().unwrap())
}

#[cfg(test)]
mod tests {
    use metainfo::MetaInfo;

    use super::*;

    #[test]
    fn test_log_id() {
        let log_id = crate::id_gen::gen_log_id();

        let mut metainfo = MetaInfo::new();

        metainfo.set_logid(log_id);

        let get_log_id = metainfo.get_logid().unwrap_or_default();

        println!("log_id: {}", get_log_id)
    }
}
