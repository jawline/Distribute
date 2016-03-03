use std::net::{TcpListener, TcpStream};
use std::thread;
use fccore::node::Node;

pub fn node_listener(address: &str, core: Arc<Mutex<Core>>) {

	let listener = TcpListener::bind(address).unwrap();

	fn handle_client(stream) {
		core.lock().unwrap().add_node(Node::new(stream));
	}

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => handle_client(stream),
	        Err(e) => { println!("Connection from node failed"); }
	    }
	}

	// close the socket server
	drop(listener);
}