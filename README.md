# Building Redis 

## TCP Overview

### Concepts

- TCP is a widely used protocol for communication between applications. It is a reliable protocol, built on top of IP.
- IP(Internet Protocol) is a connectionless protocol, which means that there is no established connection between the sender and the receiver.
- When a program sends data over IP, the data is broken into packets, each of which is sent individually to the receiver. The receiver then reassembles the packets into the original data.
- Each packet contains : 
      * a header section
      * a data section
- The header contains information about the packet, such as the source and destination IP addresses, the length of the packet, and a checksum to ensure that the packet is not corrupted.
- The data section contains the actual data being sent.
- TCP was created to address the limitations of IP.
- Primarly TCP offers two guarantees:
      * The data will be delivered . It does this by asking the receiver to acknowledge all sent packets, and re-transmitting any packets if an acknowledgement isn't received.
      * The data will be delivered in the order in which it was sent. This is done by numbering each packet, and asking the receiver to acknowledge the packets in order.

### TCP Connections

- TCP is a connection-oriented protocol, which means that a connection must be established between the sender and the receiver before data can be sent.To do this, one program must act as a server, and the other as a client.
- The server listens for incoming connections, and the client initiates a connection to the server.Both the server and the client can send
and receive data(It's a two-way channel).
- A TCP connection is identified using a unique combination of four values:
      * The source IP address
      * The destination IP address
      * The source port
      * The destination port

### TCP Handshake

The TCP handshake is how a connection is established between a client and a server. It is a three-step process:
   -**Step 1 : SYN** :  The client sends a packet with the SYN flag set to the server. This tells the server that the client wants to establish a connection.
   -**Step 2 : SYN-ACK** : The server responds with a packet that has the SYN and ACK flags set. This tells the client that the server has received the request, and is willing to establish a connection.
   -**Step 3 : ACK** : The client sends a packet with the ACK flag set. This tells the server that the client has received the response, and is ready to start sending data.

## TCP Servers in Rust

TCP is the underlying protocol for many networked applications, including web servers, databases, and messaging systems.In this part, we will build a simple TCP server in Rust using ```std::net``` module.

To write a TCP Server, we'll need to be familiar with the following methods:
   - **TcpListener::bind** : This method creates a new TcpListener which will listen for incoming connections on the specified address.
   - **TcpListener::incoming** : This method returns an iterator over the connections received on this listener.
   - **TcpListener::connect** : This method creates a new TcpStream and connects to the specified address.
   - **TcpStream::read** : This method reads data from the stream.
   - **TcpStream::write_all** : This method writes a buffer into the stream.

### The TcpListener struct

The ```TcpListener``` struct is used to listen for incoming TCP connections. It is created by calling the ```bind``` method on the ```TcpListener``` type, and it listens for incoming connections on the specified address.

Here are some methods that are available on the ```TcpListener``` struct:
```rust
impl TcpListener {
    // accept waits for and returns the next connection to the listener
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr)>

    // incoming returns an iterator over the connections being received on this listener
    pub fn incoming(&self) -> Incoming<TcpStream>

    // local_addr returns the local socket address of the listener
    pub fn local_addr(&self) -> Result<SocketAddr>
}
```

Once you've created a listener, you can use TcpListener::incoming() to get an iterator over the incoming connections.

This method returns an iterator that yields connections as they are accepted, allowing you to handle each new connection in a loop.

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

      for stream in listener.incoming() {
         match stream {
               Ok(stream) => {
                  println!("New connection: {}", stream.peer_addr().unwrap());
               }
               Err(e) => {
                  println!("Error: {}", e);
               }
         }
      }
}
```



### The TcpStream struct

The iterator returned by the ```incoming``` method of the ```TcpListener``` struct yields a new ```TcpStream``` for each incoming connection. The ```TcpStream``` struct is used to read and write data to and from the connection.

Here are some methods that are available on the ```TcpStream``` struct:
```rust
impl TcpStream {
    // read reads bytes from the stream
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize>

    // write writes bytes to the stream and returns the number of bytes written.
    // It's often easier to use write_all instead of this method.
    pub fn write(&mut self, buf: &[u8]) -> Result<usize>

    // write_all writes all the bytes in buf to the stream
    pub fn write_all(&mut self, buf: &[u8]) -> Result<()>
}
```

To read data from a connection, you'll need to pass in a mutable byte slice to TcpStream::read. The data received will be stored in this byte slice. TcpStream::read returns a Result<usize> indicating the number of bytes read:

```rust
let mut buf = [0; 1024];
let n = stream.read(&mut buf)?;
println!("received {} bytes", n);
println!("data: {:?}", &buf[..n]);
```

To write data to a connection, you'll need to pass in a byte slice to TcpStream::write_all. It returns a Result<()> indicating whether the write was successful:

```rust
let buf = b"hello world";
stream.write_all(buf)?;
println!("wrote to stream");
```

Let's put it all together and build a simple TCP server in Rust.

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Creates a TCP server listening on localhost:8080
    let listener = TcpListener::bind("localhost:8080").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512]; // 512 byte buffer
    loop {
        let bytes_read = stream.read(&mut buf).expect("Failed to read from client");

        if bytes_read == 0 {
            return;
        }

        stream.write_all(&buf[0..bytes_read]).expect("Failed to write to client");
    }
}
```

There are some limitations to this server:
   - It can only handle one client at a time.
   - It reads a fixed-size buffer from the client, and writes the same buffer back to the client.
   - It doesn't handle errors gracefully.For example, if a client disconnects abruptly, the server will panic.
