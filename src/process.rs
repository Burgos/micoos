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
    pub fn new(process_body: fn() -> ()) -> Process {
        let mut p = Process {
            quantum: 50,
            remaining: 50,
            state: State::CREATED,
            registers: [0; 16],
        };

        // initialize link register
        p.registers[14] = (process_body as *const u32) as u32;

        // setup CPSR - interrupts enabled, user mode
        p.registers[15] = 0x10;

        p
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

    pub fn set_function_to_run (&mut self, function_to_run: fn() -> ()) -> Result<(), ProcessError> {
        if self.state != State::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.registers[14] = (function_to_run as *const u32) as u32;
        Ok(())
    }

    pub fn mark_process_ready (&mut self) -> Result<(), ProcessError> {
        if self.state != State::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.state = State::READY;
        Ok(())
    }


    pub fn set_stack_pointer (&mut self, number_of_processes: usize) -> Result<(), ProcessError> {
        if self.state != State::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.registers[13] = (0x10000 + (number_of_processes + 1) * 0x5000) as u32;
        Ok(())
    }
}
