#![no_std]
#![no_main]
#![feature(portable_simd)]

mod vga_buffers;

use core::{panic::PanicInfo, simd::usizex4};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::fmt::Write;
    write!(vga_buffers::WRITER.lock(), "{}", _info).unwrap();
    loop{}
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
    write!(vga_buffers::WRITER.lock(), ", some numbers: {} {} ", 42, 1.337).unwrap();

    loop {}
}