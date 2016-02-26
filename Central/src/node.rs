use std::net::{TcpListener, TcpStream};

pub struct Node {
	stream: TcpStream
}

impl Node {
	pub fn new(stream: TcpStream) -> Node {
		Node{
			stream: stream
		}
	}
}