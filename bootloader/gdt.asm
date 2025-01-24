gdt_start:
    dq 0x0000000000000000 ; null
    dq 0x00cf9a000000ffff ; code
    dq 0x00cf92000000ffff ; data

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

setup_gdt:
    lgdt [gdt_descriptor]
    ret

CODE_SEG equ gdt_start + 0x08
DATA_SEG equ gdt_start + 0x10
KERNEL_SEG equ gdt_start + 0x18
