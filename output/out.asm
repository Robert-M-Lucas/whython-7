    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__4:
	push rbp
	mov rbp, rsp
	sub rsp, 24
	mov rcx, rbp
	dec rcx
	dec rcx
	mov qword [rbp-16], 0h
	mov qword [rbp-8], 0000000000000D0Ah
	mov rbx, 10
	._4.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	test rax, rax
	jnz ._4.loop
	sub rsp, 48
	mov ecx, -11
	call GetStdHandle
	mov rcx, rax
	mov rdx, rbp 
	sub rdx, 16
	mov qword [rsp + 40], 10h
	mov r8, [rsp + 40]
	mov r9, dword 00h
	mov qword [rsp + 32], 00h
	call WriteFile
	add rsp, 48
	leave
	ret

main:
	push rbp
	mov rbp, rsp
	sub rsp, 88
	mov qword [rbp-8], 0
	main.0:
	mov qword [rbp-16], 25
	mov rax, [rbp-8]
	mov rcx, [rbp-16]
	cmp rcx, rax
	setle [rbp-24]
	mov rax, [rbp-24]
	cmp rax, 0
	jnz main.1
	mov qword [rbp-32], 0
	mov qword [rbp-40], 1
	mov rax, qword [rbp-8]
	push rax
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp-32]
	push rax
	call _2
	add rsp, 24
	mov qword [rbp-48], rax
	mov rax, qword [rbp-48]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-56], 1
	mov rax, [rbp-8]
	add rax, [rbp-56]
	mov [rbp-8], rax
	jmp main.0
	main.1:
	mov qword [rbp-72], 0
	mov qword [rbp-80], 1
	mov qword [rbp-88], 21
	mov rax, qword [rbp-88]
	push rax
	mov rax, qword [rbp-80]
	push rax
	mov rax, qword [rbp-72]
	push rax
	call _2
	add rsp, 24
	mov qword [rbp-64], rax
	mov rcx, [rbp-64]
	call ExitProcess

_2:
	push rbp
	mov rbp, rsp
	sub rsp, 56
	mov qword [rbp-8], 0
	mov rax, [rbp+32]
	mov rcx, [rbp-8]
	cmp rax, rcx
	setnle [rbp-16]
	mov rax, [rbp-16]
	cmp rax, 0
	jnz .2.0
	mov rax, [rbp+16]
	leave
	ret
	.2.0:
	.2.1:
	mov rax, [rbp+16]
	add rax, [rbp+24]
	mov [rbp-40], rax
	mov qword [rbp-48], 1
	mov rax, [rbp+32]
	sub rax, [rbp-48]
	mov [rbp-56], rax
	mov rax, qword [rbp-56]
	push rax
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp+24]
	push rax
	call _2
	add rsp, 24
	mov qword [rbp-32], rax
	mov rax, [rbp-32]
	leave
	ret
