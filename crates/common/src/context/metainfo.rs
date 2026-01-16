use metainfo::MetaInfo;

#[allow(dead_code)]
pub trait ChrysaorMetaInfo {
    fn get_log_id(&self) -> Option<super::LogId>;
    fn set_log_id(&mut self, log_id: super::LogId);
}

impl ChrysaorMetaInfo for MetaInfo {
    #[inline]
    fn get_log_id(&self) -> Option<super::LogId> {
        self.get_faststr::<super::LogId>().map(|l| l.to_owned())
        // .cloned()
        // .map(super::LogId::from)
    }

    #[inline]
    fn set_log_id(&mut self, log_id: super::LogId) {
        self.insert_faststr::<super::LogId>(log_id)
    }
}
