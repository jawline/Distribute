#![allow(dead_code)]

use fccore::config::Config;
use std::thread::sleep_ms;
use simplelog::Log;
use fccore::job::Job;

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
