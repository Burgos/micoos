// implementation of the process

use arm1176;
use msgbox::MessageBox;
use msgbox::MessageBoxResult;
use msgbox::Message;
use vital::Vital;


// Describes in which particular state
// the process is
#[derive(Copy, Clone, PartialEq)]
pub enum ProcessState {
    CREATED,
    RUNNING,
    PAUSED,
    STOPPED,
    BLOCKED,
    READY,
}

// Describes the error when dealing the
// process operation
#[derive(PartialEq)]
pub enum ProcessError {
    ProcessAlreadyRunning
}


// Process structure
#[derive(Copy, Clone)]
pub struct Process<'a> {
    quantum: i32,
    remaining: i32,
    registers: [u32; 17],
    state: ProcessState,
    process_body: fn() -> (),
    msgbox: MessageBox,
    vital: *const Vital<'a>
}

// Describes the result of the tick method -
// should the process yield or not.
#[derive(PartialEq)]
pub enum ProcessTickResult {
    Yield,
    DontYield
}

// Wrapper around the process keeeping it from
// executing arbitrar code
fn process_runner(process_body: fn() -> ()) {
        process_body();

        // Don't let it go away
        loop {}
}

impl<'a> Process<'a> {

    // Process constructor
    pub fn new(quantum: i32, process_body: fn() -> (), vital: *const Vital<'a>) -> Process<'a> {
        let mut p = Process {
            quantum: quantum,
            remaining: quantum,
            state: ProcessState::CREATED,
            registers: [0; 17],
            process_body: process_body,
            msgbox: MessageBox::new(),
            vital: vital
        };

        // initialize link register
        p.registers[0]  = (process_body as *const u32) as u32;
        p.registers[15] = (process_runner as *const u32) as u32;

        // setup CPSR - interrupts enabled, user mode
        p.registers[16] = 0x10;

        p
    }

    // Saves the process context
    #[no_mangle]
    pub fn save_context(&mut self) -> () {
        // now, let's save all registers to PCB
        arm1176::save_context_to_stack(&mut self.registers);
    }

    // Restores the process context
    pub fn restore_context(&mut self) -> () {
        arm1176::restore_context_from_stack(&mut self.registers);
    }

    // Sends message to the process
    pub fn send_message(&mut self, msg: Message) -> Result<(), MessageBoxResult> {
        self.msgbox.send_message(msg)
    }

    // Sets the runner function on process
    pub fn set_function_to_run (&mut self, function_to_run: fn() -> ()) -> Result<(), ProcessError> {
        if self.state != ProcessState::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.registers[15] = (function_to_run as *const u32) as u32;
        Ok(())
    }

    // Marks the process ready to be executed
    pub fn mark_process_ready (&mut self) -> Result<(), ProcessError> {
        if self.state != ProcessState::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.state = ProcessState::READY;
        Ok(())
    }

    // TODO
    pub fn yield_process (&self) -> () {
        let ref vital = unsafe { &*self.vital };
    }

    // Set's the process' stack
    pub fn set_stack_pointer (&mut self, number_of_processes: usize) -> Result<(), ProcessError> {
        if self.state != ProcessState::CREATED {
            return Err(ProcessError::ProcessAlreadyRunning);
        }

        self.registers[13] = (0x10000 + (number_of_processes + 1) * 0x5000) as u32;
        Ok(())
    }

    // Sets the time quantum
    pub fn set_time_quantum (&mut self, quantum: i32) -> Result<(), ProcessError> {
        self.quantum = quantum;
        Ok(())
    }

    // Sends a tick singal to the process and returns
    // the indicator if the process should change
    pub fn tick (&mut self) -> ProcessTickResult
    {
        self.remaining = self.remaining - 1;
        if self.remaining == 0 {
            self.remaining = self.quantum;
            return ProcessTickResult::Yield
        }

        ProcessTickResult::DontYield
    }

    // Indicates if the process is ready
    pub fn is_process_ready (&self) -> bool {
        self.state != ProcessState::BLOCKED &&
            self.state != ProcessState::STOPPED
    }


    // Receives message sent to the process
    pub fn receive_message(&mut self, msg: Message) -> &Message {
        loop {
            match (self.msgbox.is_empty()) {
                true => return self.msgbox.get_next_unread(),
                false => self.yield_process()
            }
        }
    }
}

