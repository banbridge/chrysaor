use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum)]
#[serde(rename_all = "snake_case")]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Gender {
    Male = 1,
    Female = 2,
    Unknown = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gender() {
        println!("{:?}", Gender::Male);
    }
}
