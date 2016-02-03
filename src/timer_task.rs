// Defines timer task. This task consists of several pieces:
// 1. keep the tick from the boot
// 2. call every x ms registered handler

pub struct TimerTask
{
    // elapsed ticks since boot
    elapsed_ticks: i32,

    // ticks per ms for the current processor
    ticks_per_ms: i32,
}

impl TimerTask
{
    // called every time the interrupt happens.
    pub fn tick(&mut self) -> () {
        self.elapsed_ticks = self.elapsed_ticks + 1;
    }

    pub const fn new(ticks_per_ms: i32) -> TimerTask {
        TimerTask { elapsed_ticks: 0,
                    ticks_per_ms: ticks_per_ms 
        }
    }
}
