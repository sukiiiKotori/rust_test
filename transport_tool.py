import socket

name = 'test111'  # 把这里改成你自己的姓名

def CreatCilent():
    tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    tcp_socket.connect(('192.168.1.116', 6666))
    tcp_socket.send((name).encode('utf-8'))
    while True:
        while True:
            path = input('''please input file path such as 'xxx_test.rs': ''')
            try:
                f = open('src/' + path, 'rb')
                break
            except IOError:
                print('file path ERROR')
        # 转为字节流
        data = f.read()
        f.close()
        lenth = len(data)
        # 发送文件长度
        lenth_bytes = lenth.to_bytes(length=4, byteorder='big', signed=False)
        tcp_socket.send(lenth_bytes)
        # 发送文件名
        fname = path
        tcp_socket.send((fname+'\n').encode('ascii'))
        # 发送文件字节流
        print("sending file size:%dB" % len(data))
        tcp_socket.send(data)
        print(fname+" is all sent!")


if __name__ == '__main__':
    CreatCilent()
