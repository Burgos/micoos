// Core routines for keeping OS alive

use timer_task::TimerTask;
use timer_task::TickResult;
use register::Register;
use scheduler::Scheduler;
use msgbox::Message;
use msgbox::MessageBox;
use msgbox::MessageBoxResult;
use core::mem;
use swi::*;

pub struct Vital<'a> {
    pub timer_task: TimerTask,
    pub scheduler: &'a mut Scheduler<'a>,
    pub swi_handler: SoftwareInterruptHandler<'a>
}

impl<'a> Vital<'a> {
    pub const fn new (scheduler: &'a mut Scheduler<'a>) -> Vital<'a> {
        Vital {
            timer_task: TimerTask::new(0, 0, None),
            scheduler: scheduler,
            swi_handler: SoftwareInterruptHandler::new(None),
        }
    }

    pub fn register_to_scheduler (&mut self) -> () {
        let address = {
            unsafe { mem::transmute(&self) }
        };

        self.scheduler.set_vital_instance(address);
        self.swi_handler.set_vital_instance(address);
    }

    pub fn set_timer_task (&mut self, timer_task: TimerTask) {
        self.timer_task = timer_task;
        self.timer_task.call_now();
    }

    pub fn send_message_to_process (&mut self, process_id: usize, msg: Message)
            -> Result<(), MessageBoxResult> {
        let mut process = self.scheduler.get_process_by_id(process_id);
        
        match process {
            Some(p) => p.send_message(msg),
            None => return Err(MessageBoxResult::NoSuchProcess)
        }
    }

    pub fn yield_process (&mut self) -> () {
        self.scheduler.schedule_next();
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


#[no_mangle]
pub fn swi_interrupt_routine (vital_instance: &mut Vital, code: u32) -> u32 {
    // calls swf
    0
}
