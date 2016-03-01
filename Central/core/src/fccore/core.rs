#![allow(dead_code)]

use fccore::config::Config;
use std::thread::sleep_ms;
use simplelog::Log;
use fccore::job::{Job, JobState};
use fccore::job_config::JobConfig;
use rand;

use time;

const TAG : &'static str = "core";
const LOG_DIR : &'static str = "./logs/";

pub struct Core {

    /**
     * Is the core alive
     */
    pub alive: bool,
  
    /**
     * configuration for the core
     */
    config: Config,

    pub jobs: Job,
  
    /**
     * Core log, stores log messages and timestamps
     */
    log: Log
}

impl Core {

    pub fn new(config_file : &str) -> Core {
        let config = Config::load(config_file);
        
        let mut core = Core {
            alive: true,
            log: Log::new(&format!("{}log{}", LOG_DIR, time::now().to_timespec().sec), config.log_config.log_limit),
            jobs: Job::from_config(&JobConfig::load(&config.job_config)),
            config: config
        };

        core
    }

    fn deep_random(job: &mut Job) {
        if job.children.len() == 0 {
            job.state = match rand::random::<u8>() % 10 {
                1 | 2 | 3 => JobState::Success,
                5 => JobState::InProgress,
                9 => JobState::Failed,
                _ => JobState::NotStarted
            };
        } else {
            for child in &mut job.children {
                Core::deep_random(child);
            }
        }
    }

    pub fn update(&mut self) {
        Core::deep_random(&mut self.jobs);
    }

    /**
     * Get the core config struct
     */
    pub fn config(&self) -> &Config { &self.config }
    
    /**
     * Return the core log
     */
    pub fn log(&self) -> &Log { &self.log }
    
    /**
     * Return the core log as mutable
     */
     pub fn log_mut(&mut self) -> &mut Log { &mut self.log }
}
