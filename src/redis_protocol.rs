use crate::redis_replica::Client;
use std::{fmt::format, io::Write, net::{SocketAddr, TcpStream}};




pub fn send_simple_string(response: &str) -> String {
    format!("+{}\r\n", response)
}

pub fn send_bulk_string(response: String) -> String {
    format!("${}\r\n{}\r\n", response.len(), response)
}

pub fn send_null_bulk_string() -> String {
    format!("$-1\r\n")
}


/// Handshake routine.
pub async fn handshake(master: SocketAddr) {
    let mut cli = Client::connect(master).await;
    // Step 1: PING master.
    
        send_simple_string("*1\r\n$4\r\nping\r\n")
;
}