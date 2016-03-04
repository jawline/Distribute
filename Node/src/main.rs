extern crate protocol;

use std::net::TcpStream;
use protocol::node::Node;
use protocol::handshakes::client_handshake;

fn main() {
    let mut node = Node::new(TcpStream::connect("127.0.0.1:19760").unwrap());

    if !client_handshake(&mut node) {
    	println!("Handshake failed");
    } else {
    	println!("Handshake was fine");
    }
}