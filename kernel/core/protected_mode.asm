[BITS 16]
%include "constants.inc"

global switch_to_protected_mode
global gdt_start
global gdt_descriptor

section .data
gdt_start:
    ; null
    dd 0x0
    dd 0x0

    ; segment
    dw 0xFFFF    ; segment limit (16 bits)
    dw 0x0       ; base address (low 16 bits)
    db 0x0       ; base address (next 8 bits)
    db 10011010b ; access byte: executable, readable, accessed
    db 11001111b ; granularity: 4KB blocks, 32-bit, limit (high 4 bits)
    db 0x0       ; base address (high 8 bits)

    ; segment
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 10010010b
    db 11001111b
    db 0x0
    
gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

CODE_SEG equ 0x08
DATA_SEG equ 0x10

section .text
switch_to_protected_mode:
    cli
    lgdt [gdt_descriptor]
    mov eax, cr0
    or eax, 0x1
    mov cr0, eax
    jmp CODE_SEG:init_pm

[BITS 32]
init_pm:
    mov ax, DATA_SEG
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ebp, STACK_OFFSET
    mov esp, ebp
    call begin_pm
    ret