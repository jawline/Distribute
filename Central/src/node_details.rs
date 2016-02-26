pub struct NodeDetails {
	pub address: String,
	pub key: String
}

impl NodeDetails {
	pub fn new(address: &str, key: &str) -> NodeDetails {
		NodeDetails {
			address: address.to_string(),
			key: key.to_string()
		}
	}
}