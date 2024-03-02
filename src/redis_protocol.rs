use std::{fmt::format, io::Write, net::TcpStream};

pub fn send_simple_string(response: &str) -> String {
    format!("+{}\r\n", response)
}

pub fn send_bulk_string(response: String) -> String {
    format!("${}\r\n{}\r\n", response.len(), response)
}

pub fn send_null_bulk_string() -> String {
    format!("$-1\r\n")
}

pub fn send_handshake_ping() -> String {
   format!("*1\r\n$4\r\nping\r\n")
}