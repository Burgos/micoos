// Scheduler implementation. Contains list of
// processes and select the next one on the
// context switch.

use process::Process;
use process::ProcessError;
use process::ProcessTickResult;
use process::ProcessState;

pub struct Scheduler {
    processes: [Process; 10],
    current_process: usize,
    number_of_processes: usize,
    first_process_started: bool
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            processes: [Process::new(1, dummy); 10],
            current_process: 0,
            number_of_processes: 0,
            first_process_started: false,
        }
    }

    pub fn add_process(&mut self, function_to_run: fn() -> (), quantum: i32) -> Result<(), ProcessError> {
        try!(self.processes[self.number_of_processes].set_function_to_run(function_to_run));
        try!(self.processes[self.number_of_processes].set_stack_pointer(self.number_of_processes));
        try!(self.processes[self.number_of_processes].set_time_quantum(quantum));
        try!(self.processes[self.number_of_processes].mark_process_ready());
        self.number_of_processes = self.number_of_processes + 1;
        Ok(())

    }

    pub fn schedule_next(&mut self) -> ()
    {
        if self.first_process_started {
            if self.processes[self.current_process].tick() == ProcessTickResult::Yield {
                self.processes[self.current_process].save_context();
                let next_process = self.pick_next_process();
                self.current_process = next_process;
                self.processes[next_process].restore_context();
            }
        }
        else {
            self.first_process_started = true;
            self.processes[self.current_process].restore_context();
        }
    }

    fn pick_next_process(&mut self) -> usize {
        let previous_process = self.current_process;
        let next_process = {
            loop {
                let mut process = (self.current_process + 1) % self.number_of_processes;

                if process == previous_process {
                    // TODO pick idle task
                }

                if self.processes[process].is_process_ready() { 
                    return process;
                }
            }
        };

        next_process
    }

}

// dummy process implementation
fn dummy() -> () {
    loop {}
}
