use crate::first::first;
use std::fs::File;

pub fn startup() {
    if let Ok(_) = File::open("config.ron") {
        println!("Program starting...");
    } else {
        first();
    }
}
