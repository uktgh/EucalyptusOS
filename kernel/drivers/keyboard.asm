[BITS 32]
%include "constants.inc"

global keyboard_init
global keyboard_read

section .text
keyboard_init:
    ; enb keyboard irq
    in al, 0x21
    and al, 0xFD
    out 0x21, al
    ret

keyboard_read:
    push ebp
    mov ebp, esp
    
.wait_key:
    in al, 0x64
    test al, 2
    jnz .wait_key
    
    in al, 0x60
    movzx eax, al
    
    mov esp, ebp
    pop ebp
    ret