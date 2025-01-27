[BITS 16]

delay:
    pusha
    mov cx, 0xFFFF

.loop:
    loop .loop
    popa
    ret

hex_to_string:
    pusha
    mov cx, 4
.loop:
    rol dx, 4
    mov ax, dx          
    and ax, 0x0F ; 4b
    add al, '0'
    cmp al, '9'
    jle .store
    add al, 7
.store:
    mov [bx], al ; save char
    inc bx
    dec cx
    jnz .loop
    popa
    ret