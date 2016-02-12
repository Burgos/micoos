// Scheduler implementation. Contains list of
// processes and select the next one on the
// context switch.

use process::Process;

pub struct Scheduler {
    processes: [Process; 10],
    current_process: usize,
    number_of_processes: usize
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            processes: [Process::new(); 10],
            current_process: 0,
            number_of_processes: 0
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes[self.number_of_processes] = process;
        self.number_of_processes = self.number_of_processes + 1;
    }


}
