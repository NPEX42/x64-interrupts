use crate::pic::*;
use lazy_static::lazy_static;
use x86_64::structures::{idt::{InterruptDescriptorTable, InterruptStackFrame}};
use spin::Mutex;
use x86_64::instructions::port::*;

const PIC1: u16 = 0x21;
const PIC2: u16 = 0xA1;

type BasicHandler = fn();
type BreakpointHandler = fn(u64, u64, u64);

lazy_static! {
    static ref DF_HANDLER : Mutex<BasicHandler> = Mutex::new(default_handler);
    static ref BP_HANDLER : Mutex<BreakpointHandler> = Mutex::new(default_bp_handler);
    static ref HANDLERS : Mutex<[BasicHandler; 16]> = Mutex::new([default_handler; 16]);

    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        set_handler!(idt, 0, irq_0);
        set_handler!(idt, 1, irq_1);
        set_handler!(idt, 2, irq_2);
        set_handler!(idt, 3, irq_3);
        set_handler!(idt, 4, irq_4);
        set_handler!(idt, 5, irq_5);
        set_handler!(idt, 6, irq_6);
        set_handler!(idt, 7, irq_7);
        set_handler!(idt, 8, irq_8);
        set_handler!(idt, 9, irq_9);
        set_handler!(idt, 10, irq_10);
        set_handler!(idt, 11, irq_11);
        set_handler!(idt, 12, irq_12);
        set_handler!(idt, 13, irq_13);
        set_handler!(idt, 14, irq_14);
        set_handler!(idt, 15, irq_15);
        idt
    };
}


fn default_handler() {

}

fn default_bp_handler(_ip: u64, _flags: u64, _cs: u64) {

} 

gen_interrupt_handler!(irq_0, 0);
gen_interrupt_handler!(irq_1, 1);
gen_interrupt_handler!(irq_2, 2);
gen_interrupt_handler!(irq_3, 3);
gen_interrupt_handler!(irq_4, 4);
gen_interrupt_handler!(irq_5, 5);
gen_interrupt_handler!(irq_6, 6);
gen_interrupt_handler!(irq_7, 7);
gen_interrupt_handler!(irq_8, 8);
gen_interrupt_handler!(irq_9, 9);
gen_interrupt_handler!(irq_10, 10);
gen_interrupt_handler!(irq_11, 11);
gen_interrupt_handler!(irq_12, 12);
gen_interrupt_handler!(irq_13, 13);
gen_interrupt_handler!(irq_14, 14);
gen_interrupt_handler!(irq_15, 15);

macro gen_interrupt_handler($name : ident, $irq : expr) {
    extern "x86-interrupt" fn $name(_ : InterruptStackFrame) {
        let handlers = HANDLERS.lock();
        handlers[$irq]();
        unsafe { notify_end_of_interrupt(index($irq)) }
    }
}

macro set_handler($idt : ident, $irq : expr, $handler : ident) {
    $idt[index($irq) as usize].set_handler_fn($handler);
}

pub(crate) fn set_basic_handler(irq : u8, handler : BasicHandler) {
    HANDLERS.lock()[index(irq) as usize] = handler;
}

pub(crate) fn index(irq : u8) -> u8 {
    pic_1_offset() + irq
}

pub(crate) fn set_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { PIC1 } else { PIC2 });
    unsafe {
        let value = port.read() | (1 << (if irq < 8 { irq } else { irq - 8 }));
        port.write(value);
    }
}

pub(crate) fn clear_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { PIC1 } else { PIC2 });
    unsafe {
        let value = port.read() & !(1 << if irq < 8 { irq } else { irq - 8 });
        port.write(value);
    }
}
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    BP_HANDLER.lock()(stack_frame.instruction_pointer.as_u64(), stack_frame.cpu_flags, stack_frame.code_segment);
}

#[allow(dead_code)]
extern "x86-interrupt" fn double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    DF_HANDLER.lock()();
    panic!("Double Fault Exception");
} 