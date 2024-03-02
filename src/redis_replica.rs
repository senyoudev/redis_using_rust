use std::net::TcpStream;

use crate::redis_protocol;

use redis_protocol::send_handshake_ping;

pub fn handle_replica(mut _stream: TcpStream, is_master: bool) {
    // Perform the handshake process
    if !is_master {
        send_handshake_ping(&_stream);
    }
}