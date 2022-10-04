use std::net::UdpSocket;
use serde_json::{Value};
use std::str;

pub fn server_socket_thread(){
    let socket = UdpSocket::bind("127.0.0.1:41234").unwrap();
    loop{
        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 6092];
        let (amt, _src) = socket.recv_from(&mut buf).unwrap();

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buff_str = String::from_utf8(buf[..amt].to_vec()).unwrap();

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(&buff_str).unwrap();

        println!("{}", v["important"]);
    }
}

pub fn client_socket_thread(){

}