// Core routines for keeping OS alive

use timer_task::TimerTask;
use timer_task::TickResult;
use register::Register;

pub struct Vital {
    timer_task: TimerTask,
}

impl Vital {
    pub const fn new () -> Vital {
        Vital {
            timer_task: TimerTask::new(0, 0, None)
        }
    }

    pub fn set_timer_task (&mut self, timer_task: TimerTask) {
        self.timer_task = timer_task;
    }
}



// Timer interrupt, define and set
#[no_mangle]
pub fn timer_interrupt_routine(vital_instance: &mut Vital, lr_irq: u32) -> u32 {
    // safe to do as it is called from the routine while
    // no other timer interrupts might pop up as they are still
    // masked
    unsafe {
        match vital_instance.timer_task.tick(lr_irq) {
            TickResult::CallMethod => {
                call_scheduled_task(lr_irq);
                lr_irq
            },
            _ => lr_irq
        }
    }
}

pub fn call_scheduled_task(value: u32) -> () {
    Register::new(0x101f1000 as *mut u32).set(0x30 + value);
}

