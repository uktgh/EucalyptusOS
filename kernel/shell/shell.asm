[BITS 32]
%include "constants.inc"
%include "macros.inc"

global start_shell
extern command_handler

section .data
prompt: db 'EucalyptusOS> ', 0
welcome_msg: db 'Welcome to EucalyptusOS!', 13, 10, 0

section .bss
command_buffer: resb MAX_COMMAND_LENGTH

section .text
start_shell:
    push ebp
    mov ebp, esp
    
    push welcome_msg
    call print_string
    add esp, 4
    
.shell_loop:
    push prompt
    call print_string
    add esp, 4
    
    push command_buffer
    call read_command
    add esp, 4
    
    push command_buffer
    call command_handler
    add esp, 4
    
    jmp .shell_loop
