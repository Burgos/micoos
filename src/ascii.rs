// This module contains various constants that are
// platform specific for arm1176

use register::Register;
use vital;
use vital::Vital;
use core::mem;
use ascii_font::FONT;


pub fn putchar (val: u8, x_offset: u32, y_offset: u32, color: u32) -> () {
    let char_index = val as usize;

    let bmp = FONT[char_index];

    // Sample for '0'
    // 00 00 00 00 18 24 42 42 42 42 42 42 24 18 00 00
    // 16 rows, two digits per row, 8 bits per row.

    let addr = ((1024 * 1024) + ((y_offset * 640 * 4) + x_offset)) as *mut u32;

    // Iterate trough each row
    for j in 0 .. 16 {
        // Now we have 18. Convert it to value
        let value = (bmp[((j as u32 * 2) + 0) as usize]) * 16 + (bmp[((j as u32 * 2) + 1) as usize]);

        // and now for every bit in the value, show it.
        for i in 0 .. 8 {
            Register::new((addr as u32 + j as u32 * 640 * 4 + i * 4) as *mut u32).set(((value & (0x80 >> i)) >> (7 - i)) as u32 * color);
        }
    }
    
    /*
    for i in 0 .. 4 {
        for j in 0 .. 8 {
            let c = bmp[(i + j * 4) as usize];
            // Now we have the v
            Register::new((addr as u32 + j * 640 * 4 + i * 4) as *mut u32).set(Ascii[char_index].pixels[j as usize][i as usize] as u32 * color);
        }
    }
    */
}
