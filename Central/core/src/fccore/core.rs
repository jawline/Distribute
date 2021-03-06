#![allow(dead_code)]

use fccore::config::Config;
use simplelog::Log;
use fccore::job::{Job, JobState};
use fccore::job_config::JobConfig;
use protocol::node::Node;
use rand;

use time;

const TAG : &'static str = "core";
const LOG_DIR : &'static str = "./logs/";

pub struct Core {

    /**
     * Random ID generated once when core is created
     */
    pub rid: usize,

    /**
     * Is the core alive
     */
    pub alive: bool,
  
    /**
     * configuration for the core
     */
    config: Config,

    pub jobs: Job,
    pub nodes: Vec<Node>,
  
    /**
     * Core log, stores log messages and timestamps
     */
    log: Log
}

impl Core {

    pub fn new(config_file : &str) -> Core {
        let config = Config::load(config_file);
        
        Core {
            rid: rand::random::<usize>(),
            alive: true,
            log: Log::new(&format!("{}log{}", LOG_DIR, time::now().to_timespec().sec), config.log_config.log_limit),
            jobs: Job::from_config(&JobConfig::load(&config.job_config)),
            nodes: Vec::new(),
            config: config
        }
    }

    fn deep_random(job: &mut Job) {
        if job.children.len() == 0 {
            if job.state == JobState::NotStarted || job.state == JobState::InProgress {
                job.state = match rand::random::<u8>() % 40 {
                    1 | 2 | 3 => JobState::Success,
                    5 => JobState::InProgress,
                    9 => JobState::Failed,
                    _ => JobState::NotStarted
                };
            }
        } else {
            for child in &mut job.children {
                Core::deep_random(child);
            }
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
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
