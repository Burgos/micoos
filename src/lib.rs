#![crate_type = "staticlib"]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]
#![feature(core_intrinsics)] 
#![feature(const_fn)]
#![feature(unique)]

extern crate spin;

pub use core::mem;

#[macro_use]
pub mod screen;
pub mod register;
pub mod arm1176;
pub mod timer_task;
pub mod vital;
pub mod process;
pub mod scheduler;
pub mod msgbox;
pub mod swi;
pub mod system_calls;
pub mod ascii_font;

#[lang="eh_personality"] extern fn eh_personality() {}
#[no_mangle]
#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !
{
    loop { }
}
pub extern "C" fn rust_begin_unwind(_args : &core::fmt::Arguments, _file : &str, _line : u32) -> !
{
    loop {}
}

#[no_mangle]
pub extern fn _Unwind_Resume() -> ()
{
    loop {}
}


#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr1() -> ()
{
    loop {}
}


#[no_mangle]
pub fn kernel() -> () {
    use vital::VITAL;
    
    arm1176::initialize_screen();

    {
        let mut vital_instance = VITAL.lock();
        vital_instance.scheduler.add_process(process_1, 1);
        vital_instance.scheduler.add_process(process_2, 3);
        vital_instance.scheduler.add_process(process_3, 9);
    } // to drop the spinlock

    kprint!("Before enabling timer interrupt\n");

    arm1176::enable_timer_interrupt();
    loop {
        arm1176::wfe();
    }
}

pub fn process_1() -> () {
    use system_calls::*;
    use register::Register;
    let serial = Register::new(0x101f1000 as *mut u32);
    loop {
        for x in 0 .. 10 {
            serial.set('A' as u32);
            let process_id = sys_get_process_id();;
            serial.set(0x30 + process_id);

            if x == 9 {
                sys_send_message_to_process(3, '0' as u32);
                process::Process::yield_process();
            }
        }
    }
}

pub fn process_2() -> () {
    use system_calls::*;
    use register::Register;
    let serial = Register::new(0x101f1000 as *mut u32);
    loop {
        for x in 0 .. 10 {
            process::Process::yield_process();
            serial.set('B' as u32);
            serial.set(0x30 + x);
        }
    }
}

pub fn process_3() -> () {
    use system_calls::*;
    use register::Register;
    let serial = Register::new(0x101f1000 as *mut u32);
    loop {
        for x in 0 .. 10 {
            process::Process::yield_process();
            serial.set('C' as u32);
            let message = process::Process::receive_message();
            let process_id = sys_get_process_id();
            serial.set(message.body[0] + process_id);
        }
    }
}
