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
use ascii::putchar;

pub struct Writer {
    column_pos: usize,
    color: u32,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte (&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT as u32 - 2 * CHAR_HEIGHT as u32;// BUFFER_HEIGHT as u32 - 1 * CHAR_HEIGHT as u32;
                let col = self.column_pos as u32;

                
                putchar(byte, col, row, self.color);

                self.column_pos = (self.column_pos as u32 + 4 * CHAR_WIDTH as u32) as usize;
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
    color: 0x00FF0000,
    buffer: unsafe { Unique::new((1024 * 1024) as *mut _) },
});

macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::screen::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
