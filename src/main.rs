use enigo::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();
    loop {
        enigo.mouse_move_to(500, 10);
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(Duration::from_secs(5));
        enigo.mouse_move_to(10, 500);
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(Duration::from_secs(5));
    }
}
