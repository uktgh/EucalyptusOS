pub struct MemoryAllocator {
    heap_start: usize,
    heap_size: usize,
    free_list: FreeList,
}

struct FreeList {
    head: Option<&'static mut FreeNode>,
}

struct FreeNode {
    size: usize,
    next: Option<&'static mut FreeNode>,
}

impl MemoryAllocator {
    pub fn new(heap_start: usize, heap_size: usize) -> Self {
        let mut allocator = MemoryAllocator {
            heap_start,
            heap_size,
            free_list: FreeList { head: None },
        };
        allocator.init();
        allocator
    }

    fn init(&mut self) {
        self.free_list.head = Some(unsafe {
            &mut *(self.heap_start as *mut FreeNode)
        });
        if let Some(head) = self.free_list.head.as_mut() {
            head.size = self.heap_size;
            head.next = None;
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        let size = align_up(size, core::mem::align_of::<FreeNode>());
        let mut current = &mut self.free_list.head;

        while let Some(ref mut node) = current {
            if node.size >= size {
                let next = node.next.take();
                let allocated = node as *mut FreeNode as *mut u8;
                if node.size > size + core::mem::size_of::<FreeNode>() {
                    let new_node = unsafe {
                        &mut *(allocated.add(size) as *mut FreeNode)
                    };
                    new_node.size = node.size - size - core::mem::size_of::<FreeNode>();
                    new_node.next = next;
                    node.size = size;
                    node.next = Some(new_node);
                } else {
                    node.size = size;
                }
                *current = node.next.take();
                return Some(allocated);
            }
            current = &mut node.next;
        }
        None
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        let size = align_up(size, core::mem::align_of::<FreeNode>());
        let mut node = FreeNode {
            size,
            next: None,
        };
        let mut current = &mut self.free_list.head;

        while let Some(ref mut cur_node) = current {
            if (ptr as usize) < (cur_node as *mut FreeNode as usize) {
                node.next = Some(cur_node);
                *current = Some(unsafe { &mut *(ptr as *mut FreeNode) });
                if let Some(new_node) = current.as_mut() {
                    *new_node = node;
                }
                return;
            }
            current = &mut cur_node.next;
        }
        *current = Some(unsafe { &mut *(ptr as *mut FreeNode) });
        if let Some(new_node) = current.as_mut() {
            *new_node = node;
        }
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
