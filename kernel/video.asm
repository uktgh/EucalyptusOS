[BITS 16]

print_string:
    pusha
    mov ah, 0x0E
.loop:
    lodsb
    test al, al
    jz .done
    int 0x10 ; print
    jmp .loop
.done:
    popa
    ret

clear_screen:
    pusha
    mov ah, 0x00
    mov al, 0x03 ; 80x25 text
    int 0x10
    popa
    ret

set_cursor:
    pusha
    mov ah, 0x02
    mov bh, 0x00
    int 0x10
    popa
    ret