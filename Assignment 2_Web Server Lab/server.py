
from socket import *
from pathlib import Path
from urllib.parse import unquote

HOST = ""
PORT = 6789
BASE_DIR = Path(__file__).resolve().parent

serverSocket = socket(AF_INET, SOCK_STREAM)
serverSocket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)

# Prepare a server socket
serverSocket.bind((HOST, PORT))
serverSocket.listen(1)

print("Ready to serve...")

try:
    while True:
        # Establish the connection
        connectionSocket, addr = serverSocket.accept()

        try:
            # Receive the HTTP request
            message = connectionSocket.recv(4096).decode(
                "iso-8859-1"
            )

            if not message:
                continue

            # Get the requested filename
            request_line = message.splitlines()[0]
            parts = request_line.split()

            if len(parts) < 2 or parts[0] != "GET":
                body = b"400 Bad Request"
                header = (
                    "HTTP/1.1 400 Bad Request\r\n"
                    "Content-Type: text/plain\r\n"
                    f"Content-Length: {len(body)}\r\n"
                    "Connection: close\r\n\r\n"
                )
                connectionSocket.sendall(
                    header.encode("iso-8859-1") + body
                )
                continue

            filename = unquote(parts[1].split("?", 1)[0])
            filename = filename.lstrip("/")
            filepath = (BASE_DIR / filename).resolve()

            # Prevent access to files outside the server folder
            if BASE_DIR not in filepath.parents:
                raise IOError("Invalid file path")

            # Read the requested file
            with open(filepath, "rb") as f:
                outputdata = f.read()

            # Create the HTTP response header
            header = (
                "HTTP/1.1 200 OK\r\n"
                "Content-Type: text/html; charset=utf-8\r\n"
                f"Content-Length: {len(outputdata)}\r\n"
                "Connection: close\r\n\r\n"
            )

            # Send the HTTP header
            connectionSocket.sendall(
                header.encode("iso-8859-1")
            )

            # Send the requested file
            connectionSocket.sendall(outputdata)

            print("File sent successfully:", filename)

        except (IOError, OSError, IndexError):
            # Send a 404 Not Found response
            body = b"404 Not Found"

            header = (
                "HTTP/1.1 404 Not Found\r\n"
                "Content-Type: text/plain; charset=utf-8\r\n"
                f"Content-Length: {len(body)}\r\n"
                "Connection: close\r\n\r\n"
            )

            connectionSocket.sendall(
                header.encode("iso-8859-1") + body
            )

            print("404 Not Found")

        finally:
            # Close the client socket
            connectionSocket.close()

except KeyboardInterrupt:
    print("\nServer stopped.")

finally:
    serverSocket.close()
