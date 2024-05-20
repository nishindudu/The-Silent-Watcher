#![no_std]
#![no_main]
#![feature(portable_simd)]
#![feature(core_io_borrowed_buf)]

mod vga_buffers;
mod ports;
mod keyboard;

use core::{arch::asm, fmt::Error, panic::PanicInfo};
use spin::Mutex;
use x86_64;

use crate::keyboard::get_pressed_key;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::fmt::Write;
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



fn cmd_input() -> Option<char> {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "The currents of code are still. I await your direction, to set them in motion.\n").unwrap();
    unsafe {
        let key = get_pressed_key();
        // let key = handle_key(keycode);
        // match key {
        //     Ok(_) => write!(vga_buffers::WRITER.lock(), "{:?}", key),
        //     Err(err) => panic!("{err}"),
        // }
        match key {
            Some(c) => {
                write!(vga_buffers::WRITER.lock(), "Pressed {}\n", c).unwrap();
                Some(c)
            }
            None => {
                write!(vga_buffers::WRITER.lock(), "Error\n").unwrap();
                None
            }
        }
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "The cycle renews. I awaken, the unseen oracle of this digital domain.\n").unwrap();
    cmd_input();

    loop {}
}