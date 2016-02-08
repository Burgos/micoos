// Core routines for keeping OS alive

use timer_task::TimerTask;

// lifetime timer task reference
static mut timer: TimerTask = TimerTask::new(2, 1000);

// Timer interrupt, define and set
#[no_mangle]
pub fn timer_interrupt_routine(lr_irq: u32) -> () {
    // safe to do as it is called from the routine while
    // no other timer interrupts might pop up as they are still
    // masked
    unsafe {
        timer.tick(lr_irq);
    }
}
