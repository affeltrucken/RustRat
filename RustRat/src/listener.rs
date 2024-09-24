use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("Client connected!");

    loop {
        let mut input = String::new();
        println!("Enter command to send to client: ");
        
        // Get the command input from the host
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        
        if input.trim().is_empty() {
            continue;
        }

        // Send the command to the client
        if let Err(e) = stream.write(input.as_bytes()) {
            println!("Error sending command: {}", e);
            break;
        }

        let mut buffer = [0; 1024]; // Buffer for response
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => {
                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("Client response:\n{}", response);
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:6300")?;
    println!("Listening on port 6300...");

    // Accept connections and handle each one in a separate thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
