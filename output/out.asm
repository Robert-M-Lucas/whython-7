    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__2:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rcx, rbp
	mov qword [rbp-16], 0D0Ah
	mov qword [rbp-8], 0h
	mov rbx, 10
	._2.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	test rax, rax
	jnz ._2.loop
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
	sub rsp, 72
	mov qword [rbp-8], 2
	mov qword [rbp-16], 3
	mov rax, [rbp-8]
	mov rcx, [rbp-16]
	mul rcx
	mov [rbp-24], rax
	mov rax, qword [rbp-24]
	push rax
	call __2
	add rsp, 8
	mov qword [rbp-32], 9
	mov qword [rbp-40], 2
	mov rax, [rbp-32]
	mov rcx, [rbp-40]
	div rcx
	mov [rbp-48], rax
	mov rax, qword [rbp-48]
	push rax
	call __2
	add rsp, 8
	mov qword [rbp-64], 1
	mov qword [rbp-72], 3
	mov rax, [rbp-64]
	sub rax, [rbp-72]
	mov [rbp-56], rax
	mov rcx, [rbp-56]
	call ExitProcess
