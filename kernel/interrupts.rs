use core::arch::asm;

static IDT: [u64; 256] = [0; 256];

pub fn setup_idt() {
    let idt_ptr = IDT_PTR {
        limit: (core::mem::size_of_val(&IDT) - 1) as u16,
        base: &IDT as *const _ as u64,
    };
    unsafe {
        asm!("lidt [{}]", in(reg) &idt_ptr);
    }
}

#[repr(C, packed)]
struct IDT_PTR {
    limit: u16,
    base: u64,
}

pub fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}
