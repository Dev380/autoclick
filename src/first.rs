use crate::config::Config;
use crate::print::{start, warranty};
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs::File;
use std::io::{self, prelude::*};

fn config(inp: u64) {
    let c = Config::new(inp, true, true);
    let mut con = File::create("config.ron").expect("Error creating file. Please try again.");
    if let Err(e) = write!(
        con,
        "{}",
        to_string_pretty(
            &c,
            PrettyConfig::new()
                .with_depth_limit(2)
                .with_separate_tuple_members(true)
                .with_enumerate_arrays(true),
        )
        .expect("Serilization failed! Please try again.")
    ) {
        panic!("Error writing to config.ron. Error code: {}", e);
    }
}
pub fn first() {
    println!("Welcome, new user! We will commence your setup process shortly");
    println!("");
    start();
    // let w = "show w".to_string();
    // let c = "show c".to_string();
    loop {
        let mut inp = String::new();
        let mut counter = 3;
        loop {
            match io::stdin().read_line(&mut inp) {
                Ok(_) => break,
                Err(e) => {
                    if counter != 0 {
                        counter -= 1;
                        continue;
                    } else {
                        panic!(
                            "Error: {}. This is probably a problem with your operating system.",
                            e
                        );
                    }
                }
            }
        }
        match inp.trim() {
            x if "show w".to_string() == x => warranty(),
            y if "show c".to_string() == y => {
                println!("Please visit https://www.gnu.org/licenses/gpl.html to view the license.")
            }
            _ => break,
        }
    }
    let mut inp = String::new();
    loop {
        println!("How many seconds do you want to wait before the mouse moves?");
        let mut counter = 3;
        loop {
            match io::stdin().read_line(&mut inp) {
                Ok(_) => break,
                Err(e) => {
                    if counter != 0 {
                        counter -= 1;
                        continue;
                    } else {
                        panic!(
                            "Error: {}. This is probably a problem with your operating system.",
                            e
                        );
                    }
                }
            }
        }
        let inp = match inp.trim().parse() {
            Ok(u) => u,
            Err(_) => continue,
        };
        if inp == 0 {
            println!("0 isn't allowed!");
            continue;
        }
        config(inp);
        break;
    }
}
