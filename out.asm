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
	sub rsp, 32
	mov rax, qword 1
	mov qword [rbp-16], rax
	mov rax, qword 1
	mov qword [rbp-24], rax
	mov rax, [rbp-16]
	add rax, [rbp-24]
	mov [rbp-8], rax
	mov rax, qword 1
	mov qword [rbp-32], rax
	mov rcx, [rbp-32]
	call ExitProcess
