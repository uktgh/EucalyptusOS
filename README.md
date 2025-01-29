# 🌿 EucalyptusOS 

## project structure
```
📁 EucalyptusOS/
├── 📁 boot/
│   └── 📄 boot.asm
├── 📁 kernel/
│   ├── 📁 core/
│   │   ├── 📄 interrupts.asm
│   │   ├── 📄 memory.asm
│   │   └── 📄 protected_mode.asm
│   ├── 📁 drivers
│   │   ├── 📄 disk.asm
│   │   ├── 📄 keyboard.asm
│   │   └── 📄 video.asm
│   ├── 📁 fs
│   │   └── 📄 filesystem.asm
│   ├── 📁 inc
│   │   ├── 📄 constants.inc
│   │   ├── 📄 fs_constants.inc
│   │   └── 📄 macros.inc
│   ├── 📁 shell
│   │   ├── 📄 commands.asm
│   │   └── 📄 shell.asm
│   └── 📄 kernel.asm
├── 📁 linker/
│   └── 📄 kernel.ld
└── 📑 makefile
```

## prerequisites
- nasm >= 2.15
- gnu make >= 4.3
- qemu >= 6.2 (testing)
- gcc >= 9.4 (tools) 

## features
- [x] basic boot in 16 bit real mode
- [x] basic video output (character printing)
- [x] basic keyboard input
- [x] main kernel loop
- [x] character echo
- [x] shutdown command (esc)
- [x] switch to 32 bit protected mode
- [x] memory management
- [x] basic file system
- [x] interactive shell
- [x] multiple commands 
- [x] interrupt handling
- [x] disk driver
- [ ] system timer
- [ ] basic multitasking 

## setup & usage
```
git clone https://github.com/uktgh/EucalyptusOS.git
cd EucalyptusOS


clean  ─  make clean
build  ─  make

run    ─  make run
       └ qemu-system-i386 -drive format=raw,file=build/imgs/os.img
```
