// implementation of the process

use arm1176;
use msgbox::MessageBox;

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessState {
    CREATED,
    RUNNING,
    PAUSED,
    STOPPED,
    BLOCKED,
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
    registers: [u32; 17],
    state: ProcessState,
    process_body: fn() -> (),
    msgbox: MessageBox
}

#[derive(PartialEq)]
pub enum ProcessTickResult {
    Yield,
    DontYield
}

fn process_runner(process_body: fn() -> ()) {
        process_body();

        // Don't let it go away
        loop {}
}

impl Process {
    pub fn new(quantum: i32, process_body: fn() -> ()) -> Process {
        let mut p = Process {
            quantum: quantum,
            remaining: quantum,
            state: ProcessState::CREATED,
            registers: [0; 17],
            process_body: process_body,
            msgbox: MessageBox::new()
        };

        // initialize link register
        p.registers[0]  = (process_body as *const u32) as u32;
        p.registers[15] = (process_runner as *const u32) as u32;

        // setup CPSR - interrupts enabled, user mode
        p.registers[16] = 0x10;

        p
    }

    #[no_mangle]
    pub fn save_context(&mut self) -> () {
        // now, let's save all registers to PCB
        arm1176::save_context_to_stack(&mut self.registers);
    }

    pub fn restore_context(&mut self) -> () {
        arm1176::restore_context_from_stack(&mut self.registers);
    }

    pub fn set_function_to_run (&mut self, function_to_run: fn() -> ()) -> Result<(), ProcessError> {
        if self.state != ProcessState::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.registers[15] = (function_to_run as *const u32) as u32;
        Ok(())
    }

    pub fn mark_process_ready (&mut self) -> Result<(), ProcessError> {
        if self.state != ProcessState::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.state = ProcessState::READY;
        Ok(())
    }


    pub fn set_stack_pointer (&mut self, number_of_processes: usize) -> Result<(), ProcessError> {
        if self.state != ProcessState::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.registers[13] = (0x10000 + (number_of_processes + 1) * 0x5000) as u32;
        Ok(())
    }

    pub fn set_time_quantum (&mut self, quantum: i32) -> Result<(), ProcessError> {
        self.quantum = quantum;
        Ok(())
    }

    pub fn tick (&mut self) -> ProcessTickResult
    {
        self.remaining = self.remaining - 1;
        if self.remaining == 0 {
            self.remaining = self.quantum;
            return ProcessTickResult::Yield
        }

        ProcessTickResult::DontYield
    }

    pub fn is_process_ready (&self) -> bool {
        self.state != ProcessState::BLOCKED &&
            self.state != ProcessState::STOPPED
    }
}
