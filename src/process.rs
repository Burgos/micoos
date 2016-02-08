// implementation of the process

use arm1176;

enum State {
    CREATED,
    RUNNING,
    PAUSED,
    STOPPED
}

pub struct Process {
    quantum: i32,
    remaining: i32,
    stack_pointer: u32,
    link_register: u32,
    spsr: u32,
    state: State
}

impl Process {
    pub fn new() -> Process {
        Process {
            quantum: 50,
            remaining: 50,
            stack_pointer: 0,
            link_register: 0,
            spsr: 0,
            state: State::CREATED
        }
    }

    pub fn run(&self) -> () {
        self.init();
        self.run_task();
    }

    fn init(&self) -> () {
        // copy a stack etc here
    }

    pub fn run_task(&self) -> () {

    }

    pub fn save_context(&mut self, lr_irq: u32) -> () {
        // next time this task is selected, return
        // from irq at the point before context switch        
        self.link_register = lr_irq;

        arm1176::save_spsr_to_process(&mut self.spsr);
        // all processes are executing in the user mode
        // so, we might wanna switch to sys mode,
        // interrupts still disabled. At this point we should
        // be in irq or supervisor mode. We're jumping to sys
        // mode as all registers are actually the same as in user mode
        arm1176::switch_to_sys_mode(arm1176::InterruptType::DISABLED);

        // now, let's save all registers to user stack
        arm1176::save_context_to_stack();

        // and let's save sp to stack pointer field
        arm1176::save_sp_to_process(&mut self.stack_pointer);

        arm1176::switch_to_irq_mode(arm1176::InterruptType::DISABLED);
    }

    pub fn restore_context(&mut self) -> () {

    }
}
