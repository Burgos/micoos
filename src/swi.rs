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
        let serial = Register::new(0x101f1000 as *mut u32);
        serial.set('*' as u32);
        0
    }
}

#[no_mangle]
pub fn call_swi(number: u32) -> u32 {
    arm1176::swi(number)
}
