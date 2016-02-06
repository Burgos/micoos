// implementation of the process

enum State {
    CREATED,
    RUNNING,
    PAUSED,
    STOPPED
}

pub struct Process {
    quantum: i32,
    remaining: i32,
    stack_pointer: i32,
    state: State
}

impl Process {
    pub fn new() -> Process {
        Process {
            quantum: 50,
            remaining: 50,
            stack_pointer: 0,
            state: State::CREATED
        }
    }

    pub fn run(&self) -> () {
        self.init();
        self.run_task();
    }

    fn init(&self) -> () {
        // copy a stack etc here
    }

    pub fn run_task(&self) -> () {

    }

    pub fn save_context(&mut self) -> () {

    }

    pub fn restore_context(&mut self) -> () {

    }
}
