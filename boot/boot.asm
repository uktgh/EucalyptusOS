[BITS 16]
[ORG 0x7C00]

start:
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00

    mov ah, 0x00
    mov al, 0x03
    int 0x10

    ; load than jump to kernel
    mov ah, 0x02
    mov al, 0x02
    mov ch, 0x00
    mov cl, 0x02
    mov dh, 0x00
    mov dl, 0x80
    mov bx, kernel
    int 0x13

    jmp kernel

    times 510-($-$$) db 0
    dw 0xAA55

kernel:
    %include "kernel/kernel.asm"