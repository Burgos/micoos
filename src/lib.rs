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

#[no_mangle]
pub fn kernel() -> () {
    arm1176::enable_timer_interrupt();
}
