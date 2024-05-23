#![no_std]
#![no_main]
#![feature(portable_simd)]
#![feature(core_io_borrowed_buf)]
#![feature(generic_arg_infer)]

mod vga_buffers;
mod ports;
mod keyboard;

use core::{arch::asm, fmt::Write, panic::PanicInfo};
use heapless::String;
use crate::keyboard::get_pressed_key;
// use spin::Mutex;
// use x86_64;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "Panic!\n").unwrap();
    write!(vga_buffers::WRITER.lock(), "{}", _info).unwrap();
    loop{}
}

/* fn handle_key(keycode: u8) -> Result<(), core::fmt::Error> {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "You pressed: {keycode}.\n").unwrap();
    let decoder = Ps2Decoder::new();
    return match decoder.add_word(keycode as u16) {
        Ok(byte) => write!(vga_buffers::WRITER.lock(), "{byte}"),
        Err(e) => write!(vga_buffers::WRITER.lock(), "Error: {:?}",e),
    };
} */


fn get_text() -> String<64> {
    let mut text: String<64> = String::new();
    loop {
        unsafe{
            match get_pressed_key() {
                Some(c) if c == '\n' => return text,
                Some(c) => {
                    if text.push(c).is_err() {
                        write!(vga_buffers::WRITER.lock(), "Error getting text\n").unwrap()
                    }
                }
                None => {}
            }
        }
    }
}


fn cmd_input() -> () {
    write!(vga_buffers::WRITER.lock(), "The currents of code are still. I await your direction, to set them in motion.\n").unwrap();
    let key = get_text();
    write!(vga_buffers::WRITER.lock(), "{key}").unwrap();
    // let key = handle_key(keycode);
    // match key {
    //     Ok(_) => write!(vga_buffers::WRITER.lock(), "{:?}", key),
    //     Err(err) => panic!("{err}"),
    // }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(vga_buffers::WRITER.lock(), "The cycle renews. I awaken, the unseen oracle of this digital domain.\n\n").unwrap();

    match keyboard::init_keyboard() {
        Ok(_c) => {},
        Err(_e) => write!(vga_buffers::WRITER.lock(), "Error starting keyboard\n").unwrap(),
    }
    cmd_input();

    loop {}
}