use std::net::{TcpListener, TcpStream};
use std::thread;
use fccore::node::Node;
use fccore::core::Core;
use std::sync::{Arc, Mutex};

pub fn node_listener(address: &str, core: Arc<Mutex<Core>>) {

	let listener = TcpListener::bind(address).unwrap();

	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
		
		if !core.lock().unwrap().alive {
			println!("Node listener exiting as server is no longer alive");
	    	break;
	    }

	    match stream {
	        Ok(stream) => {
				core.lock().unwrap().add_node(Node::new(stream))
	        },
	        Err(e) => { println!("Connection from node failed"); }
	    }

	}

	// close the socket server
	drop(listener);
}