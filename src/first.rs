use crate::config::Config;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs::File;
use std::io::{self, prelude::*};

pub fn first(mut license: File) {
    println!("Welcome, new user! We will commence your setup process shortly");
    println!("");
    println!(
        " Autoclick  Copyright (C) 2020  Dev380
This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `show c' to read the entire license.
Type anything else to continue. By continuing, you accept the terms
and conditions outlined in the license file included with the program.
This message will not be shown again.
"
    );
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
        x if "show w".to_string() == x => println!("15. Disclaimer of Warranty.

THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY APPLICABLE LAW.
EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM “AS IS” WITHOUT WARRANTY OF ANY KIND,
EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION."),
        y if "show c".to_string() == y => {
            let mut li = String::new();
            license.read_to_string(&mut li).expect("Failed to read license!");
            println!("{}", li);
        },
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
        let c = Config::new(inp, true);
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
        break;
    }
}
