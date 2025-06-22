use metainfo::MetaInfo;

pub const LOG_ID_HEADER: &str = "x-log-id";


pub trait BanbridgeMetainfo{
    fn get_logid(&self)->Option<super::LogId>;
    fn set_logid(&mut self, s: super::LogId);
}

impl BanbridgeMetainfo for MetaInfo{
    fn get_logid(&self)->Option<super::LogId>{
        self.get_faststr::<super::LogId>().cloned().map(super::LogId::from) 
    }
    fn set_logid(&mut self, s: super::LogId){
        self.insert_faststr::<super::LogId>(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_id(){
        let log_id = crate::id_gen::gen_log_id();
        
        let mut metainfo = MetaInfo::new();
        
        metainfo.set_logid(log_id);
        
        let get_log_id = metainfo.get_logid().unwrap_or_default();
        
        println!("log_id: {}", get_log_id)
    }
}