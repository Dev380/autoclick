use crate::first::first;
use std::fs::File;

pub fn startup() {
    let license = match File::open("LICENSE") {
    Ok(l) => l,
    Err(e) => panic!(
        "Error: The license file cannot be accessed. Error code: {}. Please try downloading the program again.",
        e
    ),
    };
    if let Ok(_) = File::open("config.ron") {
        println!("Program starting...");
    } else {
        first(license);
    }
}
