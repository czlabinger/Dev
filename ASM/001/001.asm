# 32-bit Write to stdout

.global _start
.intel_syntax
.section .text

_start:
  # print
  mov %eax, 4
  mov %ebx, 1
  lea %ecx, [message]
  mov %edx, 14
  int 0x80

  # exit Programm
  mov %eax, 1
  mov %ebx, 65
  int 0x80

.section .data
  message:
  .ascii "Hello, World!\n"
