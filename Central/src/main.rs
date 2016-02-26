mod listener;
mod node;

use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
	println!("Handling new node");
}

fn main() {
    println!("C&C Central Node");

    listener::listen("127.0.0.1:13450", handle_client);
}
