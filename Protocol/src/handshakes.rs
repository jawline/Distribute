use node::Node;

pub fn client_handshake(node: &mut Node) -> Result<(), String> {
	
	let magic = node.read_line().unwrap();

	if magic != "CENTRAL0" {
		return Err(("Unexpected magic: ".to_string() + &magic));
	}

	node.write_line("NODE0");
	node.write_line("PRINT_FEATURES");

	let feature_list = node.features.clone();
	
	for feature in feature_list {
		node.write_line(&feature);
	}

	node.write_line("DONE_FEATURES");

	if node.read_line().unwrap() != "THANKS" {
		return Err("Unexpected server message, expecting THANKS".to_string());
	}

	Ok(())
}

pub fn server_handshake(node: &mut Node) -> Result<(), String> {

	node.write_line("CENTRAL0");

	let magic = node.read_line().unwrap();

	if magic != "NODE0" {
		return Err(("Unexpected magic: ".to_string() + &magic));
	}

	if node.read_line().unwrap() != "PRINT_FEATURES" {
		return Err("Expected PRINT_FEATURES".to_string());
	}

	loop {
		let line = node.read_line().unwrap();

		if line == "DONE_FEATURES" {
			break;
		}
		
		node.add_feature(&line);
	}

	node.write_line("THANKS");

	Ok(())
}