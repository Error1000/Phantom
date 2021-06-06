import struct
import socket
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(('localhost', 41114))
message = b"start:/tmp/res.bin"
size = len(message)
s.send(struct.pack("!H", size))
s.send(message)
print(s.recv(1024))
