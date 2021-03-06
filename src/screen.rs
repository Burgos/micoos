// Code and interface taken from http://os.phil-opp.com/printing-to-screen.html
// Original code is under MIT license with copyright to Philipp Oppermann
// https://github.com/phil-opp/blog_os

#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
    pixel: u32
}

const BUFFER_WIDTH: usize  = 640;
const BUFFER_HEIGHT: usize = 480;
const CHAR_HEIGHT: u32 = 16;
const CHAR_WIDTH: u32  = 8;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

use core::fmt::Write;
use core::ptr::Unique;

pub struct Writer {
    column_pos: usize,
    color: u32,
    buffer: Unique<Buffer>,
}

macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::screen::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

impl Writer {
    pub fn write_byte (&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT as u32 - CHAR_HEIGHT as u32;// BUFFER_HEIGHT as u32 - 1 * CHAR_HEIGHT as u32;
                let col = self.column_pos as u32;

                let color = self.color; 
                putchar(self.buffer(), byte, col, row, color);

                self.column_pos = (self.column_pos as u32 + CHAR_WIDTH as u32) as usize;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() } 
    }

    fn new_line(&mut self) {
        for i in 0 .. CHAR_HEIGHT {
            for row in 0 as usize .. (BUFFER_HEIGHT as u32 - 1) as usize {
                let buffer = self.buffer();
                buffer.chars[row] = buffer.chars[row + 1];
            }
        }

        self.column_pos = 0;
        self.clear_row(BUFFER_HEIGHT);
    }

    fn clear_row(&mut self, row: usize) {
        let char = b' ';
        for i in 0 .. CHAR_HEIGHT {
            self.buffer().chars[(row as u32 - 1 - i) as usize] =
                [ScreenChar { pixel: 0 }; BUFFER_WIDTH];
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
    }
}

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        
        Ok(())
    }
}

pub fn print_something () {
    let mut writer = Writer {
        column_pos: 0,
        color: 0x00FFFFFF,
        buffer: unsafe { Unique::new((1024 * 1024) as *mut _) },
    };

//    writer.write_str("Mico ja te volim bas te volim, najvise na svetu!");
    write!(writer, "Brojevi su {}", 42);
}

use spin::Mutex;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_pos: 0,
    color: 0x00FFFFFF,
    buffer: unsafe { Unique::new((1024 * 1024) as *mut _) },
});

pub fn clear_screen() {
    for i in 0 .. BUFFER_HEIGHT {
        kprint!("\n");
    }
}

use ascii_font::FONT;

fn putchar (buffer: &mut Buffer, val: u8, x_offset: u32, y_offset: u32, color: u32) -> () {
    let char_index = val as usize;

    let bmp = FONT[char_index];

    // Sample for '0'
    // 00 00 00 00 18 24 42 42 42 42 42 42 24 18 00 00
    // 16 rows, two digits per row, 8 bits per row.

    // Iterate trough each row
    for j in 0 .. 16 {
        // Now we have 18. Convert it to value
        let value = (bmp[((j as u32 * 2) + 0) as usize]) * 16 + (bmp[((j as u32 * 2) + 1) as usize]);

        // and now for every bit in the value, show it.
        for i in 0 .. 8 {
            buffer.chars[(y_offset + j) as usize][(x_offset + i) as usize] = ScreenChar {
                pixel: ((value & (0x80 >> i)) >> (7 - i)) as u32 * color
            };
        }
    }
}
