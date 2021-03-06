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
    first_process_started: bool,
}


impl Scheduler {
    pub const fn new() -> Scheduler {
        Scheduler {
            processes: [None; 10],
            current_process: 0,
            number_of_processes: 0,
            first_process_started: false,
        }
    }

    // gets the process reference for the given process id
    pub fn get_process_by_id (&mut self, id: usize) -> Option<&mut Process> {
        match self.processes[id] {
            Some(_) =>
                self.processes[id].as_mut(),
            None => None
        }
    }

    // Registers process to scheduler
    pub fn add_process(&mut self, function_to_run: fn() -> (), quantum: i32) -> Result<(), ProcessError> {
        let process_id = self.number_of_processes + 1;
        let mut process = Process::new(quantum, function_to_run, process_id);
        try!(process.set_stack_pointer(self.number_of_processes));
        try!(process.mark_process_ready());
        self.processes[process_id] = Some(process);
        self.number_of_processes = self.number_of_processes + 1;

        if cfg!(feature="log-scheduler") {
            kprint!("Added process. Number of processes: {}\n", self.number_of_processes);
        }

        Ok(())
    }

    pub fn tick(&mut self) -> () {
        self.schedule_next(false);
    }

    pub fn yield_process(&mut self) -> () {
        self.schedule_next(true);
    }

    // Schedules next process
    fn schedule_next(&mut self, force_yield: bool) -> ()
    {
        if cfg!(feature="log-scheduler") {
            kprint!("calling schedule_next. Process started: {}\n", self.first_process_started);
        }

        if self.first_process_started {
            if force_yield || self.processes[self.current_process].as_mut().unwrap().tick() == ProcessTickResult::Yield {
                self.running_process().as_mut().unwrap().save_context();
                self.pick_next_process();
                self.running_process().as_mut().unwrap().restore_context();
            }
        }
        else {
            self.first_process_started = true;
            self.pick_next_process();

            if cfg!(feature="log-scheduler") {
                kprint!("Picked next process: {}/{}\n", self.current_process, self.number_of_processes);
            }

            self.running_process().as_mut().unwrap().restore_context();
        }
    }

    // Implementes strategy how the next process is scheduled
    fn pick_next_process(&mut self) -> usize {
        if cfg!(feature="log-scheduler") {
            kprint!("Inside pick_next_process\n");
        }

        let previous_process = self.current_process;

        if cfg!(feature="log-scheduler") {
            kprint!("Previous process: {}\n", previous_process);
        }
        
        let next_process = {
            loop {
                if cfg!(feature="log-scheduler") {
                    kprint!("Before doing modulo to pick next process. number_of_processes: {}\n",
                            self.number_of_processes);
                }

                let mut process = (self.current_process + 1) % self.number_of_processes;

                if cfg!(feature="log-scheduler") {
                    kprint!("Picked next process: {}\n", process);
                }

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
    pub fn running_process(&mut self) -> Option<&mut Process> {
        self.processes[self.current_process].as_mut()   
    }

    #[no_mangle]
    pub fn running_process_id(&mut self) -> u32 {
        self.current_process as u32
    }
}


// idle process implementation
fn idle() -> () {
    loop {}
}

