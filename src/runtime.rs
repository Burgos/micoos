#![no_std]
#![crate_type = "lib"]
#![feature(lang_items)]

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

