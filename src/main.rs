mod redis_protocol;
mod redis_server;
mod redis_replica;
use std::env;
use std::net::{SocketAddr, TcpListener,IpAddr, Ipv4Addr};


use std::thread::spawn;
use std::collections::HashMap;
use std::time::SystemTime;
use redis_protocol::handshake;
use redis_server::handle_client;




#[tokio::main]
async fn main() {
    // we define a key-value data structure to store and retrieve the items (SET-GET) => we use a hashmap
    let  data_store : HashMap<String, (String, SystemTime)> = HashMap::new();  
    let args = env::args().collect::<Vec<String>>();
    let default_port = 6379;
    let mut port : String = default_port.to_string();
    let mut master_port : u16 = default_port;


    // master & slave part
    let mut is_master = true;

    if let Some(index) = args.iter().position(|arg| arg == "--replicaof") {
        is_master = false; // since it's replicaof, then it won't be the master
        if !is_master {
            master_port = args.get(index + 2).unwrap().parse::<u16>().unwrap();
         } else {
            println!("Invalid replicaof command, using default port {}", default_port);
        }
        
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
    if !is_master {
        handshake(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), master_port)).await;
     }
 
    // Create a TCP listener and bind it to the address
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    
    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            
            // If everything goes well, print the below message
            Ok(mut _stream) => {
                let data_store_clone = data_store.clone();
                // Here we should process the stream
                spawn(move || {
                    handle_client(_stream, data_store_clone, is_master)
                });

               
               
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


