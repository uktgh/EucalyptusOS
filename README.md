# Eucalyptus OS

this is a basic operating system :)

## project structure

```
EucalyptusOS/
├── 📂 bootloader/
│   ├── 📄 boot.asm
│   ├── 📄 disk.asm
│   ├── 📄 gdt.asm
│   └── 📄 idt.asm
├── 📂 src/
│   ├── 📄 main.rs
│   ├── 📄 memory.rs
│   ├── 📄 multitasking.rs
│   └── 📄 interrupts.rs
├── 📄 makefile
└── 📄 link.ld
```

## prerequisites

- nasm
- rust and target `i389-unknown-none-elf`
- qemu

## build and run

1) clone the repository:
```
git clone https://github.com/uktgh/EucalyptusOS.git
cd EucalyptusOS
```

2) build the project:
```
make
```

3) run the os in qemu:
```
make run
```

## to-do

 - [x] init gdt
 - [x] switch to protected mode
 - [x] load kernel from disk
 - [x] set up idt
 - [x] entry point setup
 - [x] enable interrupts
 - [x] basic physical memory management with bitmap
 - [x] allocate and deallocate pages
 - [x] process creation
 - [x] basic scheduler implementation
 - [x] task switching
 - [x] set up idt in kernel
 - [x] basic interrupt handling routines
 - [ ] implement virtual memory
    - [ ] page tables setup
    - [ ] virtual to physical address translation
 - [ ] implement memory swapping to disk
 - [ ] optimize memory allocation
 - [ ] design basic file system (e.g. FAT, ext...)
 - [ ] implement file read/write operations
 - [ ] directory management
 - [ ] implement process states (ready, running, waiting, terminated)
 - [ ] priority-based scheduling
 - [ ] ipc (inner-process communication)      
