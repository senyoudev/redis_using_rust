use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;

pub struct Client {
    stream: BufStream<TcpStream>,
}

impl Client {
    /// Create a new client.
    pub async fn connect(uri: SocketAddr) -> Self {
        let stream = TcpStream::connect(uri).await.expect("Failed to connect to master");
        Self {
            stream: BufStream::new(stream),
        }
    }

    /// Send a command to the master.
    pub async fn exec(&mut self, cmd: &str, args: &[String]) -> tokio::io::Result<String> {
        // Construct the command as a string.
        let mut command = String::new();
        command.push_str(cmd);
        for arg in args {
            command.push(' ');
            command.push_str(arg);
        }
        command.push('\r');
        command.push('\n');
        
        // Serialize the command and send it to the remote.
        self.stream.write_all(command.as_bytes()).await?;
        self.stream.flush().await?;
        
        // Wait for the response.
        let mut buffer = vec![0; 512]; // Adjust the buffer size as needed
        self.stream.read_exact(&mut buffer).await?;
        println!("Received: {:?}", String::from_utf8_lossy(&buffer));
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}
