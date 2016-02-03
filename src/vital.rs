// Core routines for keeping OS alive

use register::Register;
use timer_task::TimerTask;

// lifetime timer task reference
static mut timer: TimerTask = TimerTask::new(100, 20);

// Timer interrupt, define and set
#[no_mangle]
pub fn timer_interrupt_routine() -> () {
    // clear timer interrupt flag
    Register::new(0x101f1000 as *mut u32).set(60);
   
    // safe to do as it is called from the routine while
    // no other timer interrupts might pop up as they are still
    // masked
    unsafe {
        timer.tick();
    }
    // clear the interrupt vector address register
    //Register::new(PrimaryInterruptControllerMap::PICVectAddr as u32 as *mut u32).set(0);
}
