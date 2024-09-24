use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // Create a buffer to hold the incoming data
    match stream.read(&mut buffer) {
        Ok(size) => {
            // Print the received data as a string
            println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
        }
        Err(e) => {
            println!("Failed to read from stream: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

Expla