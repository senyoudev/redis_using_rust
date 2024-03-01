use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
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
    loop {
        let mut buffer = [0u8;512];
        match _stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection Closed");
                break;
            }
            Ok(_) => {
                // Here the magic should be done
                let command = String::from_utf8_lossy(&buffer);
                if let Some(command_redis) = parse_redis_command(&command) {
                    println!("Command parsed");
                    execute_redis_command(command_redis, &mut _stream);
                } else {
                    println!("Invalid Command");

                }
                _stream.write_all("+PONG\r\n".as_bytes()).unwrap();
                println!("PONG sent");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

       
}


fn parse_redis_command(command_str: &str) -> Option<RedisCommand> {
    if let Some(index) = command_str.find('\n') {
        let command = &command_str[1..index].to_lowercase(); // Convert to lowercase for case insensitivity
        let command_string = command.to_string();
        match command_string.as_str() {
            "echo" => Some(RedisCommand::Echo),
            "ping" => Some(RedisCommand::Ping),
            // Add more commands as needed
            _ => None,
        }
    } else {
        None
    }
}

fn execute_redis_command(command:RedisCommand, stream: &mut TcpStream)  {
    match command {
        RedisCommand::Echo => {
            println!("Echo command parsed");
            // Handle Echo command logic here
            stream.write_all(b"+PONG\r\n").unwrap(); // Placeholder response
        }
        RedisCommand::Ping => {
            println!("Ping command parsed");
            // Handle Ping command logic here
            stream.write_all(b"+mango\r\n").unwrap(); // Placeholder response
        }
        RedisCommand::Command => {
            println!("Command command parsed");
            // Handle Command command logic here
            stream.write_all(b"+PONG\r\n").unwrap(); // Placeholder response
        }
        // Add more variants as needed
    }
}

