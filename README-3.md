# Programming Assignment 1: Socket Programming

## How It Works

The project consists of a TCP client and a server written in Rust.

1. The **server** listens for incoming client connections.
2. The **client** asks the user for an integer between 1 and 100 and sends it along with its name to the server.
3. The **server** receives the message, always uses the integer `10`, calculates the sum, and sends its name and integer back to the client.
4. The **client** receives the reply, calculates the sum independently, and displays the results.
5. Both programs close their sockets when finished.

## How to Run

**1. Start the server**

Open Terminal 1 in the project folder and run:

```bash
cargo run --bin server
```

Keep this terminal open.

**2. Start the client**

Open Terminal 2 in the same project folder and run:

```bash
cargo run --bin client
```

**3. Enter a number**

Enter an integer between `1` and `100` and press Enter.

**4. Check the output**

The server and client display their numbers and calculated sums.

**Note:** Start the server before running the client.
