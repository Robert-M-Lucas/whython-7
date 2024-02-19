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
	jge ._4.positive
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

__14: ; printb
	push rbp
	mov rbp, rsp
	sub rsp, 64
	mov qword [rbp-16], "true"
	mov qword [rbp-8], `\n\r`
	mov rax, [rbp+16]
	cmp rax, 0
	jz ._14.true
	mov qword [rbp-16], "fals"
	mov qword [rbp-8], `e\n\r`
	._14.true:
	mov ecx, -11
	call GetStdHandle
	mov rcx, rax
	mov rdx, rbp
	sub rdx, 16
	mov r8, 16
	mov qword [rbp - 24], 0
	mov r9, rbp
	sub r9, 24
	mov qword [rbp - 32], 0
	call WriteFile
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 48
	mov rax, qword 1
	mov qword [rbp-8], rax
	mov rax, qword 2
	mov qword [rbp-16], rax
	mov qword [rbp-24], 1
	push qword 0
	mov rax, qword [rbp-24]
	push rax
	call __14
	add rsp, 16
	mov rax, qword 7
	mov qword [rbp-40], rax
	mov rax, qword 0
	mov qword [rbp-48], rax
	mov rax, [rbp-48]
	sub rax, [rbp-40]
	mov [rbp-32], rax
	mov rcx, [rbp-32]
	call ExitProcess
