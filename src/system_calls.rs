// System calls

use swi::*;

pub fn sys_get_process_id() -> u32 {
    call_swi(1, 0, 0)
}

pub fn sys_send_message_to_process(process_id: u32, msg: u32) -> u32 {
    call_swi(2, process_id, msg)
}
