use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write, BufWriter};

pub struct Node {
	pub conn: TcpStream
}

impl Node {
	
	pub fn new(conn: TcpStream) -> Node {
		Node {
			conn: conn
		}
	}

	fn reader(&mut self) -> BufReader<&TcpStream> {
		BufReader::new(&self.conn)
	}

	fn writer(&mut self) -> BufWriter<&TcpStream> {
		BufWriter::new(&self.conn)
	}

	pub fn read_line(&mut self) -> String {
		let mut line = String::new();
		self.reader().read_line(&mut line);
		return line.trim().to_string();
	}

	pub fn write_line(&mut self, line: &str) {
		write!(self.writer(), "{}\n", line);
	}
}