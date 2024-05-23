use core::{arch::asm, marker::PhantomData};

// use crate::vga_buffers;

#[allow(dead_code)]

// use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};

/* unsafe fn outb(value: u8, port: u16) {
    asm!("outb %al, %dx" ::
        "{dx}"(port), "{al}"(value) ::
        "volatile");
} */

pub unsafe fn outb(value: u8, port: u16) {
    // asm!("outb %al, %dx", in("al") value, in("dx") port);
    asm!("out dx, al", in("al") value, in("dx") port);
    // asm!("out %al, %dx", in("al") value, in("dx") port);
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    // write!(vga_buffers::WRITER.lock(), "Getting data from port\n").unwrap();
    asm!("in al, dx", out("al") result, in("dx") port);
    // write!(vga_buffers::WRITER.lock(), "Got data from port\n").unwrap();
    result
}

pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("inw %dx, %ax", in("dx") port, out("ax") result);
    result
}

pub unsafe fn outw(value: u16, port: u16) {
    asm!("outw %ax, %dw", in("ax") value, in("dx") port);
}

pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!("inl %dx, %eax", in("dx") port, out("eax") result);
    result
}

pub unsafe fn outl(value: u32, port: u16) {
    asm!("outl %eax, %dx", in("eax") value, in("dx") port);
}

pub struct Port<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    pub fn write(&mut self, value: T){
        unsafe { T::port_out(self.port, value) }
    }
}

pub trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 { inb(port) }
    unsafe fn port_out(port: u16, value: u8) { outb(value, port); }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 { inw(port) }
    unsafe fn port_out(port: u16, value: u16) { outw(value, port); }
}
impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 { inl(port) }
    unsafe fn port_out(port: u16, value: u32) { outl(value, port); }
}