pub mod exceptions;
pub mod io;

pub trait InterruptStack {
    fn new() -> Self;
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct InterruptStackFrame {
    pub ip: u32,
    pub cs: u32,
    pub flags: u32,
    pub esp: u32,
    pub ss: u32,
}
#[repr(C, packed)]
#[derive(Debug)]
pub struct ExceptionStackWithError {
    pub error_code: u32,
    pub ip: u32,
    pub cs: u32,
    pub flags: u32,
    pub esp: u32,
    pub ss: u32,
}
impl InterruptStack for InterruptStackFrame {
    fn new() -> Self {
        Self {
            ip: 0x00,
            cs: 0x00,
            flags: 0x00,
            esp: 0x00,
            ss: 0x00,
        }
    }
}
impl InterruptStack for ExceptionStackWithError {
    fn new() -> Self {
        Self {
            error_code: 0x00,
            ip: 0x00,
            cs: 0x00,
            flags: 0x00,
            esp: 0x00,
            ss: 0x00,
        }
    }
}
