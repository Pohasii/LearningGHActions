use std::net::SocketAddr;
use std::time::SystemTime;

#[derive(Copy, Clone)]
pub struct Client {
    pub addr: SocketAddr,
    pub date_of_last_message: SystemTime,
    pub status: bool,
    pub id: i32,
}

pub fn new(addr: SocketAddr, id: i32) -> Client {
    return Client {
        addr,
        date_of_last_message: SystemTime::now(),
        status: true,
        id,
    };
}
