use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "0.0.0.0:6000";
const SERVER_NAME: &str = "Server of Senam";
const SERVER_NUMBER: i32 = 10;

fn handle_client(mut stream: TcpStream) -> io::Result<bool> {
    let peer = stream.peer_addr()?;
    println!("\n[SERVER] Accepted TCP connection from {peer}");

    // The protocol uses two newline-terminated fields: client name, then integer.
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut client_name = String::new();
    let mut number_line = String::new();

    if reader.read_line(&mut client_name)? == 0 {
        println!("[SERVER] Client disconnected before sending its name.");
        return Ok(true);
    }
    if reader.read_line(&mut number_line)? == 0 {
        println!("[SERVER] Client disconnected before sending its integer.");
        return Ok(true);
    }

    let client_name = client_name.trim();
    let client_number: i32 = match number_line.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("[SERVER] Received an invalid integer from {client_name:?}; closing connection.");
            return Ok(true);
        }
    };

    println!("[SERVER] Client name: {client_name}");
    println!("[SERVER] Server name: {SERVER_NAME}");
    println!("[SERVER] Received client integer: {client_number}");

    // An out-of-range value is the assignment's server-shutdown signal.
    if !(1..=100).contains(&client_number) {
        println!(
            "[SERVER] Integer {client_number} is outside 1..=100. \
             Shutting down the server as requested."
        );
        stream.shutdown(std::net::Shutdown::Both)?;
        return Ok(false);
    }

    let sum = client_number + SERVER_NUMBER;
    println!("[SERVER] My integer: {SERVER_NUMBER}");
    println!("[SERVER] Sum: {client_number} + {SERVER_NUMBER} = {sum}");

    writeln!(stream, "{SERVER_NAME}")?;
    writeln!(stream, "{SERVER_NUMBER}")?;
    stream.flush()?;
    println!("[SERVER] Sent server name and integer to the client.");
    println!("[SERVER] Finished handling {client_name}.");

    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(true)
}

fn main() -> io::Result<()> {
    println!("=== TCP SERVER ===");
    println!("[SERVER] Name: {SERVER_NAME}");
    println!("[SERVER] Listening on {ADDRESS}");
    let listener = TcpListener::bind(ADDRESS)?;
    println!("[SERVER] Socket bound successfully. Waiting for clients...");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if !handle_client(stream)? {
                    break;
                }
            }
            Err(error) => eprintln!("[SERVER] Failed to accept connection: {error}"),
        }
    }

    println!("[SERVER] Server stopped. Listener will be released.");
    Ok(())
}
