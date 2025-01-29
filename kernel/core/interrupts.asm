[BITS 32]
%include "constants.inc"

global setup_idt
global keyboard_handler

section .data
idt_descriptor:
    dw 256*8-1
    dd idt

section .bss
idt: resb 256*8

section .text
setup_idt:
    mov eax, keyboard_handler
    mov [idt + (0x21 * 8)], ax
    mov word [idt + (0x21 * 8) + 2], 0x08
    mov word [idt + (0x21 * 8) + 4], 0x8E00
    mov word [idt + (0x21 * 8) + 6], 0x0
    
    lidt [idt_descriptor]
    ret

keyboard_handler:
    pushad
    in al, 0x60
    mov bl, al
    mov al, 0x20
    out 0x20, al
    mov al, bl
    popad
    iret