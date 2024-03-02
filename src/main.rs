use std::env;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread::spawn;
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;




fn main() {
    // we define a key-value data structure to store and retrieve the items (SET-GET) => we use a hashmap
    let  data_store : HashMap<String, (String, SystemTime)> = HashMap::new();  
    let args = env::args().collect::<Vec<String>>();
    let default_port = 6379;
    let mut port : String = default_port.to_string();
    let mut master_host = String::new();
    let mut master_port = String::new();

    // master & slave part
    let mut is_master = true;

    if let Some(index) = args.iter().position(|arg| arg == "--replicaof") {
        is_master = false; // since it's replicaof, then it won't be the master
        if let Some(host) = args.get(index + 1) {
            master_host = host.to_string();
        }
        if let Some(port) = args.get(index + 2) {
            master_port = port.to_string();
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
 
    // Create a TCP listener and bind it to the address
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    
    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            // If everything goes well, print the below message
            Ok(_stream) => {
                let data_store_clone = data_store.clone();
                // Here we should process the stream
                spawn(move || {
                    handle_client(_stream, data_store_clone,is_master);
                });
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut _stream: TcpStream, mut data_store: HashMap<String, (String, SystemTime)>,is_master :bool ) {

    // now we implement a proper redis protocol

    // read the command from the client
    let mut buffer = [0u8;512];
    let separator = "\r\n";
    loop {
        match _stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection Closed");
                break;
            }
            Ok(_) => {
                // Here the magic should be done
                let command = String::from_utf8_lossy(&buffer);
                let command_str = command.to_string();
                let command_raw_vec : Vec<&str> = command_str.split(separator).collect();
                let command_to_be_passed = command_raw_vec[2]; //  why 2 ? because the format of the command is "*3\r\n$4\r\nECHO\r\n$5\r\nHello\r\n"
                // and the first element is "*3" and the second is "$4" and the third is the command
                match command_to_be_passed {
                    "ping" => {
                        let res = format!("{}{}", "+PONG", separator); // res is +PONG\r\n
                        println!("ping command response: {:?}", res);
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write respnse");
                    }
                    "echo" => {
                        let res = format!(
                            "{}{}{}{}",
                            command_raw_vec[3], separator, command_raw_vec[4], separator
                        ); // res is raw[3]/r/nraw[4]/r/n which is 5Hello\r\n
                        println!("echo command respnse: {:?}", res);
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write respnse");
                    }
                    "set" => {
                        // the command will be like : *3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n so the key will be in position 4 and the value will be in position 6
                        // normally we have the argument px about expiry so if the user specifies the expiry, the command will be as *3\r\n$3\r\nset\r\n$3\r\nkey\r\n$5\r\nvalue\r\n$2\r\npx\r\n$3\r\n100\r\n
                        let key = command_raw_vec[4];
                        let value = command_raw_vec[6];
                        let expiry_index = command_raw_vec.iter().position(|&x| x == "px");
                        if let Some(index) = expiry_index {
                            println!("The expiry index is: {:?}", index);
                            println!("The expiry is: {:?}", command_raw_vec[index + 2]);
                            let expiry = command_raw_vec[index + 2].parse::<u64>().unwrap();
                            println!("The expiry is: {:?}", expiry);
                            let expiry_duration = Duration::from_millis(expiry);
                            let expiration_time = SystemTime::now() + expiry_duration;
                            data_store.insert(key.to_string(), (value.to_string(),expiration_time));
                            
                            
                        } 
                        else {
                            // No expiry provided, set expiration to max
                            let expiration_time = SystemTime::now() + Duration::from_secs(365 * 24 * 60 * 60); // 1 year from now
                            data_store.insert(key.to_string(), (value.to_string(), expiration_time));
                           
                       }
                        let res = format!("{}{}", "+OK", separator); // res is +OK\r\n
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write response");
                        
                    }
                    "get" => {
                        // the command will be like : *2\r\n$3\r\nget\r\n$3\r\nkey\r\n so the key will be in position 4
                        let key = command_raw_vec[4];
                        // retrieve the result of the key
                        let res = match data_store.get(&key.to_string()) {
                            
                            Some((value, expiration_time)) => {
                                if SystemTime::now() > *expiration_time {
                                    data_store.remove(&key.to_string());
                                    format!("{}{}", "$-1", separator)
                                } else {
                                    format!("${}{}{}{}", value.len(), separator, value, separator)
                                }
                            }
                            None => format!("{}{}", "$-1", separator),
                        };
                        _stream.write_all(res.as_bytes()).expect("Failed to write response");
                    }
                    "info" => {
                        // As a first observation, I think we will find replication in position 3
                        if is_master {
                            let res = format!(
                                "{}{}{}:{}{}{}:{}{}{}{}{}",
                                "$11", separator, "role", "master", separator,
                                "master_replid", "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb", separator,
                                "master_repl_offset", "0",separator
                            );
                            _stream.write_all(res.as_bytes()).expect("Failed to write response");
                        } else {
                            let res = format!(
                                "{}{}{}:{}{}{}:{}{}",
                                "$10", separator, "role", "slave", separator,
                                "master_repl_offset", "0", separator
                            );
                            _stream.write_all(res.as_bytes()).expect("Failed to write response");
                        } 
                    
                    }

                    _ => {
                        println!("Undefined command");
                    }
                }
                println!("PONG sent");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

       
}



