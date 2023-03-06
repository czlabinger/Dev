; calculates 2^x

global _start

section .text

_start:
  mov ebx, 1
  mov ecx, 6

label:
  add ebx, ebx ; ebc += ebx
  dec ecx      ; ecx -= 1
  cmp ecx, 0   ; compare ecx to 0
  jg label     ; if greater jump to label
  
  mov eax, 1
  int 0x80
