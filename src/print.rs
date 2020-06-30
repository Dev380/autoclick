use std::time::SystemTime;

pub fn debug(seconds: u64) {
    println!(
        "DEBUG canary. If this message does not appear in {} seconds, an error has occured.
Change the debug variable in config.ron to 'false' if you don't want these messages to appear.
{:?} If you installed via cargo, it will be in /.cargo/bin
",
        seconds,
        SystemTime::now()
    );
}
pub fn start() {
    println!(
        " Autoclick  Copyright (C) 2020  Dev380
This program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.
This is free software, and you are welcome to redistribute it
under certain conditions; type `show c' to read the entire license.
Type anything else to continue. By continuing, you accept the terms
and conditions outlined in the license file included with the program.
This message will not be shown again. You should have received a copy of
the license with your download. If not, go to https://www.gnu.org/licenses/gpl.html
"
    );
}
pub fn warranty() {
    println!("15. Disclaimer of Warranty.

THERE IS NO WARRANTY FOR THE PROGRAM, TO THE EXTENT PERMITTED BY APPLICABLE LAW.
EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR OTHER PARTIES PROVIDE THE PROGRAM “AS IS” WITHOUT WARRANTY OF ANY KIND,
EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE PROGRAM IS WITH YOU. SHOULD THE PROGRAM PROVE DEFECTIVE, YOU ASSUME THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.")
}
