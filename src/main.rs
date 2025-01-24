#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod memory;
mod multitasking;
mod interrupts;

use memory::MemoryManager;
use multitasking::{Scheduler, Process};
use interrupts::setup_idt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    setup_idt();
    enable_interrupts();

    let mut memory_manager = MemoryManager::new();
    let mut scheduler = Scheduler::new();

    let process1 = Process::new(1, 0x1000);
    let process2 = Process::new(2, 0x2000);

    scheduler.add_process(process1);
    scheduler.add_process(process2);

    loop {
        scheduler.switch();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}
