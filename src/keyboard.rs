use crate::ports;
// use pc_keyboard::{self, layouts, Keyboard, ScancodeSet1};

use spin::Mutex;

pub unsafe fn get_pressed_key() -> Option<char> {
    static KEYBOARD: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x60)
    });
    let scancode = KEYBOARD.lock().read();
    let _keycode: char = match scancode_to_ascii(scancode) {
        Some(c) => return Some(c),
        None => return None,
    };
}

pub unsafe fn scancode_to_ascii(scancode: u8) -> Option<char> {
    // static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(ScancodeSet1, layouts::Us104Key, pc_keyboard::HandleControl::Ignore));
    // let mut keyboard = KEYBOARD.lock();
    match scancode {
        0x10 => Some('q'),
        0x11 => Some('w'),
        _ => None,
    }
}