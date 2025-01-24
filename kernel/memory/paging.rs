pub fn setup_paging() {
    let page_dir: [u32; 1024] = [0; 1024];
    let page_table: [u32; 1024] = [0; 1024];

    for i in 0..1024 {
        page_table[i] = (i * 4096) as u32 | 3;
    }

    page_dir[0] = (&page_table as *const u32 as u32) | 3;

    unsafe {
        asm!("mov cr3, {}", in(reg) &page_dir as *const u32 as u32);
        asm!("mov eax, cr0");
        asm!("or eax, 0x80000000");
        asm!("mov cr0, eax");
    }
}
