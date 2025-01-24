#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod memory;
mod process;
mod fs;
mod drivers;
mod interrupts;

use memory::MemoryManager;
use memory::allocator::MemoryAllocator;
use process::scheduler::Scheduler;
use process::process::Process;
use fs::FileSystemManager;
use drivers::keyboard;
use drivers::vga;
use interrupts::setup_idt;

const HEAP_START: usize = 0x100000;
const HEAP_SIZE: usize = 1024 * 1024; // 1 megabyte

#[no_mangle]
pub extern "C" fn _start() -> ! {
    setup_idt();
    enable_interrupts();

    let mut memory_manager = MemoryManager::new();
    let mut allocator = MemoryAllocator::new(HEAP_START, HEAP_SIZE);
    let mut scheduler = Scheduler::new();
    let mut fs_manager = FileSystemManager::new();

    keyboard::initialize_keyboard();
    vga::initialize_vga();
    vga::write_text("Hello, OS World!", 0, 0);

    let process1 = Process::new(1, 0x1000, 1);
    let process2 = Process::new(2, 0x2000, 2);

    scheduler.add_process(process1);
    scheduler.add_process(process2);

    fs_manager.mount("ext");

    loop {
        scheduler.schedule();
        if let Some(key) = keyboard::read_key() {
            vga::write_text(&format!("Key pressed: {}", key), 1, 0);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti");
    }
}
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod memory;
mod process;
mod fs;
mod drivers;
mod interrupts;

use memory::MemoryManager;
use memory::allocator::MemoryAllocator;
use process::scheduler::Scheduler;
use process::process::Process;
use fs::FileSystemManager;
use drivers::keyboard;
use drivers::vga;
use interrupts::setup_idt;

const HEAP_START: usize = 0x100000;
const HEAP_SIZE: usize = 1024 * 1024; // 1 megabyte

#[no_mangle]
pub extern "C" fn _start() -> ! {
    setup_idt();
    enable_interrupts();

    let mut memory_manager = MemoryManager::new();
    let mut allocator = MemoryAllocator::new(HEAP_START, HEAP_SIZE);
    let mut scheduler = Scheduler::new();
    let mut fs_manager = FileSystemManager::new();

    keyboard::initialize_keyboard();
    vga::initialize_vga();
    vga::write_text("Hello, OS World!", 0, 0);

    let process1 = Process::new(1, 0x1000, 1);
    let process2 = Process::new(2, 0x2000, 2);

    scheduler.add_process(process1);
    scheduler.add_process(process2);

    fs_manager.mount("ext");

    loop {
        scheduler.schedule();
        if let Some(key) = keyboard::read_key() {
            vga::write_text(&format!("Key pressed: {}", key), 1, 0);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti");
    }
}
