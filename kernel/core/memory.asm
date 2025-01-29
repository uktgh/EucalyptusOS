[BITS 32]
; Memory Management Unit
PAGES_PER_TABLE equ 1024
PAGE_SIZE equ 4096

init_memory:
    mov eax, page_directory
    mov cr3, eax
    
    ; enb paging
    mov eax, cr0
    or eax, 0x80000000
    mov cr0, eax
    ret

; allocate
allocate_block:
    push ebp
    mov ebp, esp
    pop ebp
    ret
