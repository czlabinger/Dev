;; Bootable Kernel

bits 32

global _start
extern kmain

section .text

; grub 
align 4
dd 0x1BADB002
dd 0x00
dd - (0x1BADB002 + 0x00)

_start:
    cli
    mov esp, stack_space
    call kmain
    hlt

section .bss
resb 8192
stack_space: