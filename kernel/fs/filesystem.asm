[BITS 32]

%include "fs_constants.inc"

global init_fs
global create_file
global read_file
global write_file
extern read_sector

struc file_entry
    .name: resb MAX_FILENAME_LENGTH
    .size: resd 1
    .start_sector: resd 1
    .flags: resb 1
    alignb 32
endstruc

section .data
fs_magic: db 'EFS', 0 ; EFS = eucalyptus file system 
root_dir_sector: dd ROOT_DIR_SECTOR
free_space_map: times 1024 db 0

section .bss
file_table: resb FILE_TABLE_SIZE

section .text
init_fs:
    push ebp
    mov ebp, esp
    
    ; root directory
    push dword file_table       ; buffer address
    push dword 0                ; head
    push dword ROOT_DIR_SECTOR  ; sector number
    push dword 0                ; cylinder
    push dword 1                ; sectors
    call read_sector
    add esp, 20
    
    ; init free space map
    mov ecx, 1024
    mov edi, free_space_map
    mov al, 0xFF
    rep stosb
    
    mov esp, ebp
    pop ebp
    ret

create_file:
    push ebp
    mov ebp, esp

    mov esp, ebp
    pop ebp
    ret

read_file:
    push ebp
    mov ebp, esp
    
    mov esp, ebp
    pop ebp
    ret

write_file:
    push ebp
    mov ebp, esp
    
    mov esp, ebp
    pop ebp
    ret
