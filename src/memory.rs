const MEMORY_SIZE: usize = 1024 * 1024 * 32; // 32 megabyte
const PAGE_SIZE: usize = 4096;

struct Bitmap {
    map: [u8; MEMORY_SIZE / PAGE_SIZE / 8],
}

impl Bitmap {
    fn new() -> Self {
        Self {
            map: [0; MEMORY_SIZE / PAGE_SIZE / 8],
        }
    }

    fn set(&mut self, page: usize) {
        let byte = page / 8;
        let bit = page % 8;
        self.map[byte] |= 1 << bit;
    }

    fn clear(&mut self, page: usize) {
        let byte = page / 8;
        let bit = page % 8;
        self.map[byte] &= !(1 << bit);
    }

    fn is_set(&self, page: usize) -> bool {
        let byte = page / 8;
        let bit = page % 8;
        self.map[byte] & (1 << bit) != 0
    }

    fn find_free(&self) -> Option<usize> {
        for (byte, &bits) in self.map.iter().enumerate() {
            if bits != 0xFF {
                for bit in 0..8 {
                    if bits & (1 << bit) == 0 {
                        return Some(byte * 8 + bit);
                    }
                }
            }
        }
        None
    }
}

struct MemoryManager {
    bitmap: Bitmap,
}

impl MemoryManager {
    fn new() -> Self {
        Self {
            bitmap: Bitmap::new(),
        }
    }

    fn allocate_page(&mut self) -> Option<usize> {
        if let Some(page) = self.bitmap.find_free() {
            self.bitmap.set(page);
            Some(page * PAGE_SIZE)
        } else {
            None
        }
    }

    fn deallocate_page(&mut self, address: usize) {
        let page = address / PAGE_SIZE;
        self.bitmap.clear(page);
    }
}
