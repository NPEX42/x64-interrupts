use pic8259::ChainedPics;
use spin::Mutex;

const PIC_0: u8 = 0x20;
const PIC_1: u8 = 0x28;

static PICS: Mutex<ChainedPics> = Mutex::new(
    unsafe { ChainedPics::new(PIC_0, PIC_1) }
);

pub(crate) unsafe fn initialize() {
    PICS.lock().initialize();
}

pub(crate) unsafe fn notify_end_of_interrupt(irq : u8) {
    PICS.lock().notify_end_of_interrupt(irq);
}

pub(crate) fn pic_1_offset() -> u8 {
    PIC_0
}

pub(crate) fn _pic_2_offset() -> u8 {
    PIC_1
}