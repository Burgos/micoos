// Scheduler implementation. Contains list of
// processes and select the next one on the
// context switch.

use process::Process;
use process::ProcessError;

pub struct Scheduler {
    processes: [Process; 10],
    current_process: usize,
    number_of_processes: usize
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            processes: [Process::new(dummy); 10],
            current_process: 0,
            number_of_processes: 0
        }
    }

    pub fn add_process(&mut self, function_to_run: fn() -> ()) -> Result<(), ProcessError> {
        try!(self.processes[self.number_of_processes].set_function_to_run(function_to_run));
        try!(self.processes[self.number_of_processes].mark_process_ready());
        try!(self.processes[self.number_of_processes].set_stack_pointer(self.number_of_processes));
        self.number_of_processes = self.number_of_processes + 1;
        Ok(())
    }

    pub fn schedule_next(&mut self) -> ()
    {
        self.current_process = (self.current_process + 1) % self.number_of_processes;
        self.processes[self.current_process].restore_context()
    }
}

// dummy process implementation
fn dummy() -> () {
    loop {}
}
