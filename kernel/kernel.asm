[BITS 16]

kernel_start:
    mov si, msg
    call print_string

main_loop:
    mov ah, 0x00
    int 0x16
    
    cmp al, 0x1B
    je shutdown
    
    mov ah, 0x0E
    int 0x10
    
    jmp main_loop

shutdown:
    mov ax, 0x5307
    mov bx, 0x0001
    mov cx, 0x0003
    int 0x15
    
    cli
    hlt

%include "kernel/video.asm"
%include "kernel/keyboard.asm"
%include "kernel/utils.asm"

msg db 'EucalyptusOS v0.0.7', 13, 10, 0