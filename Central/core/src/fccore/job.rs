use std::vec::Vec;
use fccore::job_config::JobConfig;

static mut LAST_UID: usize = 0;

#[derive(PartialEq, Clone)]
pub enum JobState {
	Success,
	Failed,
	InProgress,
	NotStarted
}

impl ToString for JobState {
	fn to_string(&self) -> String {
		match *self {
			JobState::Success => "success",
			JobState::Failed => "failed",
			JobState::InProgress => "in_progress",
			JobState::NotStarted => "not_started"
		}.to_string()
	}
}

pub struct Job {
	pub uid: usize,
	pub task_name: String,
	pub children: Vec<Job>,
	pub state: JobState
}

impl Job {

	pub fn from_config(config: &JobConfig) -> Job {

		let mut new_job = Job::new(&config.name);

		let children: Vec<Job> = config.children.iter().map(|child_config| Job::from_config(child_config)).collect();

		for child in children {
			new_job.add_child(child);
		}

		new_job
	}

	pub fn new(name: &str) -> Job {
		Job::new_with_state(name, JobState::NotStarted)
	}

	pub fn new_with_state(name: &str, state: JobState) -> Job {
		unsafe {

			let result = Job {
				uid: LAST_UID,
				task_name: name.to_string(),
				children: Vec::new(),
				state: state
			};

			LAST_UID = LAST_UID + 1;

			result
		}
	}

	pub fn add_child(&mut self, job: Job) {
		self.children.push(job);
	}

	pub fn get_state(&self) -> JobState {
		if self.children.len() > 0 {
			let mut base = JobState::Success;

			for child in &self.children {
				match child.get_state(){
					JobState::Success => {
						if base != JobState::Failed && base != JobState::InProgress && base != JobState::NotStarted {
							base = JobState::Success
						}
					},
					
					JobState::InProgress => {
						if base != JobState::Failed {
							base = JobState::InProgress;
						}
					},
					
					JobState::Failed => {
						base = JobState::Failed;
					},

					JobState::NotStarted => {
						if base != JobState::InProgress && base != JobState::Failed {
							base = JobState::NotStarted;
						}
					}
				};
			}
			base
		} else {
			self.state.clone()
		}
	}
}