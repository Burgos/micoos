// System calls

use swi::*;

pub fn sys_get_process_id() -> u32 {
    call_swi(1, 0, 0)
}
