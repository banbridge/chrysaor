use metainfo::METAINFO;
use crate::context::{BanbridgeMetainfo, LogId};

pub fn get_logid() ->Option<LogId>{
    METAINFO.with(|v|{
        v.borrow().get_logid()
    })
}

pub fn get_or_defalue_logid()->LogId{
    get_logid().unwrap_or(String::from("-").parse().unwrap())
}