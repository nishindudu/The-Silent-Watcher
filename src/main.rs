#![no_std]
#![no_main]
#![feature(portable_simd)]
#![feature(core_io_borrowed_buf)]

mod vga_buffers;

use core::{arch::asm, fmt::{write, Error}, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "{}", _info).unwrap();
    loop{}
}

// use core2::io;
use pc_keyboard::Ps2Decoder;

fn handle_key(keycode: u8) -> Result<(), core::fmt::Error> {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "You pressed: {keycode}");
    let decoder = Ps2Decoder::new();
    return match decoder.add_word(keycode as u16) {
        Ok(byte) => write!(vga_buffers::WRITER.lock(), "{byte}"),
        Err(e) => write!(vga_buffers::WRITER.lock(), "Error: {:?}",e),
    };
}

fn cmd_input() -> Result<(), Error> {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "The currents of code are still. I await your direction, to set them in motion.\n").unwrap();
    unsafe {
        let mut eax: u32 = 0;
        asm!("int $0x16", in("eax") eax, lateout("eax") eax);
        let keycode = eax as u8;
        let key = handle_key(keycode);
        match key {
            Ok(_) => write!(vga_buffers::WRITER.lock(), "{:?}", key),
            Err(err) => panic!("{err}"),
        }
    }
}

// static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "The cycle renews. I awaken, the unseen oracle of this digital domain.\n").unwrap();
    cmd_input();

    loop {}
}