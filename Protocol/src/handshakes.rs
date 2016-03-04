use node::Node;

pub fn client_handshake(node: &mut Node) -> Result<(), &'static str> {
	
	if node.read_line() != "SERVER0" {
		return Err("Unexpected server magic");
	}

	node.write_line("NODE0");

	node.write_line("PRINT_FEATURES");
	node.write_line("DONE_FEATURES");

	if node.read_line() != "THANKS" {
		return Err("Unexpected server message, expecting THANKS");
	}

	Ok(())
}

pub fn server_handshake(node: &mut Node) -> Result<(), &'static str> {

	node.write_line("CENTRAL0");

	if node.read_line() != "NODE0" {
		return Err("Unexpected client magic");
	}

	if node.read_line() != "PRINT_FEATURES" {
		return Err("Expected PRINT_FEATURES");
	}

	let mut features = Vec::new();

	loop {
		let line = node.read_line();

		if line == "DONE_FEATURES" {
			break;
		}
		
		features.push(line);
	}

	node.write_line("THANKS");

	Ok(())
}