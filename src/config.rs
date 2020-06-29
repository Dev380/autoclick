use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    seconds: u64,
    debug: bool,
}
impl Config {
    pub fn new(seconds: u64, debug: bool) -> Config {
        if seconds != 0 {
            Config { seconds, debug }
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
}
