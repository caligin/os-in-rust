global long_mode_start

section .text
bits 64
long_mode_start:
	mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
	extern rust_main
    call rust_main
    mov rax, 0x2f652f6e2f6f2f64
    mov qword [0xb8000], rax
    hlt
