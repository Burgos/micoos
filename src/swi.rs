// Software interrupt implementation

use vital::*;
use process::*;
use register::*;
use arm1176;

#[no_mangle]
pub fn call_swi(number: u32, value_1: u32, value_2: u32) -> u32 {
    arm1176::swi(number, value_1, value_2)
}

pub fn handle(vital: &mut Vital, interrupt: u32, value_1: u32, value_2: u32) -> u32 {
    match interrupt {
        1 => // running process id:
            get_process_id(),
        _ => 0
    }
}

#[no_mangle]
pub fn get_process_id() -> u32 {
    let mut vital = VITAL.lock();
    let i = vital.running_process_id();

    if cfg!(feature="log-swi") {
        kprint!("Got process id for process: {}", i);
    }

    i
}
