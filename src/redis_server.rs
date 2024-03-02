use crate::redis_protocol;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;
use std::time::SystemTime;
use std::collections::HashMap;
use redis_protocol::{send_bulk_string,send_simple_string,send_null_bulk_string,send_handshake_ping};


pub fn handle_client(mut _stream: TcpStream, mut data_store: HashMap<String, (String, SystemTime)>,is_master :bool ) {

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
                        //let res = format!("{}{}", "+PONG", separator); // res is +PONG\r\n
                        let res = redis_protocol::send_simple_string("PONG");
                        println!("ping command response: {:?}", res);
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write respnse");
                    }
                    "echo" => {
                        // let res = format!(
                        //     "{}{}{}{}",
                        //     command_raw_vec[3], separator, command_raw_vec[4], separator
                        // ); // res is raw[3]/r/nraw[4]/r/n which is 5Hello\r\n
                       let res = redis_protocol::send_bulk_string(command_raw_vec[4].to_string());

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
                        let res = send_simple_string("OK"); // res is +OK\r\n
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
                                    send_null_bulk_string()
                                } else {
                                    send_bulk_string(value.to_string())
                                }
                            }
                            None => send_null_bulk_string(),
                        };
                        _stream.write_all(res.as_bytes()).expect("Failed to write response");
                    }
                    "info" => {
                        let role = if is_master { "master" } else { "slave" };
                        let response = send_bulk_string(
                            [
                                format!("role:{}", role),
                                "master_replid:8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb".to_string(),
                                "master_repl_offset:0".to_string(),
                            ]
                            .join("\r\n"),
                        );
                        _stream.write_all(response.as_bytes()).expect("Failed to write response");
                    }
                    
                   
                    

                    _ => {
                        if !is_master {
                            let res = send_handshake_ping();
                            _stream
                                .write_all(res.as_bytes())
                                .expect("Failed to write respnse");
                        } else {
                            let res = send_simple_string("Undefined command");
                            _stream
                                .write_all(res.as_bytes())
                                .expect("Failed to write respnse");
                        
                        }
                        println!("Undefined command");
                    }
                }
           
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

       
}


