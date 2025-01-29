# 🌿 EucalyptusOS 

## project structure
```
EucalyptusOS/
├── boot/
│   └── boot.asm
├── kernel/
│   ├── kernel.asm
│   ├── video.asm
│   ├── keyboard.asm
│   └── utils.asm
└── makefile
```

## features
- [x] basic boot in 16 bit real mode
- [x] basic video output (character printing)
- [x] basic keyboard input
- [x] main kernel loop
- [x] character echo
- [x] shutdown command (esc)
- [ ] switch to 32 bit protected mode
- [ ] memory management
- [ ] basic file system
- [ ] interactive shell
- [ ] multiple commands 
- [ ] interrupt handling
- [ ] system timer
- [ ] disk driver
- [ ] basic multitasking 

## setup & usage
```
git clone https://github.com/uktgh/EucalyptusOS.git
cd EucalyptusOS

build  ->  make
run    ->  make run
```
- system boots showing a message
- any key pressed is displayed on screen
- press esc to shutdown the system

## development notes
- system operates in 16 bit mode
- uses bios interrupts for i/o
- video memory mapped at 0xB8000
- stack initialized at 0x7C00

## current status 
this is a basic educational project in active development. it currently provides a minimal bootable environment with basic i/o capabilities.