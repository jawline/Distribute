use node::Node;

pub fn client_handshake(node: &mut Node) -> Result<(), String> {
	
	let magic = node.read_line();

	if magic != "CENTRAL0" {
		return Err(("Unexpected magic: ".to_string() + &magic));
	}

	node.write_line("NODE0");
	node.write_line("PRINT_FEATURES");
	node.write_line("DONE_FEATURES");

	if node.read_line() != "THANKS" {
		return Err("Unexpected server message, expecting THANKS".to_string());
	}

	Ok(())
}

pub fn server_handshake(node: &mut Node) -> Result<(), String> {

	node.write_line("CENTRAL0");

	let magic = node.read_line();

	if magic != "NODE0" {
		return Err(("Unexpected magic: ".to_string() + &magic));
	}

	if node.read_line() != "PRINT_FEATURES" {
		return Err("Expected PRINT_FEATURES".to_string());
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