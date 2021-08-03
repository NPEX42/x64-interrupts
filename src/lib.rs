#![cfg_attr(feature = "no_std", no_std)]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
use x86_64::instructions::interrupts::*;
pub(crate) mod idt;
pub(crate) mod pic;
pub(crate) mod gdt;

/// Initialize the x86 Interrupt Subsystem. This
/// is an unsafe operation as having the x86 
/// registers CS & the TSS set to incorrect 
/// values may result in undefined behaviour
pub unsafe fn initialize() -> Result<(), &'static str> {
    pic::initialize();
    Ok(())
}

/// Enable the specified Interrupt
pub fn enable_irq(id : u8) {
    idt::clear_irq_mask(id);
}

/// Disable the specified Interrupt
pub fn disable_irq(id : u8) {
    idt::set_irq_mask(id);
}

/// Set the specified IRQ's handler
pub fn set_irq_handler(id : u8, handler : fn()) {
    without_interrupts( || {
        idt::set_basic_handler(id, handler);
    });
}