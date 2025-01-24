use core::ptr;

const KEYBOARD_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;
const KEYBOARD_BUFFER_SIZE: usize = 1024;

static mut KEYBOARD_BUFFER: [u8; KEYBOARD_BUFFER_SIZE] = [0; KEYBOARD_BUFFER_SIZE];
static mut READ_INDEX: usize = 0;
static mut WRITE_INDEX: usize = 0;

pub fn initialize_keyboard() {
    unsafe {
        let port: u16 = 0x21;
        let mask = 0xFD;
        asm!(
            "in al, dx",
            "and al, {0}",
            "out dx, al",
            in(reg) mask,
            in("dx") port,
            out("al") _
        );
    }
}

pub fn read_key() -> Option<u8> {
    unsafe {
        if READ_INDEX == WRITE_INDEX {
            None
        } else {
            let key = KEYBOARD_BUFFER[READ_INDEX];
            READ_INDEX = (READ_INDEX + 1) % KEYBOARD_BUFFER_SIZE;
            Some(key)
        }
    }
}

pub fn keyboard_interrupt_handler() {
    unsafe {
        let mut status: u8;
        asm!(
            "in al, dx",
            in("dx") KEYBOARD_STATUS_PORT,
            out("al") status,
        );

        if status & 0x01 != 0 {
            let mut keycode: u8;
            asm!(
                "in al, dx",
                in("dx") KEYBOARD_PORT,
                out("al") keycode,
            );

            KEYBOARD_BUFFER[WRITE_INDEX] = keycode;
            WRITE_INDEX = (WRITE_INDEX + 1) % KEYBOARD_BUFFER_SIZE;
        }
    }
}
