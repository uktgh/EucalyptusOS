[BITS 32]
extern clear_screen
extern print_string
extern init_protected_mode
extern init_interrupts
extern init_memory
extern init_fs
extern start_shell

global kernel_start

section .text
kernel_start:
    ; init core
    call init_protected_mode
    call init_interrupts
    call init_memory
    call init_fs
    
    call clear_screen
    
    ; logo
    push logo
    call print_string
    add esp, 4
    
    call start_shell
    
    cli
    hlt

section .data
logo:   db "  ____                  __          __           ____  _____ ", 13, 10
        db " / __/_ _____  ___ ____/ /_ _____ _/ /_ _ ___  / __ \/ ___/", 13, 10
        db "/ _// // / _ \/ _ `/ _  / // / _ `/  ' \ / _ \/ /_/ /\__ \ ", 13, 10
        db "/___/\_,_/_//_/\_,_/\_,_/\_,_/\_,_/_/_/_/ \___/\____/____/ ", 13, 10
        db 13, 10
        db "Version 0.1.0", 13, 10
        db "Copyright (c) 2025 EucalyptusOS Team", 13, 10
        db 13, 10, 0