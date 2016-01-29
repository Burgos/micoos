#![crate_type = "staticlib"]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_intrinsics)] 

pub use core::mem;

pub mod register;
pub mod arm1176;

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
	let msg: &[u8] = unsafe { mem::transmute("MICO!") };
	let serial_port = register::Register::new(0x101f1000 as *mut u8);

    arm1176::enable_timer_interrupt();

	for i in 0..5 {
		serial_port.set(msg[i])
	}
}


