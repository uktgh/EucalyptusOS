pub mod paging;
pub mod allocator;

pub struct MemoryManager {
    bitmap: [u8; 32 * 1024 / 4096 / 8],
    paging_enabled: bool,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            bitmap: [0; 32 * 1024 / 4096 / 8],
            paging_enabled: false,
        }
    }

    pub fn allocate_page(&mut self) -> Option<usize> {
        for (byte, &bits) in self.bitmap.iter().enumerate() {
            if bits != 0xFF {
                for bit in 0..8 {
                    if bits & (1 << bit) == 0 {
                        self.bitmap[byte] |= 1 << bit;
                        return Some((byte * 8 + bit) * 4096);
                    }
                }
            }
        }
        None
    }

    pub fn deallocate_page(&mut self, address: usize) {
        let page = address / 4096;
        let byte = page / 8;
        let bit = page % 8;
        self.bitmap[byte] &= !(1 << bit);
    }
}
