#![allow(dead_code)]

use fccore::config::Config;
use std::thread::sleep_ms;
use simplelog::Log;
use fccore::job::{Job, JobState};

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
            jobs: Job::new("Central"),
            config: config
        };

        let mut microbench = Job::new("Microbenchmarks");

        microbench.add_child(Job::new_with_state("Test 1", JobState::Failed));
        microbench.add_child(Job::new_with_state("Test 2", JobState::InProgress));
        microbench.add_child(Job::new_with_state("Test 4", JobState::InProgress));
        microbench.add_child(Job::new_with_state("Test 3", JobState::Success));
        microbench.add_child(Job::new("Test 5"));
        microbench.add_child(Job::new("Test 6"));
        microbench.add_child(Job::new("Test 7"));

        core.jobs.add_child(microbench);

        let mut browser = Job::new("Browser Tests");
        browser.add_child(Job::new_with_state("Test 1", JobState::InProgress));
        browser.add_child(Job::new("Test 2"));
        browser.add_child(Job::new("Test 3"));
        browser.add_child(Job::new("Test 4"));
        browser.add_child(Job::new("Test 5"));
        browser.add_child(Job::new("Test 6"));
        core.jobs.add_child(browser);

        let mut other = Job::new("Other Tests");
        other.add_child(Job::new_with_state("Test 1", JobState::Success));
        other.add_child(Job::new_with_state("Test 2", JobState::Success));
        other.add_child(Job::new_with_state("Test 3", JobState::Success));
        core.jobs.add_child(other);

        let mut crypto = Job::new("Crypto Tests");
        crypto.add_child(Job::new("Test 1"));
        crypto.add_child(Job::new("Test 2"));
        crypto.add_child(Job::new("Test 3"));
        
        core.jobs.add_child(crypto);

        core.log.add(TAG, &format!("Connecting to server {} with key {}", &core.config.server_url, &core.config.api_key));
        core
    }

    pub fn update(&mut self) {}

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
