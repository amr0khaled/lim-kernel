use core::arch::naked_asm;

use spin::MutexGuard;

use crate::{
    debug, fault,
    internal::{
        idt::{Idt, IdtEntry},
        interrupts::{ExceptionStackWithError, InterruptStackFrame},
    },
    println,
};

static EXCEPTION_DIVISION_BY_ZERO: u8 = 0;
static EXCEPTION_DEBUG: u8 = 1;
static EXCEPTION_NON_MASKABLE_INTERRUPT: u8 = 2;
static EXCEPTION_BREAKPOINT: u8 = 3;
static EXCEPTION_OVERFLOW: u8 = 4;
static EXCEPTION_BOUND_RANGE_EXCEEDED: u8 = 5;
static EXCEPTION_INVALID_OPCODE: u8 = 6;
static EXCEPTION_DEVICE_NOT_FOUND: u8 = 7;
static EXCEPTION_DOUBLE_FAULT: u8 = 8;
static EXCEPTION_INVALID_TSS: u8 = 10;
static EXCEPTION_SEGMNET_NOT_PRESENT: u8 = 11;
static EXCEPTION_STACK_SEGEMENT_FAULT: u8 = 12;
static EXCEPTION_GENERAL_PROTECTION_FAULT: u8 = 13;
static EXCEPTION_PAGE_FAULT: u8 = 14;
static EXCEPTION_X87_FLOATING_POINT: u8 = 16;
static EXCEPTION_ALIGNMENT_CHECK: u8 = 17;
static EXCEPTION_MACHINE_CHECK: u8 = 18;
static EXCEPTION_SIMD_FLOATING_POINT: u8 = 19;
static EXCEPTION_VIRTUALIZATION: u8 = 20;
static EXCEPTION_CONTROL_PROTECTION: u8 = 21;
static EXCEPTION_HYPERVISOR_INJECTION: u8 = 28;
static EXCEPTION_VMM_COMMUNICATION: u8 = 29;
static EXCEPTION_SECURITY: u8 = 30;

#[repr(C, packed)]
#[derive(Debug)]
struct PageFaultFrame {
    // Set when caused by page-protection violation
    present: bool,
    // Set when it caused by a write access, when not set it's read access
    write: bool,
    // Set when caused while CPL = 3 (user privilage),
    // it doesn't mean that it caused by privilage violation
    user: bool,
    // Set when one or more page directory entries contain reserved bits whick is set 1
    // Only applies to PSE or PAE flags are set to 1 in CR4
    // It also include invalid physical address (1 << 40 on a processor with only 39 bits)
    reserved_write: bool,
    // Set when caused by an fetch instruction.
    // only applies when non-execute bit is supported and enabled
    instruction_fetch: bool,
    // Set when was caused by a protection-key violation
    // PKRU register (user-mode access)
    // or PKRS MSR (for supervisor-mode accesses) specifies the protection key rights
    protection_key: bool,
    // Set when caused by a shadow stack
    shadow_stack: bool,
    // Set when the fault was due to Intel SGX violation,
    // The fault is unrelated to ordinary paging.
    sgx: bool,
}
impl PageFaultFrame {
    pub fn from(code: u32) -> Self {
        Self {
            present: (code & (1)) != 0,
            write: (code & (1 << 1)) != 0,
            user: (code & (1 << 2)) != 0,
            reserved_write: (code & (1 << 3)) != 0,
            instruction_fetch: (code & (1 << 4)) != 0,
            protection_key: (code & (1 << 5)) != 0,
            shadow_stack: (code & (1 << 6)) != 0,
            sgx: (code & (1 << 15)) != 0,
        }
    }
}

/// Faults: Can be corrected and the program continue
pub extern "x86-interrupt" fn exception_division_by_zero(frame: InterruptStackFrame) {
    fault!("Division by zero: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_bound_range_exceeded(frame: InterruptStackFrame) {
    fault!("Bound range exceeded: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_invalid_opcode(frame: InterruptStackFrame) {
    fault!("Invalid opcode: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_device_not_found(frame: InterruptStackFrame) {
    fault!("device not found: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_invalid_tss(frame: ExceptionStackWithError) {
    fault!("Invalid TSS: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_segment_not_present(frame: ExceptionStackWithError) {
    fault!("Segment not present: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_stack_segment_fault(frame: ExceptionStackWithError) {
    fault!("Stack segment not present: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_general_protection_fault(frame: ExceptionStackWithError) {
    fault!("General protection fault: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_page_fault(frame: &ExceptionStackWithError) {
    let error_code: PageFaultFrame = PageFaultFrame::from(frame.error_code);
    fault!("Page Fault: {:?}\nError Code: {:?}", frame, error_code);
}
pub extern "x86-interrupt" fn exception_x87_floating_point(frame: InterruptStackFrame) {
    fault!("x87 floating point: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_alignment_check(frame: ExceptionStackWithError) {
    fault!("Alignment check: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_simd_floating_point(frame: InterruptStackFrame) {
    fault!("SIMD floating point: {:?}", frame)
}

/// Traps: Reported after execution of trapping instrction (debugging)
/*
*
* The Debug exception occurs on the following conditions:
*
*   Instruction fetch breakpoint (Fault)
*   General detect condition (Fault)
*   Data read or write breakpoint (Trap)
*   I/O read or write breakpoint (Trap)
*   Single-step (Trap)
*   Task-switch (Trap)
*
*
*/
// TODO: Write a read functions for debug registers (DR0..DR3, DR6, DR7)
pub extern "x86-interrupt" fn exception_debug(frame: InterruptStackFrame) {
    debug!("{:?}", frame)
}
pub extern "x86-interrupt" fn exception_breakpoint(frame: InterruptStackFrame) {
    debug!("Breakpoint: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_overflow(frame: InterruptStackFrame) {
    debug!("Overflow: {:?}", frame)
}
/// Aborts: Severe unrecoverable error
pub extern "x86-interrupt" fn exception_double_fault(frame: ExceptionStackWithError) {
    debug!("Double fault: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_machine_check(frame: InterruptStackFrame) {
    debug!("Machine check: {:?}", frame)
}
// Not exactly an exception, it doesn't have a vector number
// but occurs when an execution is generated  when attempt to call double fault exception handler
// It results in the processor resetting.
// TODO: Look on how to avoid it
pub extern "x86-interrupt" fn exception_triple_fault(frame: InterruptStackFrame) {
    fault!("Triple fault: {:?}", frame)
}

// Other interrupts
pub extern "x86-interrupt" fn non_maskable_interrupt(frame: InterruptStackFrame) {
    println!("Non-maskable: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_virtualization(frame: InterruptStackFrame) {
    fault!("Virtualization: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_control_protection(frame: ExceptionStackWithError) {
    fault!("Control Protection: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_hypervisor_injection(frame: InterruptStackFrame) {
    fault!("Hypervisor injection: {:?}", frame)
}
pub extern "x86-interrupt" fn exception_vmm_communication(frame: ExceptionStackWithError) {
    fault!("VMM communication: {:?}", frame)
}

pub extern "x86-interrupt" fn exception_security(frame: ExceptionStackWithError) {
    fault!("Security: {:?}", frame)
}

#[unsafe(naked)]
extern "C" fn get_interrupt_stack_error(stack: &mut ExceptionStackWithError) {
    naked_asm!(
        "mov eax, [esp + 4]",  // get frame address
        "mov edx, [esp + 20]", // get flags
        "mov [eax + 12], edx", // store flags inplace in frame
        "mov edx, [esp + 16]", // get flags
        "mov [eax + 8], edx",  // store cs inplace in frame
        "mov edx, [esp + 12]", // get ip
        "mov [eax + 4], edx",  // store ip inplace in frame
        "mov edx, [esp + 8]",  // get error_code
        "mov [eax], edx",      // store error_code inplace in frame
        "ret",
    )
}
#[unsafe(naked)]
extern "C" fn get_interrupt_frame(frame: &mut InterruptStackFrame) {
    naked_asm!(
        "mov eax, [esp + 4]",  // get frame address
        "mov edx, [esp + 16]", // get flags
        "mov [eax + 8], edx",  // store flags inplace in frame
        "mov edx, [esp + 12]", // get flags
        "mov [eax + 4], edx",  // store cs inplace in frame
        "mov edx, [esp + 8]",  // get ip
        "mov [eax], edx",      // store ip inplace in frame
        "ret",
    )
}

pub extern "C" fn exception_handlers_init(idt: &mut MutexGuard<'_, Idt>) {
    idt.set(
        EXCEPTION_DIVISION_BY_ZERO,
        IdtEntry::new_kernel_handler(exception_division_by_zero as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_DEBUG,
        IdtEntry::new_kernel_handler(exception_debug as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_NON_MASKABLE_INTERRUPT,
        IdtEntry::new_kernel_handler(non_maskable_interrupt as *const () as u64, false),
    );
    idt.set(
        EXCEPTION_BREAKPOINT,
        IdtEntry::new_kernel_handler(exception_breakpoint as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_OVERFLOW,
        IdtEntry::new_kernel_handler(exception_overflow as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_BOUND_RANGE_EXCEEDED,
        IdtEntry::new_kernel_handler(exception_bound_range_exceeded as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_INVALID_OPCODE,
        IdtEntry::new_kernel_handler(exception_invalid_opcode as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_DEVICE_NOT_FOUND,
        IdtEntry::new_kernel_handler(exception_device_not_found as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_DOUBLE_FAULT,
        IdtEntry::new_kernel_handler(exception_double_fault as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_INVALID_TSS,
        IdtEntry::new_kernel_handler(exception_invalid_tss as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_SEGMNET_NOT_PRESENT,
        IdtEntry::new_kernel_handler(exception_segment_not_present as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_STACK_SEGEMENT_FAULT,
        IdtEntry::new_kernel_handler(exception_stack_segment_fault as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_GENERAL_PROTECTION_FAULT,
        IdtEntry::new_kernel_handler(exception_general_protection_fault as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_PAGE_FAULT,
        IdtEntry::new_kernel_handler(exception_page_fault as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_X87_FLOATING_POINT,
        IdtEntry::new_kernel_handler(exception_x87_floating_point as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_ALIGNMENT_CHECK,
        IdtEntry::new_kernel_handler(exception_alignment_check as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_MACHINE_CHECK,
        IdtEntry::new_kernel_handler(exception_machine_check as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_SIMD_FLOATING_POINT,
        IdtEntry::new_kernel_handler(exception_simd_floating_point as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_VIRTUALIZATION,
        IdtEntry::new_kernel_handler(exception_virtualization as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_CONTROL_PROTECTION,
        IdtEntry::new_kernel_handler(exception_control_protection as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_HYPERVISOR_INJECTION,
        IdtEntry::new_kernel_handler(exception_hypervisor_injection as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_VMM_COMMUNICATION,
        IdtEntry::new_kernel_handler(exception_vmm_communication as *const () as u64, true),
    );
    idt.set(
        EXCEPTION_SECURITY,
        IdtEntry::new_kernel_handler(exception_security as *const () as u64, true),
    );
}
