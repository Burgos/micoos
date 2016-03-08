// Software interrupt implementation

use vital::*;
use process::*;
use register::*;

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
}
