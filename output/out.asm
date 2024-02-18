    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__4: ; printi
	push rbp
	mov rbp, rsp
	sub rsp, 80
	mov rcx, rbp
	dec rcx
	mov rax, qword [rbp+16]
	mov qword [rbp-24], ""
	mov qword [rbp-16], ""
	mov dword [rbp-8], ""
	mov dword [rbp-4], `\0\0\0\n`
	cmp rax, 0
	jg ._4.positive
	mov dword [rbp-20], "-"
	mov r8, rax
	mov rax, 0
	sub rax, r8
	._4.positive:
	mov rbx, 10
	._4.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	test rax, rax
	jnz ._4.loop
	mov ecx, -11
	call GetStdHandle
	mov rcx, rax
	mov rdx, rbp
	sub rdx, 24
	mov r8, 24
	mov qword [rbp - 40], 0
	mov r9, rbp
	sub r9, 24
	mov qword [rbp - 48], 0
	call WriteFile
	leave
	ret

_2: ; fibb
	push rbp
	mov rbp, rsp
	sub rsp, 64
	mov rax, qword 0
	mov qword [rbp-8], rax
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
	mov rax, qword 1
	mov qword [rbp-48], rax
	mov rax, [rbp+32]
	sub rax, [rbp-48]
	mov [rbp-56], rax
	push qword 0
	mov rax, qword [rbp-56]
	push rax
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp+24]
	push rax
	call _2
	add rsp, 32
	mov qword [rbp-32], rax
	mov rax, [rbp-32]
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 64
	mov rax, qword 0
	mov qword [rbp-8], rax
	main.0:
	mov rax, qword 30
	mov qword [rbp-16], rax
	mov rax, [rbp-8]
	mov rcx, [rbp-16]
	cmp rcx, rax
	setle [rbp-24]
	mov rax, [rbp-24]
	cmp rax, 0
	jnz main.1
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	call __4
	add rsp, 16
	mov rax, qword 0
	mov qword [rbp-32], rax
	mov rax, qword 1
	mov qword [rbp-40], rax
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp-32]
	push rax
	call _2
	add rsp, 32
	mov qword [rbp-48], rax
	push qword 0
	mov rax, qword [rbp-48]
	push rax
	call __4
	add rsp, 16
	mov rax, qword 1
	mov qword [rbp-56], rax
	mov rax, [rbp-8]
	add rax, [rbp-56]
	mov [rbp-8], rax
	jmp main.0
	main.1:
	mov rax, qword 1
	mov qword [rbp-64], rax
	mov rcx, [rbp-64]
	call ExitProcess
