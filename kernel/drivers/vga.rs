const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct VGABuffer {
    chars: [[u16; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub fn initialize_vga() {
    clear_screen();
}

pub fn clear_screen() {
    for row in 0..BUFFER_HEIGHT {
        for col in 0..BUFFER_WIDTH {
            write_character(' ', row, col);
        }
    }
}

pub fn write_text(text: &str, row: usize, col: usize) {
    let mut col = col;
    for byte in text.bytes() {
        if col >= BUFFER_WIDTH {
            col = 0;
        }
        write_character(byte as char, row, col);
        col += 1;
    }
}

fn write_character(character: char, row: usize, col: usize) {
    let offset = row * BUFFER_WIDTH + col;
    unsafe {
        *VGA_BUFFER.offset(offset as isize * 2) = character as u8;
        *VGA_BUFFER.offset(offset as isize * 2 + 1) = 0x0F; // white on black
    }
}

pub fn draw_pixel(x: usize, y: usize, color: u8) {
    let buffer = VGA_BUFFER as *mut VGABuffer;
    unsafe {
        ptr::write_volatile(&mut (*buffer).chars[y][x], color as u16);
    }
}
