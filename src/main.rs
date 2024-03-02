mod redis_protocol;
mod redis_server;
mod redis_replica;
use std::env;
use std::io::Write;
use std::net::TcpListener;

use std::thread::spawn;
use std::collections::HashMap;
use std::time::SystemTime;
use redis_replica::handle_replica;
use redis_server::handle_client;
use tokio::stream;

use crate::redis_protocol::send_handshake_ping;




fn main() {
    // we define a key-value data structure to store and retrieve the items (SET-GET) => we use a hashmap
    let  data_store : HashMap<String, (String, SystemTime)> = HashMap::new();  
    let args = env::args().collect::<Vec<String>>();
    let default_port = 6379;
    let mut port : String = default_port.to_string();


    // master & slave part
    let mut is_master = true;

    if let Some(index) = args.iter().position(|arg| arg == "--replicaof") {
        is_master = false; // since it's replicaof, then it won't be the master
        
    }

    if let Some(index) = args.iter().position(|arg| arg == "--port") {
        if let Some(port_str) = args.get(index + 1) {
            if let Ok(p) = port_str.parse::<u16>() {
                port = p.to_string();
            } else {
                eprintln!("Invalid port number provided, using default port {}", default_port);
            }
        }
    }
 
    // Create a TCP listener and bind it to the address
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    
    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            
            // If everything goes well, print the below message
            Ok(mut _stream) => {
                let data_store_clone = data_store.clone();
                let stream_clone = _stream.try_clone().expect("Failed to clone stream");
                // Here we should process the stream
                spawn(move || {
                    handle_client(_stream, data_store_clone, is_master);
                    handle_replica(stream_clone, is_master);
                });
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


