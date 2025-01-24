[bits 16]
[org 0x7c00]

%include "gdt.asm"
%include "idt.asm"
%include "disk.asm"
%include "utils.asm"

start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00

    call setup_gdt
    call setup_idt

    mov eax, cr0
    or eax, 1
    mov cr0, eax

    jmp CODE_SEG:init_pm

[bits 32]
init_pm:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    call load_kernel

    call clear_bss
    call setup_paging

    jmp KERNEL_SEG:0x1000

times 510 - ($ - $$) db 0
dw 0xaa55
