#[derive(Clone)]
pub struct Endpoint {
    ip: String,
    port: i32,
    str_: String,
}

impl Endpoint {
    fn get_ip(&self) -> &String {
        &self.ip
    }

    fn get_port(&self) -> i32 {
        self.port
    }
}