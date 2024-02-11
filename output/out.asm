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
	sub rsp, 120
	mov qword [rbp-8], 13
	mov qword [rbp-16], 3
	mov rax, [rbp-8]
	mov rcx, [rbp-16]
	mul rcx
	mov [rbp-24], rax
	mov rax, qword [rbp-24]
	push rax
	call __2
	add rsp, 8
	mov qword [rbp-32], 13
	mov qword [rbp-40], 3
	mov rax, [rbp-32]
	mov rcx, [rbp-40]
	div rcx
	mov [rbp-48], rax
	mov rax, qword [rbp-48]
	push rax
	call __2
	add rsp, 8
	mov qword [rbp-56], 13
	mov qword [rbp-64], 3
	mov rax, [rbp-56]
	add rax, [rbp-64]
	mov [rbp-72], rax
	mov rax, qword [rbp-72]
	push rax
	call __2
	add rsp, 8
	mov qword [rbp-80], 13
	mov qword [rbp-88], 3
	mov rax, [rbp-80]
	sub rax, [rbp-88]
	mov [rbp-96], rax
	mov rax, qword [rbp-96]
	push rax
	call __2
	add rsp, 8
	mov qword [rbp-112], 1
	mov qword [rbp-120], 3
	mov rax, [rbp-112]
	sub rax, [rbp-120]
	mov [rbp-104], rax
	mov rcx, [rbp-104]
	call ExitProcess
