// This module contains various constants that are
// platform specific for arm1176

use register::Register;
use vital;
use vital::Vital;
use core::mem;

pub struct Character {
    pixels: [[u8; 8]; 8]
}

static Ascii: &'static [Character; 2] = &[ 
    Character {
        pixels:  [
                     [ 1, 1, 1, 1, 1, 1, 1, 1 ],
                     [ 1, 1, 0, 0, 0, 0, 1, 1 ],
                     [ 1, 1, 0, 0, 0, 0, 1, 1 ],
                     [ 1, 1, 0, 0, 0, 0, 1, 1 ],
                     [ 1, 1, 0, 0, 0, 0, 1, 1 ],
                     [ 1, 1, 0, 0, 0, 0, 1, 1 ],
                     [ 1, 1, 0, 0, 0, 0, 1, 1 ],
                     [ 1, 1, 1, 1, 1, 1, 1, 1 ]
                  ]
    },
    Character {
        pixels:  [
                     [ 0, 0, 1, 1, 1, 0, 0, 0 ],
                     [ 0, 1, 1, 1, 1, 0, 0, 0 ],
                     [ 0, 1, 1, 1, 1, 0, 0, 0 ],
                     [ 0, 0, 0, 1, 1, 0, 0, 0 ],
                     [ 0, 0, 0, 1, 1, 0, 0, 0 ],
                     [ 0, 0, 0, 1, 1, 0, 0, 0 ],
                     [ 0, 0, 0, 1, 1, 0, 0, 0 ],
                     [ 0, 0, 0, 1, 1, 0, 0, 0 ]
                  ]
    },
];

pub fn putchar (val: char, x_offset: u32, y_offset: u32, color: u32) -> () {
    let char_index = (val as u8 - '0' as u8) as usize;

    let addr = ((1024 * 1024) + ((y_offset * 640 * 4) + x_offset)) as *mut u32;

    for i in 0 .. 8 {
        for j in 0 .. 8 {
            Register::new((addr as u32 + j * 640 * 4 + i * 4) as *mut u32).set(Ascii[char_index].pixels[j as usize][i as usize] as u32 * color);
        }
    }
}
