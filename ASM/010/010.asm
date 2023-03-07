; for Loop that prints alphabet

global _start

section .text

_start:

  mov esi, 0
  call forLoop
  
  mov eax, 1 
  mov ebx, 0
  int 0x80

forLoop:
  
  mov eax, 4
  mov ebx, 1
  mov ecx, msg
  mov edx, len
  int 0x80

  inc esi
  inc byte [msg]
  cmp esi, 25
  jle forLoop
  
  ret

section .data
  msg db 0x41, 0x0a
  len equ $ - msg
