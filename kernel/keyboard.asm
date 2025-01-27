[BITS 16]

read_char:
    mov ah, 0x00
    int 0x16
    ret

check_key:
    mov ah, 0x01
    int 0x16
    ret