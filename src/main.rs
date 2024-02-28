use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;

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
                handle_client(_stream);
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut _stream: TcpStream) {

        match _stream.write_all(b"+PONG\r\n") {
            Ok(_) => {
                println!("PONG sent");
            }
            Err(e) => {
                println!("error: {}", e);
            }

}
}
