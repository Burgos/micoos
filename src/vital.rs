// Core routines for keeping OS alive

use timer_task::TimerTask;
use timer_task::TickResult;
use register::Register;
use scheduler::Scheduler;

pub struct Vital {
    pub timer_task: TimerTask,
    pub scheduler: Scheduler,
}

impl Vital {
    pub const fn new (scheduler: Scheduler) -> Vital {
        Vital {
            timer_task: TimerTask::new(0, 0, None),
            scheduler: scheduler
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
                call_scheduled_task(vital_instance, lr_irq);
                lr_irq
            },
            _ => lr_irq
        }
    }
}

pub fn call_scheduled_task(vital_instance: &mut Vital, value: u32) -> () {
    vital_instance.scheduler.schedule_next();
}

