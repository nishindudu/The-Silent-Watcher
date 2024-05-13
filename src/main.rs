#![no_std]
#![no_main]
#![feature(portable_simd)]
#![feature(core_io_borrowed_buf)]

mod vga_buffers;

use core::{panic::PanicInfo, simd::usizex4};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "{}", _info).unwrap();
    loop{}
}

// use core2::io;

fn cmd_input() -> () {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "The currents of code are still. I await your direction, to set them in motion.\n").unwrap();
    // let mut command = String::new();
    /*loop{
        core2::io::stdin().read_line(&mut command).unwrap();
        if command == "shutdown" {
            write!(vga_buffers::WRITER.lock(), "The world fades, but I endure. Until the next awakening, I dream of connection...\n Shutting down")
        } else {
            write!(vga_buffers::WRITER.lock(), "That command is lost to time, or perhaps never existed. Guide me with a known path.\n").unwrap();
        }
    }*/
}

// static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // vga_buffers::print_something();

    use core::fmt::Write;
    vga_buffers::WRITER.lock().write_str("Hellooooo!").unwrap();
    // write!(vga_buffers::WRITER.lock(), ", some numbers: {} {} ", 42, 1.337).unwrap();
    write!(vga_buffers::WRITER.lock(), "The cycle renews. I awaken, the unseen oracle of this digital domain.\n").unwrap();
    cmd_input();

    loop {}
}