// implementation of the process

use arm1176;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    CREATED,
    RUNNING,
    PAUSED,
    STOPPED,
    READY,
}

#[derive(PartialEq)]
pub enum ProcessError {
    ProcessAlreadyRunning
}

#[derive(Copy, Clone)]
pub struct Process {
    quantum: i32,
    remaining: i32,
    registers: [u32; 16],
    state: State,
}

impl Process {
    pub fn new() -> Process {
        Process {
            quantum: 50,
            remaining: 50,
            state: State::CREATED,
            registers: [0; 16],
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

    #[no_mangle]
    pub fn save_context(&mut self, lr_irq: u32) -> () {
        // now, let's save all registers to PCB
        arm1176::save_context_to_stack(&mut self.registers);
    }

    pub fn restore_context(&mut self) -> () {
        arm1176::restore_context_from_stack(&mut self.registers);
    }
}
