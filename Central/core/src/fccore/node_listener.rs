use std::net::{TcpListener, TcpStream};
use std::thread::{spawn, sleep, JoinHandle};
use fccore::node::Node;
use fccore::core::Core;
use std::sync::{Arc, Mutex};

const TAG: &'static str = "Node Listener";

pub fn start_node_listener(core: Arc<Mutex<Core>>) -> JoinHandle<()> {
    core.lock().unwrap().log_mut().add(TAG, "starting logic thread loop");
    let addr = core.lock().unwrap().config().listener_address.to_string();
    spawn(move || { node_listener(&addr, core); } )
}

pub fn node_listener(address: &str, core: Arc<Mutex<Core>>) {

	println!("Launching node listener on {}", address);

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

	println!("Node listener closing");

	// close the socket server
	drop(listener);
}