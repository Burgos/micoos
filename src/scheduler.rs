// Scheduler implementation. Contains list of
// processes and select the next one on the
// context switch.

use process::Process;
use process::ProcessError;
use process::ProcessTickResult;
use process::ProcessState;
use vital::Vital;

pub struct Scheduler<'a> {
    processes: [Option<Process<'a>>; 10],
    current_process: usize,
    number_of_processes: usize,
    first_process_started: bool,
    vital: Option<*const Vital<'a>>
}


impl<'a> Scheduler<'a> {
    pub fn new() -> Scheduler<'a> {
        Scheduler {
            processes: [None; 10],
            current_process: 0,
            number_of_processes: 0,
            first_process_started: false,
            vital: None
        }
    }

    // gets the process reference for the given process id
    pub fn get_process_by_id (&mut self, id: usize) -> Option<&mut Process<'a>> {
        match self.processes[id] {
            Some(_) =>
                self.processes[id].as_mut(),
            None => None
        }
    }

    // Sets the vital instance, needed to resolve the lifetime issues
    pub fn set_vital_instance (&mut self, vital: *const Vital<'a>) -> () {
        self.vital = Some(vital);
    }

    // Registers process to scheduler
    pub fn add_process(&mut self, function_to_run: fn() -> (), quantum: i32) -> Result<(), ProcessError> {
        let mut process = Process::new(quantum, function_to_run, self.vital.unwrap());
        try!(process.set_stack_pointer(self.number_of_processes));
        try!(process.mark_process_ready());
        self.processes[self.number_of_processes + 1] = Some(process);
        self.number_of_processes = self.number_of_processes + 1;
        Ok(())
    }

    // Schedules next process
    pub fn schedule_next(&mut self) -> ()
    {
        if self.first_process_started {
            if self.processes[self.current_process].as_mut().unwrap().tick() == ProcessTickResult::Yield {
                self.running_process().as_mut().unwrap().save_context();
                self.pick_next_process();
                self.running_process().as_mut().unwrap().restore_context();
            }
        }
        else {
            self.first_process_started = true;
            self.pick_next_process();
            self.running_process().as_mut().unwrap().restore_context();
        }
    }

    // Implementes strategy how the next process is scheduled
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

                match self.processes[process + 1].as_ref() {
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

    // Gets the running process reference
    fn running_process(&mut self) -> Option<&mut Process<'a>> {
        self.processes[self.current_process].as_mut()   
    }
}

// idle process implementation
fn idle() -> () {
    loop {}
}
