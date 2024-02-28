use std::net::TcpListener;

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
                println!("accepted new connection");
            }
            // If there is an error, print the error message
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
