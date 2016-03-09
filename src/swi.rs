// Software interrupt implementation

use vital::*;
use process::*;
use register::*;
use arm1176;

pub struct SoftwareInterruptHandler<'a> {
    vital: Option<*mut Vital<'a>>
}


impl<'a> SoftwareInterruptHandler<'a> {
    // Constructor
    pub const fn new (vital: Option<*mut Vital<'a>>) -> SoftwareInterruptHandler<'a> {
            SoftwareInterruptHandler {
                vital: vital
            }
    }
    
    // Sets the vital instance, needed to resolve the lifetime issues
    pub fn set_vital_instance (&mut self, vital: *mut Vital<'a>) -> () {
        self.vital = Some(vital);
    }

    pub fn handle(&mut self, interrupt: u32) -> u32 {
        match interrupt {
            1 => // running process id:
                self.get_process_id(),
            _ => 0
        }
    }


    #[no_mangle]
    pub fn get_process_id(&mut self) -> u32 {
        let vital: &mut Vital = unsafe { &mut *self.vital.unwrap() };
        vital.scheduler.running_process_id()
    }
}

#[no_mangle]
pub fn call_swi(number: u32) -> u32 {
    arm1176::swi(number)
}
