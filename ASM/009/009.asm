; While loop

section .text

global _start

_start:

  mov esi, 10
  call L1

  mov eax, 1
  mov ebx, 0
  int 0x80

L1:
  mov eax, 4
  mov ebx, 1
  mov ecx, msg
  mov edx, len
  int 0x80
  
  dec esi
  jne L1
  ret

section .data
  msg db "test", 0x0a
  len equ $ - msg
