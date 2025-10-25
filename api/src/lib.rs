mod admin;

include!(concat!(env!("OUT_DIR"), "/proto_gen.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let req = proto_gen::v1::UserDto {
            username: "test".into(),
            user_id: "1".into(),
            age: 18,
            email: "test@test.com".into(),
            phone: "13800000000".into(),
        };

        let req_str = serde_json::to_string(&req).unwrap();
        println!("{:?}", req);
        println!("json data: {:?}", req_str);

        // let req_builder = proto_gen::v1::ListUserReqDtoBuilder::default()
        //     .build()
        //     .unwrap();
        // println!("{:?}", req_builder);
    }
}
