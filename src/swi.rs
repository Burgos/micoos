// Software interrupt implementation

use vital::*;
use process::*;
use register::*;
use arm1176;

#[no_mangle]
pub fn call_swi(number: u32, value: u32) -> u32 {
    arm1176::swi(number, value)
}

pub fn handle(vital: &mut Vital, interrupt: u32, value: u32) -> u32 {
    match interrupt {
        1 => // running process id:
            get_process_id(vital),
        _ => 0
    }
}

#[no_mangle]
pub fn get_process_id(vital: &mut Vital) -> u32 {
    let i = vital.running_process_id();
        i
}
