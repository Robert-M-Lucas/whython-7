    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 64
	mov rax, qword 12
	mov qword [rbp-8], rax
	mov rax, rbp
	add rax, -8
	mov qword [rbp-24], rax
	mov rax, rbp
	add rax, -24
	mov qword [rbp-16], rax
	mov rax, qword 1
	mov qword [rbp-32], rax
	mov rax, [rbp-8]
	add rax, [rbp-32]
	mov [rbp-8], rax
	mov r9, qword [rbp-16]
	mov rax, qword [r9+0]
	mov qword [rbp-48], rax
	mov r9, qword [rbp-48]
	mov rax, qword [r9+0]
	mov qword [rbp-40], rax
	mov rax, qword [rbp-40]
	mov qword [rbp-56], rax
	mov rcx, [rbp-56]
	call ExitProcess
