[BITS 32]

global clear_screen
global print_string
global print_char
global print_newline

section .data
VIDEO_MEMORY equ 0xB8000
COLS equ 80
ROWS equ 25
WHITE_ON_BLACK equ 0x0F

cursor_pos dd VIDEO_MEMORY

section .text
clear_screen:
    push ebp
    mov ebp, esp
    
    mov ecx, ROWS * COLS
    mov edi, VIDEO_MEMORY
    mov ax, WHITE_ON_BLACK << 8 | ' '
    rep stosw
    
    mov dword [cursor_pos], VIDEO_MEMORY
    
    mov esp, ebp
    pop ebp
    ret

print_string:
    push ebp
    mov ebp, esp
    push esi
    
    mov esi, [ebp + 8]
.loop:
    lodsb
    test al, al
    jz .done
    push eax
    call print_char
    add esp, 4
    jmp .loop
    
.done:
    pop esi
    mov esp, ebp
    pop ebp
    ret

print_char:
    push ebp
    mov ebp, esp
    
    mov eax, [ebp + 8]
    mov edi, [cursor_pos]
    mov ah, WHITE_ON_BLACK
    mov word [edi], ax
    add edi, 2
    mov dword [cursor_pos], edi
    
    mov eax, VIDEO_MEMORY
    add eax, COLS * ROWS * 2
    cmp edi, eax
    jl .done
    
    call scroll_screen
    
.done:
    mov esp, ebp
    pop ebp
    ret

scroll_screen:
    push ebp
    mov ebp, esp
    push esi
    push edi
    
    mov esi, VIDEO_MEMORY + COLS * 2
    mov edi, VIDEO_MEMORY
    mov ecx, (ROWS - 1) * COLS
    rep movsw
    
    mov ecx, COLS
    mov ax, WHITE_ON_BLACK << 8 | ' '
    rep stosw
    
    mov dword [cursor_pos], VIDEO_MEMORY + (ROWS - 1) * COLS * 2
    
    pop edi
    pop esi
    mov esp, ebp
    pop ebp
    ret

print_newline:
    push ebp
    mov ebp, esp
    
    ; next line pos
    mov eax, [cursor_pos]
    sub eax, VIDEO_MEMORY
    xor edx, edx
    mov ecx, COLS * 2
    div ecx
    inc eax
    mul ecx
    add eax, VIDEO_MEMORY
    mov dword [cursor_pos], eax
    
    mov esp, ebp
    pop ebp
    ret
