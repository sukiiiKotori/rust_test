import socket

def CreatCilent():
    tcp_socket=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
    ip=input('please input server IP: ')
    tcp_socket.connect((ip,6666))
    name=input('please input your name: ')
    tcp_socket.send((name).encode('utf-8'))
    while True:
        while True:
            path=input('''please input file path such as 'xxx_test.rs': ''')
            try:
                f=open('src/' + path,'rb') 
                break
            except IOError:
                print('file path ERROR')
        #转为字节流
        data=f.read()
        f.close()
        #计算长度
        lenth=len(data)
        #发送长度
        lenth_bytes=lenth.to_bytes(length=4,byteorder='big',signed=False)
        tcp_socket.send(lenth_bytes)
        #发送文件名
        fname=path
        print(fname)
        tcp_socket.send((fname+'\n').encode('ascii'))
        #发送文件流
        print("sending file size:%dB"%len(data))
        tcp_socket.send(data)
        print(fname+" is all sent!")

if __name__ == '__main__':
    CreatCilent()