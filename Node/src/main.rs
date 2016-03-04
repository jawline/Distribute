extern crate protocol;

use std::net::TcpStream;
use protocol::node::Node;
use protocol::handshakes::client_handshake;

fn main() {
    let mut node = Node::new(TcpStream::connect("127.0.0.1:19760").unwrap());

    let handshake_result = client_handshake(&mut node);

    match handshake_result {
    	Ok(()) => { println!("Handshake done"); },
    	Err(why) => {
 			println!("Handshake failed because {}", why);
    	}
    }
}