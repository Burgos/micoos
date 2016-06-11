// Defines timer task. This task consists of several pieces:
// 1. keep the tick from the boot
// 2. call every x ms registered handler
use screen;

pub struct TimerTask
{
    // elapsed ticks since boot
    elapsed_ticks: i32,

    // ticks per ms for the current processor
    ticks_per_ms: i32,

    // ticks under ms for the processor
    ticks_to_ms: i32,

    // ticks to call of repeated task
    until_next_call_ms: i32,
    
    // scheduled task frequency in ms.
    call_frequency_ms: i32,

    // scheduled task to be run
    scheduled_task: Option<fn(time_value: u32) -> ()>,
}


pub enum TickResult {
    CallMethod,
    MethodCalled,
    SkipCall
}

impl TimerTask
{
    // called every time the interrupt happens. 
    // Takes old task's lr and potentially returns
    // new task's lr
    pub fn tick(&mut self, value: u32) -> TickResult {
        self.elapsed_ticks = self.elapsed_ticks + 1;

        self.ticks_to_ms = self.ticks_to_ms - 1;
        if self.ticks_to_ms <= 0
        {
            self.ticks_to_ms = self.ticks_per_ms;
            self.until_next_call_ms = self.until_next_call_ms - 1;

            if self.until_next_call_ms <= 0
            {
                self.until_next_call_ms = self.call_frequency_ms;
                
                if cfg!(feature="log-timertask") {
                    kprint!("Calling scheduled task\n");
                }

                let result = match self.scheduled_task {
                    Some(ref method) => {
                        method(value);
                        TickResult::MethodCalled
                    },
                    _ => { TickResult::CallMethod }
                };

                return result;
            }
        }

        // if we don't need to call method, tell the caller
        TickResult::SkipCall
    }

    pub const fn new(ticks_per_ms: i32, call_frequency_ms: i32,
                     scheduled_task: Option<fn(timer_value: u32) -> ()>) -> TimerTask {
        TimerTask { 
                    elapsed_ticks: 0,
                    ticks_per_ms: ticks_per_ms,
                    call_frequency_ms: call_frequency_ms,
                    until_next_call_ms: call_frequency_ms,
                    ticks_to_ms: ticks_per_ms,
                    scheduled_task: scheduled_task
        }
    }

    // Schedules the method to be called right away
    pub fn call_now(&mut self) -> () {
        self.ticks_to_ms = 0;
        self.until_next_call_ms = 0;
    }
}
