#![crate_type = "staticlib"]
#![no_std]
#![feature(lang_items)]
#![feature(core)]
#![feature(core_intrinsics)] 

pub use core::mem;

mod register;
use register::Register;

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


#[cold] #[inline(never)]
fn panic_bounds_check(file_line: &(&'static str, u32),
                     index: usize, len: usize) -> ! {
	loop {}
}


#[no_mangle]
pub fn kernel() -> () {
	let msg: &[u8] = unsafe { mem::transmute("MICO!") };
	let serial_port = Register::Register::new(0x101f1000 as *mut u8, 0);

	let serial: *mut u8 = 0x101f1000 as *mut u8;

	unsafe
	{
		for i in 0..5 {
			serial_port.set(msg[i])
		}
	}

	
}


