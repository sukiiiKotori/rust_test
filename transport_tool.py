import os
import time
import socket

name = 'test'  # 把这里改成你自己的姓名

def CreatCilent():
    tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    tcp_socket.connect(('192.168.1.117', 6666))
    tcp_socket.send((name).encode('utf-8'))
    time.sleep(0.1)
    file_list = os.listdir('src')
    for fname in file_list:
        f = open('src/' + fname, 'rb')
        # transform to byte stream
        data = f.read()
        f.close()
        lenth = len(data)
        lenth_bytes = lenth.to_bytes(length=4, byteorder='big', signed=False)
        tcp_socket.send(lenth_bytes)
        tcp_socket.send((fname+'\n').encode('ascii'))
        tcp_socket.send(data)
        print(fname+" is sent")
        time.sleep(0.1)
    print('所有文件均上传成功')

if __name__ == '__main__':
    CreatCilent()