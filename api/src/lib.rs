pub mod admin;

#[cfg(test)]
mod tests {
    use prost_validate::{Validator};
    use super::*;

    #[test]
    fn it_works() {
        let req = admin::admin_v1::ListUserReq{
            username: None,
            user_id: None,
            page: None, 
        } ;
        
        let result = req.validate();

        println!("{:?}", result)
    }
}