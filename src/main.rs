use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::string;
use std::thread::spawn;

// an enum for commands 
pub enum RedisCommand {
    Command,
    Echo,
    Ping
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Create a TCP listener and bind it to the address
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            // If everything goes well, print the below message
            Ok(_stream) => {
                // Here we should process the stream
                spawn(|| {
                    handle_client(_stream)
                });
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut _stream: TcpStream) {

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
                        let res = format!("{}{}", "+PONG", separator);
                        println!("ping command response: {:?}", res);
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write respnse");
                    }
                    "echo" => {
                        let res = format!(
                            "{}{}{}{}",
                            command_raw_vec[3], separator, command_raw_vec[4], separator
                        );
                        println!("echo command respnse: {:?}", res);
                        _stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write respnse");
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



