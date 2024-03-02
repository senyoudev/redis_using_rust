use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time;
use std::thread::spawn;
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;





fn main() {
    // we define a key-value data structure to store and retrieve the items (SET-GET) => we use a hashmap
    let mut data_store : HashMap<String, (String, Option<time::Duration>,Option<SystemTime>)> = HashMap::new();    
 
    // Create a TCP listener and bind it to the address
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            // If everything goes well, print the below message
            Ok(_stream) => {
                let data_store_clone = data_store.clone();
                // Here we should process the stream
                spawn(|| {
                    handle_client(_stream, data_store_clone);
                });
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut _stream: TcpStream, mut data_store: HashMap<String, (String, Option<Duration>,Option<SystemTime>)>) {

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
                        let mut expired_time : Option<Duration> = None;
                        if let Some(index) = expiry_index {
                            if let Ok(expiry) = command_raw_vec[index + 1].parse::<u64>() {
                                expired_time  = Some(Duration::from_millis(expiry));
                            }
                        }
                        let set_time = SystemTime::now();
                        data_store.insert(key.to_string(), (value.to_string(),expired_time,Some(set_time)));
                        let res = format!("{}{}", "+OK", separator); // res is +OK\r\n
                        println!("set command response: {:?}", res);
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write response");
                    }
                    "get" => {
                        // the command will be like : *2\r\n$3\r\nget\r\n$3\r\nkey\r\n so the key will be in position 4
                        let key = command_raw_vec[4];
                        //check if the key exists in the data store and it is not expired


                        if let Some((value,expiry_duration,set_time)) = data_store.get(key) {
                                let now = SystemTime::now();
                                let time_in_which_should_be_expired = set_time.unwrap().duration_since(UNIX_EPOCH).unwrap() + expiry_duration.unwrap();

                                if now.duration_since(UNIX_EPOCH).unwrap() > time_in_which_should_be_expired {
                                    data_store.remove(key);
                                    let res = format!("{}{}", "$-1", separator); // res is $-1\r\n
                                    println!("get command response: {:?}", res);
                                    _stream
                                        .write_all(res.as_bytes())
                                        .expect("Failed to write response");
                                    continue;
                                }
                            
                            
                                let res = format!("${}{}{}{}", value.len(), separator, value, separator); // res is $5\r\nvalue\r\n
                                 println!("get command response: {:?}", res);
                                _stream
                                    .write_all(res.as_bytes())
                                    .expect("Failed to write response");
                            
                            
                        } else {
                            let res = format!("{}{}", "$-1", separator); // res is $-1\r\n
                            println!("get command response: {:?}", res);
                            _stream
                                .write_all(res.as_bytes())
                                .expect("Failed to write response");
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



