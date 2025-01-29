%define MAX_FILES 64
%define MAX_FILENAME_LENGTH 16
%define SECTOR_SIZE 512
%define ROOT_DIR_SECTOR 1
%define FILE_TABLE_SIZE (MAX_FILES * 32)  ; 32 bytes per file entry