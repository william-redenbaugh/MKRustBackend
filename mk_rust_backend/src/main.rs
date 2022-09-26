use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:41234").unwrap();
    loop{
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        
        println!("{:?}", buf);
        buf.reverse();
        socket.send_to(buf, &src).unwrap();
    }   
}