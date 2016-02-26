use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn listen(addr: &str, callback: fn(TcpStream)) {

	println!("Starting to listen");

	let listener = TcpListener::bind(addr).unwrap();

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
	            callback(stream)
	        }
	        Err(e) => { println!("Client failed to connect"); /* connection failed */ }
	    }
	}

	// close the socket server
	drop(listener);
}