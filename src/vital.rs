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
use process::Process;

#[repr(C)] 
pub struct Vital {
    pub timer_task: TimerTask,
    pub scheduler: Scheduler,
    do_yield_process: bool
}

impl Vital {
    pub const fn new (scheduler: Scheduler) -> Vital {
        Vital {
            timer_task: TimerTask::new(0, 0, None),
            scheduler: scheduler,
            do_yield_process: false,
        }
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
        self.do_yield_process = true;
    }

    pub fn running_process_id (&mut self) -> u32 {
        self.scheduler.running_process_id()
    }
    
    pub fn running_process (&mut self) -> Option<&mut Process> {
        self.scheduler.running_process()
    }
}



// Timer interrupt, define and set
#[no_mangle]
pub fn timer_interrupt_routine(vital_instance: &mut Vital, value: u32) -> u32 {
    // safe to do as it is called from the routine while
    // no other timer interrupts might pop up as they are still
    // masked
    let mut vital = match VITAL.try_lock() {
        Some(vital) => vital,
        _ => return 0
    };

    let timer_res = vital.timer_task.tick(value);

    let yield_process = vital.do_yield_process;
    vital.do_yield_process = false;
    drop(vital);

    // yield process always overrides
    if yield_process {
        call_scheduled_task(0);
    }
    else {
        match timer_res {
            TickResult::CallMethod => {
                call_scheduled_task(0);
            },
            _ => ()
        }
    }

    0
}

pub fn call_scheduled_task(value: u32) -> () {
    let mut global_vital_instance = VITAL.lock();
    global_vital_instance.scheduler.schedule_next();
}


#[no_mangle]
pub fn swi_interrupt_routine (vital_instance: &mut Vital, code: u32, value_1: u32, value_2: u32) -> u32 {
    use swi;
    swi::handle(vital_instance, code, value_1, value_2)
}

use spin::Mutex;

pub static VITAL: Mutex<Vital> = Mutex::new(Vital {
        scheduler: Scheduler::new(),
        timer_task: TimerTask::new(2, 1000, None),
        do_yield_process: false
    });
