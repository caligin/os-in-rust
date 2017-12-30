global start

section .text
bits 32
start:
    ; print `QQ` to screen
    mov dword [0xb8000], 0x2f512f51
    hlt
