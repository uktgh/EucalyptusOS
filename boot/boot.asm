[BITS 16]
[ORG 0x7C00]

start:
    ; init segments
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00

    ; video mode
    mov ah, 0x00
    mov al, 0x03
    int 0x10

    ; load kernel
    mov ah, 0x02
    mov al, 0x20 ; sectors to read
    mov ch, 0x00 ; cylinder 0
    mov cl, 0x02 ; sector 2
    mov dh, 0x00 ; head 0
    mov dl, 0x80 ; drive
    mov bx, KERNEL_OFFSET
    int 0x13

    jmp KERNEL_OFFSET

    times 510-($-$$) db 0
    dw 0xAA55

KERNEL_OFFSET equ 0x1000