#![crate_type = "staticlib"]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_intrinsics)] 
#![feature(const_fn)]

pub use core::mem;

pub mod register;
pub mod arm1176;
pub mod timer_task;
pub mod vital;
pub mod process;
pub mod scheduler;


#[lang="stack_exhausted"] extern fn stack_exhausted() {}
#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop { }
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

// http://stackoverflow.com/questions/31494087/linking-rust-with-c-undefined-reference-to-aeabi-functions
#[no_mangle]
pub unsafe extern fn __aeabi_memclr4(s: *mut u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = 0u8;
        i += 1;
    }
    return s;
} 

#[no_mangle]
pub unsafe extern fn __aeabi_memclr8(s: *mut u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = 0u8;
        i += 1;
    }
    return s;
} 

#[no_mangle]
pub unsafe extern fn __aeabi_memclr(s: *mut u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = 0u8;
        i += 1;
    }
    return s;
} 

#[no_mangle]
pub fn kernel() -> () {
    arm1176::enable_timer_interrupt();
}
