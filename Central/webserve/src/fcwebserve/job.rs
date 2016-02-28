use rustc_serialize::json;
use fccore::job::Job;

#[derive(RustcEncodable, RustcDecodable)]
pub struct SerializedJob {
	pub uid: usize,
	pub name: String,
	pub children: Vec<SerializedJob>
}

impl SerializedJob {
	pub fn new(job: &Job) -> SerializedJob {
		SerializedJob {
			uid: job.uid,
			name: job.task_name.to_string(),
			children: job.children.iter().map(|x| SerializedJob::new(x)).collect()
		}
	}
}