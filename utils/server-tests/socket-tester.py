import socket

HOST = '127.0.0.1'
PORT = 9010

def main():
    with socket.create_connection((HOST, PORT)) as sock:
        print(f"Connected to a server {HOST} at port {PORT}")

        while True:
            try:
                msg = input("Message to send: ")
                if msg.strip().lower == 'exit':
                    print("Closeing connection")
                    break

                sock.sendall((msg + '\n').encode())

                response = sock.recv(1024).decode()
                print(f"Server responded: {response}")
            except (KeyboardInterrupt, EOFError):
                print("Disconnected")
                break
            except Exception as e:
                print(f"Error: {e}")
                break

if __name__ == '__main__':
    main()