use std::vec::Vec;

static mut LAST_UID: usize = 0;

pub struct Job {
	pub uid: usize,
	pub task_name: String,
	pub children: Vec<Job>
}

impl Job {

	pub fn new(name: &str) -> Job {
		unsafe {

			let result = Job {
				uid: LAST_UID,
				task_name: name.to_string(),
				children: Vec::new()
			};

			LAST_UID = LAST_UID + 1;

			result
		}
	}

	pub fn add_child(&mut self, job: Job) {
		self.children.push(job);
	}

	pub fn get_state() -> String {
		"success".to_string()
	}
}