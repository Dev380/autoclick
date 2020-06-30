use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    seconds: u64,
    debug: bool,
    running_message: bool,
}
impl Config {
    pub fn new(seconds: u64, debug: bool, running_message: bool) -> Config {
        if seconds != 0 {
            Config {
                seconds,
                debug,
                running_message,
            }
        } else {
            panic!("Number must be non-zero!");
        }
    }
    pub fn seconds_get(&self) -> u64 {
        self.seconds
    }
    pub fn debug_get(&self) -> bool {
        self.debug
    }
    pub fn running_get(&self) -> bool {
        self.running_message
    }
}
