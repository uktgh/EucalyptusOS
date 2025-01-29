[BITS 32]
%include "constants.inc"

global command_handler

section .data
cmd_table:
    dd cmd_help, 'help', 0
    dd cmd_clear, 'clear', 0
    dd cmd_ls, 'ls', 0
    dd cmd_exit, 'exit', 0
    dd 0

help_msg: 
    db 'available commands:', 13, 10
    db '  help  - show this help', 13, 10
    db '  clear - clear screen', 13, 10
    db '  ls    - list files', 13, 10
    db '  exit  - shutdown system', 13, 10, 0

section .text
command_handler:
    push ebp
    mov ebp, esp
    push esi
    push edi
    
    mov esi, [ebp + 8] ; cmd string
    mov edi, cmd_table
    
.check_command:
    mov edx, [edi] ; cmd handler
    test edx, edx
    jz .unknown_command
    
    add edi, 4
    push esi
    push edi
    call strcmp
    add esp, 8
    test eax, eax
    jz .execute
    
.next:
    inc edi
    cmp byte [edi-1], 0
    jnz .next
    jmp .check_command

.execute:
    call edx
    jmp .done

.unknown_command:
    push unknown_cmd_msg
    call print_string
    add esp, 4

.done:
    pop edi
    pop esi
    mov esp, ebp
    pop ebp
    ret