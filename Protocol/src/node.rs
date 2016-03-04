use std::net::TcpStream;
use std::io::{Error, BufRead, BufReader, Write, BufWriter};

pub struct Node {
	pub conn: BufReader<TcpStream>,
	pub features: Vec<String>
}

impl Node {
	
	pub fn new(conn: TcpStream) -> Node {
		Node {
			conn: BufReader::new(conn),
			features: Vec::new()
		}
	}

	pub fn add_feature(&mut self, feature: &str) {
		self.features.push(feature.to_string());
	}

	pub fn flush(&mut self) {
		self.conn.get_ref().flush();
	}

	pub fn read_line(&mut self) -> Result<String, Error> {
		let mut line = String::new();

		let result = self.conn.read_line(&mut line);

		match result {
			Ok(_) => Ok(line.trim().to_string()),
			Err(err) => Err(err)
		}
	}

	pub fn write_line(&mut self, line: &str) -> Result<(), Error> {
		write!(self.conn.get_ref(), "{}\n", line)
	}
}