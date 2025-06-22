use std::sync::LazyLock;
use yit_id_generator::{IdGeneratorOptions, YitIdHelper};

struct UniqIdGenerator;

pub static UNIQ_ID: LazyLock<UniqIdGenerator> = LazyLock::new(UniqIdGenerator::new);

pub fn gen_uniq_id() -> String {
    UNIQ_ID.generate()
}

impl UniqIdGenerator {
    pub fn new() -> Self {
        let mut options = IdGeneratorOptions::New(1);
        options.WorkerIdBitLength = 13;
        YitIdHelper::SetIdGenerator(options);
        Self
    }
    
    pub fn generate(&self) -> String {
        YitIdHelper::NextId().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_uniq_id() {
        println!("{}", gen_uniq_id());
    }
}