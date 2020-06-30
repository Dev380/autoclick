use crate::config::Config;
use crate::print::debug;
use crate::start::startup;
use enigo::*;
use ron::de;
use std::fs::File;
use std::thread;
use std::time::Duration;

mod config;
mod first;
mod print;
mod start;

fn main() {
    startup();
    let mut enigo = Enigo::new();
    let f = match File::open("config.ron") {
        Ok(f) => f,
        Err(e) => panic!(
            "Error loading config file. Please delete it and try again. Error code: {}",
            e
        ),
    };
    let config: Config = match de::from_reader(f) {
        Ok(c) => c,
        Err(e) => panic!(
            "Error reading config file. Please delete it and try again. Error code: {}",
            e
        ),
    };
    let seconds = config.seconds_get();
    if config.debug_get() {
        debug(seconds);
        loop {
            enigo.mouse_move_to(500, 10);
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(seconds));
            debug(seconds);
            enigo.mouse_move_to(10, 500);
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(seconds));
            debug(seconds);
        }
    } else if config.running_get() {
        println!("Running... (if you don't want these messsages to appear, change the 'running_message' variable in config.ron to false)" );
        loop {
            enigo.mouse_move_to(500, 10);
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(seconds));
            println!("Running...");
            enigo.mouse_move_to(10, 500);
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(seconds));
            println!("Running...");
        }
    } else {
        loop {
            enigo.mouse_move_to(500, 10);
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(seconds));
            enigo.mouse_move_to(10, 500);
            enigo.mouse_click(MouseButton::Left);
            thread::sleep(Duration::from_secs(seconds));
        }
    }
}
