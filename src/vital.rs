// Core routines for keeping OS alive

use timer_task::TimerTask;
use register::Register;
use scheduler;

// lifetime timer task reference
static mut timer: TimerTask = TimerTask::new(2, 1000, call_scheduled_task);

// Timer interrupt, define and set
#[no_mangle]
pub fn timer_interrupt_routine(lr_irq: u32) -> u32 {
    // safe to do as it is called from the routine while
    // no other timer interrupts might pop up as they are still
    // masked
    unsafe {
        timer.tick(lr_irq)
    }
}

pub fn call_scheduled_task(value: u32) -> () {
    Register::new(0x101f1000 as *mut u32).set(0x30 + value);
}

