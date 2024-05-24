use crate::ports;
// use pc_keyboard::{self, layouts, Keyboard, ScancodeSet1};

use spin::Mutex;
// use heapless::String;

pub fn init_keyboard() -> Result<u8, u8> {
    let mut retry_count: u8 = 0;

    loop {
        send_data_to_keyboard(Some(0xAA), None);
        // write!(vga_buffers::WRITER.lock(), "Writing 0xAA to keyboard\n").unwrap();

        if read_keyboard_data() == 0xFC {
            retry_count += 1;
            reset_keyboard();
        }
        if retry_count > 3 {
            break;
        }

        if read_keyboard_data() == 0x55 {
            set_keyboard_repeat();
            return Ok(0);
        }
    }
    Err(1)
}

fn set_keyboard_repeat() {
    send_data_to_keyboard(Some(0xAD), None);
    send_data_to_keyboard(None, Some(0xF3));
    send_data_to_keyboard(None, Some(0x5F));
}

fn reset_keyboard() -> () {
    {
        let mut retry_count: u8 = 0;
        loop {
            send_data_to_keyboard(Some(0xFF), None);
            // write!(vga_buffers::WRITER.lock(), "Writing 0xFF (reset) to keyboard\n").unwrap();

            if read_keyboard_data() == 0xFA {
                break;
            }
            retry_count += 1;
            if retry_count == 4 {
                break;
            }
        }
    }

    {
        let mut retry_count: u8 = 0;
        loop{
            send_data_to_keyboard(Some(0xF0), Some(1));
            if read_keyboard_data() == 0xFA {
                break;
            }
            retry_count += 1;
            if retry_count == 4{
                break;
            }
        }
    }
}

fn send_data_to_keyboard(command: Option<u8>, data: Option<u8>) {
    // write!(vga_buffers::WRITER.lock(), "Writing data to keyboard\n").unwrap();
    static COMMAND_PORT: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x64)
    });
    static DATA_PORT: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x60)
    });
    static STATUS_REG: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x64)
    });
    
    loop {
        let status = (STATUS_REG.lock().read() >> 1) & 1;
        if status == 0{
            {
                let mut keyboard = COMMAND_PORT.lock();
                match command {
                    Some(c) => {keyboard.write(c);},
                    None => {},
                }
            }

            {
                let mut data_port = DATA_PORT.lock();
                match data {
                    Some(c) => {data_port.write(c);},
                    None => {},
                }
            }
            break;
        }
    }
    // write!(vga_buffers::WRITER.lock(), "Writing data to keyboard completed\n").unwrap();
}

fn read_keyboard_data() -> u8 {
    static DATA_PORT: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x60)
    });
    DATA_PORT.lock().read()
}


pub unsafe fn get_pressed_key() -> Option<char> {
    static KEYBOARD: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x60)
    });
    let scancode = KEYBOARD.lock().read();
    let _keycode: char = match scancode_to_ascii(scancode) {
        Some(c) => return Some(c),
        None => {return None;},
    };
}


/*
pub unsafe fn get_keys() -> Option<char> {
    static KEYBOARD: Mutex<ports::Port<u8>> = Mutex::new(unsafe {
        ports::Port::new(0x60)
    });
    let mut text = String::new();
    loop {
        let scancode = KEYBOARD.lock().read();
        let ascii_val = scancode_to_ascii(scancode);
        if ascii_val.is_some() {
            match ascii_val {
                Some(c) => {text.push(c);},
                None => {},
            }
        }
    }
} */

pub unsafe fn scancode_to_ascii(scancode: u8) -> Option<char> {
    match scancode {
        0x10 => Some('q'),
        0x11 => Some('w'),
        0x12 => Some('e'),
        0x13 => Some('r'),
        0x14 => Some('t'),
        0x15 => Some('y'),
        0x16 => Some('u'),
        0x17 => Some('i'),
        0x18 => Some('o'),
        0x19 => Some('p'),
        0x1A => Some('['),
        0x1B => Some(']'),
        0x1C => Some('\n'),
        // 0x1D => Some(''), //Right control
        0x1E => Some('a'),
        0x1F => Some('s'),
        0x20 => Some('d'),
        0x21 => Some('f'),
        0x22 => Some('g'),
        0x23 => Some('h'),
        0x24 => Some('j'),
        0x25 => Some('k'),
        0x26 => Some('l'),
        0x27 => Some(';'),
        0x28 => Some('\''),
        0x29 => Some('`'),
        // 0x2A => Some(''), //Left shift
        0x2B => Some('\\'),
        0x2C => Some('z'),
        0x2D => Some('x'),
        0x2E => Some('c'),
        0x2F => Some('v'),
        0x30 => Some('b'),
        0x31 => Some('n'),
        0x32 => Some('m'),
        0x33 => Some(','),
        0x34 => Some('.'),
        0x35 => Some('/'),
        // 0x36 => Some(''), //Right shift
        0x37 => Some('*'), //keypad *
        // 0x38 => Some('') //Left alt
        0x39 => Some(' '),
        // 0x3A => Some('') //Caps Lock
        // Numkeys
        0x47 => Some('7'),
        0x48 => Some('8'),
        0x49 => Some('9'),
        0x4A => Some('-'),
        0x4B => Some('4'),
        0x4C => Some('5'),
        0x4D => Some('5'),
        0x4E => Some('6'),
        0x4F => Some('1'),
        0x50 => Some('2'),
        0x51 => Some('3'),
        0x52 => Some('0'),
        _ => {return None;},
    }
}