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
            processes: [Process::new(1, idle); 10],
            current_process: 0,
            number_of_processes: 0,
            first_process_started: false,
        }
    }

    pub fn add_process(&mut self, function_to_run: fn() -> (), quantum: i32) -> Result<(), ProcessError> {
        try!(self.processes[self.number_of_processes + 1].set_function_to_run(function_to_run));
        try!(self.processes[self.number_of_processes + 1].set_stack_pointer(self.number_of_processes));
        try!(self.processes[self.number_of_processes + 1].set_time_quantum(quantum));
        try!(self.processes[self.number_of_processes + 1].mark_process_ready());
        self.number_of_processes = self.number_of_processes + 1;
        Ok(())

    }

    pub fn schedule_next(&mut self) -> ()
    {
        if self.first_process_started {
            if self.processes[self.current_process].tick() == ProcessTickResult::Yield {
                self.processes[self.current_process].save_context();
                self.pick_next_process();
                self.processes[self.current_process].restore_context();
            }
        }
        else {
            self.first_process_started = true;
            self.pick_next_process();
            self.processes[self.current_process].restore_context();
        }
    }

    fn pick_next_process(&mut self) -> usize {
        let previous_process = self.current_process;
        let next_process = {
            loop {
                let mut process = (self.current_process + 1) % self.number_of_processes;

                if process == previous_process {
                    // pick idle task
                    self.current_process = 0;
                    return 0;
                }

                if self.processes[process + 1].is_process_ready() { 
                    self.current_process = process + 1;
                    return process + 1;
                }
            }
        };

        next_process
    }
}

// idle process implementation
fn idle() -> () {
    loop {}
}
