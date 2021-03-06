use rustc_serialize::json;
use std::string::{String, ToString};
use fccore::Core;
use std::sync::MutexGuard;
use iron::prelude::*;
use std::sync::{Arc, Mutex};
use iron::status;
use iron::mime::Mime;
use fcwebserve::job::SerializedJob;

#[derive(RustcEncodable)]
struct Status {
    pub id: usize,
    pub alive: bool,
    pub job: SerializedJob
}

impl Status {
    pub fn from(core: &MutexGuard<Core>) -> Status {
        Status{
            id: core.rid,
            alive: core.alive,
            job: SerializedJob::new(&core.jobs)
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        json::encode(self).unwrap()
    }
}

pub fn status_report(core_ref :&Arc<Mutex<Core>>) -> Response {
    let json_content_type : Mime = "application/json".parse::<Mime>().unwrap();
    Response::with((json_content_type, status::Ok, Status::from(&core_ref.lock().unwrap()).to_string()))
}