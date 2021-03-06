use rustc_serialize::json;
use std::fs::File;
use std::io::{Read};
use std::string::{String, ToString};

#[derive(RustcEncodable, RustcDecodable)]
pub struct JobConfig {
    pub name: String,
    pub children: Vec<JobConfig>
}

impl JobConfig {
    fn read_config_file(base_file: &str) -> String {
        let mut result = String::new();
        
        if let Err(_) = File::open(base_file).unwrap().read_to_string(&mut result) {
            panic!("Could not read from FCConfig file {}", base_file);
        }
    
        return result;
    }
  
    pub fn load(base_file: &str) -> JobConfig {
        let text = JobConfig::read_config_file(base_file);
        return json::decode(&text).unwrap();
    }
}

impl ToString for JobConfig {
    fn to_string(&self) -> String {
        json::encode(self).unwrap()
    }
}