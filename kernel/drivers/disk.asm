[BITS 32]
%include "constants.inc"

global read_sector
global write_sector

section .text
read_sector:
    push ebp
    mov ebp, esp
    
    mov dx, 0x1F6
    mov al, 0xE0
    out dx, al
    
    mov dx, 0x1F2
    mov al, 1
    out dx, al
    
    mov dx, 0x1F3
    mov al, [ebp + 8]
    out dx, al
    
    mov dx, 0x1F4
    mov al, [ebp + 12]
    out dx, al
    
    mov dx, 0x1F5
    xor al, al
    out dx, al
    
    mov dx, 0x1F7
    mov al, 0x20
    out dx, al
    
.wait:
    in al, dx
    test al, 8
    jz .wait
    
    mov ecx, 256
    mov dx, 0x1F0
    mov edi, [ebp + 16]
    rep insw
    
    mov esp, ebp
    pop ebp
    ret
