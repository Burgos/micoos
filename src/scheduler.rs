// Scheduler implementation. Contains list of
// processes and select the next one on the
// context switch.

use process::Process;
use process::ProcessError;
use process::ProcessTickResult;

pub struct Scheduler {
    processes: [Process; 10],
    current_process: usize,
    number_of_processes: usize,
    first_process_started: bool
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            processes: [Process::new(dummy); 10],
            current_process: 0,
            number_of_processes: 0,
            first_process_started: false
        }
    }

    pub fn add_process(&mut self, function_to_run: fn() -> ()) -> Result<(), ProcessError> {
        try!(self.processes[self.number_of_processes].set_function_to_run(function_to_run));
        try!(self.processes[self.number_of_processes].set_stack_pointer(self.number_of_processes));
        try!(self.processes[self.number_of_processes].mark_process_ready());
        self.number_of_processes = self.number_of_processes + 1;
        Ok(())

    }

    pub fn schedule_next(&mut self) -> ()
    {
        if self.first_process_started {
            if self.processes[self.current_process].tick() == ProcessTickResult::Yield {
                self.processes[self.current_process].save_context();
                self.current_process = (self.current_process + 1) % self.number_of_processes;
                self.processes[self.current_process].restore_context();
            }
        }
        else {
            self.first_process_started = true;
            self.processes[self.current_process].restore_context();
        }
    }
}

// dummy process implementation
fn dummy() -> () {
    loop {}
}
