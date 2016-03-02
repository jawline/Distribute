use fccore::Core;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::{spawn, sleep, JoinHandle};

const TAG: &'static str = "fccore_logic";
const LOOP_DELAY_MS: u64 = 1500;

pub fn start_logic_thread(core: &Arc<Mutex<Core>>) -> JoinHandle<()> {
    let thread_core = core.clone();
    thread_core.lock().unwrap().log_mut().add(TAG, "starting logic thread loop");
    spawn(move || { fccore_thread_loop(thread_core); } )
}

fn fccore_thread_loop(core_ref: Arc<Mutex<Core>>) {
    core_ref.lock().unwrap().log_mut().add(TAG, "Starting core logic");

    let delay = Duration::from_millis(LOOP_DELAY_MS);

    while core_ref.lock().unwrap().alive {
    	core_ref.lock().unwrap().update();
    	sleep(delay);
    }

    core_ref.lock().unwrap().log_mut().add(TAG, "Core is no longer alive, logic thread exit");
}