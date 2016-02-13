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
    use timer_task::TimerTask;
    use vital::Vital;
    use scheduler::Scheduler;
    
    let scheduler =  {
        let mut scheduler = Scheduler::new();
        scheduler.add_process(print_stuff);
        scheduler
    };

    let mut vital_instance: Vital = Vital::new();
    let timer_task = TimerTask::new(2, 1000, vital::call_scheduled_task);
    vital_instance.set_timer_task(timer_task);
    arm1176::set_vital_instance(&vital_instance);

    arm1176::enable_timer_interrupt();

    loop {
        arm1176::wfe();
    }
}

pub fn print_stuff() -> () {
    use register::Register;
    Register::new(0x101f1000 as *mut u32).set(0x30 + 2);
}
