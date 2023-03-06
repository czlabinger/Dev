; memory access

global _start

section .data
  addr db "yellow"

  ; db stores 1 byte
    ; name1 db "Hello, World!"
    ; name2 db 0xff (Hex)
    ; name3 db 100 (Decimal)

  ; dw stores 2 bytes
    ; name4 dw 1000

  ; dd stores 4 bytes
    ; name5 dd 100000

section .text

_start:
  mov [addr], byte 'H'      ; set beginning of addr to 'H'
  mov [addr+5], byte '!'    ; sets beginning +5 to !

  mov eax, 4
  mov ebx, 1
  mov ecx, addr
  mov edx, 6
  int 0x80

  mov eax, 1
  mov ebx, 0
  int 0x80
