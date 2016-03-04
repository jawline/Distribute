use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write, BufWriter};

pub struct Node {
	pub conn: BufReader<TcpStream>
}

impl Node {
	
	pub fn new(conn: TcpStream) -> Node {
		Node {
			conn: BufReader::new(conn)
		}
	}

	pub fn flush(&mut self) {
		self.conn.get_ref().flush();
	}

	pub fn read_line(&mut self) -> String {
		let mut line = String::new();
		self.conn.read_line(&mut line);
		return line.trim().to_string();
	}

	pub fn write_line(&mut self, line: &str) {
		write!(self.conn.get_ref(), "{}\n", line);
		self.flush();
	}
}