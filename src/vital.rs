// Core routines for keeping OS alive

use timer_task::TimerTask;
use timer_task::TickResult;
use register::Register;
use scheduler::Scheduler;
use msgbox::Message;
use msgbox::MessageBox;
use msgbox::MessageBoxResult;

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

    pub fn send_message_to_process (&mut self, process_id: usize, msg: Message)
            -> Result<(), MessageBoxResult> {
        self.scheduler.get_process_by_id(process_id).send_message(msg)
    }
}



// Timer interrupt, define and set
#[no_mangle]
pub fn timer_interrupt_routine(vital_instance: &mut Vital, value: u32) -> u32 {
    // safe to do as it is called from the routine while
    // no other timer interrupts might pop up as they are still
    // masked
    match vital_instance.timer_task.tick(value) {
        TickResult::CallMethod => {
            call_scheduled_task(vital_instance, 0);
        },
        _ => ()
    }

    0
}

pub fn call_scheduled_task(vital_instance: &mut Vital, value: u32) -> () {
    vital_instance.scheduler.schedule_next();
}

