import socket
import requests
import json

HOST = '127.0.0.1'
SOCKET_PORT = 9010
SERVER_PORT = 9000
SUCCESS = 200

MOCK_ID = '680908739585f2452bf4cbbe'
MOCK_DEVICE_TYPE = 1

def register_device():
    url = 'http://' + HOST + ':' + str(SERVER_PORT) + "/register_device"
    
    for i in range(3):
        print("Trying to register a device. Try: ", i)
        response = requests.post(url, json = { 'device_id' : MOCK_ID, 'device_type' : MOCK_DEVICE_TYPE })

        if  response.status_code == SUCCESS:
        
            response_json = response.json()
            new_registration = response_json["new_registration"]

            if new_registration:
                print("This device was not registered before")
            else:
                print("This device was already registered")
            return True
        
    return False

def send_device_info_to_socket(socket):
    data = {
        "device_id" : MOCK_ID,
        "device_type" : MOCK_DEVICE_TYPE
    }

    json_data = json.dumps(data)
    encoded_data = json_data.encode('utf-8')
    encoded_data += b'\n'

    socket.sendall(encoded_data);

    resp = socket.recv(1024).decode()
    print("Server response: ", resp)

def main():
    if not register_device():
        print("Registration failed!")
        return

    print("Registration succeeded!")
    
    with socket.create_connection((HOST, SOCKET_PORT)) as sock:
        print(f"Connected to a server {HOST} at port {SOCKET_PORT}")

        send_device_info_to_socket(sock)

        while True:
            try:
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