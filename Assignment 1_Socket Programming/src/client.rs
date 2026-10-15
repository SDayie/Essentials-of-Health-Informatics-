use std::io::{self, BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};

const SERVER_ADDRESS: &str = "127.0.0.1:6000";
const CLIENT_NAME: &str = "Client of Senam";

fn main() -> io::Result<()> {
    println!("=== TCP CLIENT ===");
    println!("[CLIENT] Name: {CLIENT_NAME}");
    println!("Enter an integer. Use 1 through 100 for a normal request.");
    println!("To stop the server, enter a number outside that range (for example, 0).");
    print!("Your integer: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let client_number: i32 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("[CLIENT] Invalid input. Please enter a whole number.");
            return Ok(());
        }
    };

    println!("[CLIENT] Opening TCP connection to {SERVER_ADDRESS}...");
    let mut stream = TcpStream::connect(SERVER_ADDRESS)?;
    println!("[CLIENT] Connected successfully.");

    // Send the two fields as separate lines so the server can read them reliably.
    writeln!(stream, "{CLIENT_NAME}")?;
    writeln!(stream, "{client_number}")?;
    stream.flush()?;
    println!("[CLIENT] Sent client name and integer {client_number}.");

    if !(1..=100).contains(&client_number) {
        println!("[CLIENT] This value is outside 1..=100 and is intended to stop the server.");
        stream.shutdown(Shutdown::Both)?;
        println!("[CLIENT] Socket closed.");
        return Ok(());
    }

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut server_name = String::new();
    let mut server_number_line = String::new();

    if reader.read_line(&mut server_name)? == 0
        || reader.read_line(&mut server_number_line)? == 0
    {
        eprintln!("[CLIENT] The server closed the connection without a complete reply.");
        stream.shutdown(Shutdown::Both)?;
        return Ok(());
    }

    let server_name = server_name.trim();
    let server_number: i32 = server_number_line.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "Server sent a non-integer value")
    })?;

    println!("[CLIENT] Received reply from server.");
    println!("[CLIENT] Client name: {CLIENT_NAME}");
    println!("[CLIENT] Server name: {server_name}");
    println!("[CLIENT] Client integer: {client_number}");
    println!("[CLIENT] Server integer: {server_number}");
    println!("[CLIENT] Sum: {client_number} + {server_number} = {}", client_number + server_number);

    stream.shutdown(Shutdown::Both)?;
    println!("[CLIENT] Socket closed. Client finished.");
    Ok(())
}
