pub struct HelloReq {
    pub name: String,
    pub age: i32,
}

pub struct HelloResp {
    pub code: i32,
    pub msg: String,
    pub data: String,
}
