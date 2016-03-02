use fccore::job::Job;

#[derive(RustcEncodable, RustcDecodable)]
pub struct SerializedJob {
	pub uid: usize,
	pub name: String,
	pub children: Vec<SerializedJob>,
	pub state: String
}

impl SerializedJob {
	pub fn new(job: &Job) -> SerializedJob {
		SerializedJob {
			uid: job.uid,
			name: job.task_name.to_string(),
			state: job.get_state().to_string(),
			children: job.children.iter().map(|x| SerializedJob::new(x)).collect()
		}
	}
}