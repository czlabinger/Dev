global _start

section .data

address:
dw 2 ; takes ipv4 == 2 Bytes
dw 0 ; Port to connect to, since raw socet irrelevant
db 8 ; actuall address 8.8.8.8 == google dns
db 8
db 8
db 8
dd 0 ; packet has to be 16 byes
dd 0

packet:
db 8 ; Type 8 = Echo request
db 0

checksum:
dw 9
dw 0 ; identifier
dw 1 ; sequence number

buffer: ; recieves the resonse
times 1024 db 0ffh 

good:
db 'good'

bad:
db 'bad'

section .text

_start:
; Socket syscall
mov rax, 41
mov rdi, 2 ; family = ipv4
mov rsi, 3 ; raw Socket
mov rdx, 1 ; icmp
syscall

mov r12, rax ; save socket filedesctiptor
not word [checksum] ; make checksum

; sendto syscall
mov rax, 44
mov rdi, r12 ; socket
mov rsi, packet ; input buffer
mov rdx, 8 ; length
mov r10, 0 ; flags
mov r8, address ; address
mov r9, 16 ; lenth of address
syscall

; recvfrom syscall
mov rax, 45
mov rdi, r12
mov rsi, buffer ; buffer that recieves
mov rdx, 1024 ; length of buffer
mov r10, 0 ; flags
mov r8, 0 ; address
mov r9, 0
syscall

; Check icmp header 
; First 20B are ip headers
; 1B icmp Type
; 1B icmp 
cmp word [buffer+20], 0 ; reply is 0
jne failure

; write syscall
mov rax, 1
mov rdi, 1
mov rsi, good
mov rdx, 4
syscall

failure:
jmp $
