clear_bss:
    mov edi, bss_start
    mov ecx, bss_end - bss_start
    xor eax, eax
    rep stosb
    ret

bss_start equ 0x100000
bss_end equ bss_start + 0x20000

setup_paging:
    mov eax, cr3
    or eax, 0x80000000
    mov cr3, eax
    ret
