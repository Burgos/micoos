// Scheduler implementation. Contains list of
// processes and select the next one on the
// context switch.

use process::Process;
use process::ProcessError;
use process::ProcessTickResult;
use process::ProcessState;

pub struct Scheduler {
    processes: [Option<Process>; 10],
    current_process: usize,
    number_of_processes: usize,
    first_process_started: bool
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            processes: [None; 10],
            current_process: 0,
            number_of_processes: 0,
            first_process_started: false,
        }
    }

    pub fn get_process_by_id (&mut self, id: usize) -> Option<&mut Process> {
        match self.processes[id] {
            Some(_) =>
                self.processes[id].as_mut(),
            None => None
        }
    }

    pub fn add_process(&mut self, function_to_run: fn() -> (), quantum: i32) -> Result<(), ProcessError> {
        let mut process = Process::new(quantum, function_to_run);
        try!(process.set_stack_pointer(self.number_of_processes));
        try!(process.mark_process_ready());
        self.processes[self.number_of_processes + 1] = Some(process);
        self.number_of_processes = self.number_of_processes + 1;
        Ok(())
    }

    pub fn schedule_next(&mut self) -> ()
    {
        if self.first_process_started {
            if self.processes[self.current_process].unwrap().tick() == ProcessTickResult::Yield {
                self.running_process().unwrap().save_context();
                self.pick_next_process();
                self.running_process().unwrap().restore_context();
            }
        }
        else {
            self.first_process_started = true;
            self.pick_next_process();
            self.running_process().unwrap().restore_context();
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

                match self.processes[process + 1] {
                    Some(p) => if p.is_process_ready() {
                        self.current_process = process + 1;
                        return process + 1;
                    },
                    _ => ()
                };
            }
        };
        next_process
    }

    fn running_process(&mut self) -> &mut Option<Process> {
        &mut self.processes[self.current_process]   
    }
}

// idle process implementation
fn idle() -> () {
    loop {}
}
