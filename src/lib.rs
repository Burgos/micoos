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
pub mod msgbox;


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
pub unsafe fn __aeabi_unwind_cpp_pr1() -> ()
{
    loop {}
}

#[no_mangle]
pub fn kernel() -> () {
    use timer_task::TimerTask;
    use vital::Vital;
    use scheduler::Scheduler;
    use register::Register;
    
    let mut scheduler =  {
        let mut scheduler = Scheduler::new();
        scheduler
    };

    let mut vital_instance: Vital = Vital::new(&mut scheduler);
    vital_instance.register_to_scheduler();
    vital_instance.scheduler.add_process(process_1, 1);
    vital_instance.scheduler.add_process(process_2, 3);
    vital_instance.scheduler.add_process(process_3, 1);



    let timer_task = TimerTask::new(2, 1000, None);
    vital_instance.set_timer_task(timer_task);
    arm1176::set_vital_instance(&vital_instance);

    arm1176::enable_timer_interrupt();

    loop {
        arm1176::wfe();
    }
}

pub fn process_1() -> () {
    use register::Register;
    let serial = Register::new(0x101f1000 as *mut u32);
    loop {
        for x in 0 .. 10 {
            serial.set('A' as u32);
            serial.set(0x30 + x);
        }
    }
}

pub fn process_2() -> () {
    use register::Register;
    let serial = Register::new(0x101f1000 as *mut u32);
    loop {
        for x in 0 .. 10 {
            serial.set('B' as u32);
            serial.set(0x30 + x);
        }
    }
}

pub fn process_3() -> () {
    use register::Register;
    let serial = Register::new(0x101f1000 as *mut u32);
    loop {
        for x in 0 .. 10 {
            serial.set('C' as u32);
            serial.set(0x30 + x);
        }
    }
}
