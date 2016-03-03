use std::net::TcpStream;

pub struct Node {
	pub conn: TcpStream
}

impl Node {
	pub fn new(conn: TcpStream) -> Node {
		Node {
			conn: conn
		}
	}
}