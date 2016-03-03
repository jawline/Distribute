use std::net::{TcpListener, TcpStream};
use std::thread;
use fccore::node::Node;

pub fn node_listener(address: &str, core: Arc<Mutex<Core>>) {

	let listener = TcpListener::bind(address).unwrap();

	fn handle_client(stream) {
		
	}

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream)
	            });
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}

	// close the socket server
	drop(listener);
}