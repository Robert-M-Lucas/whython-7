    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text
main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov qword [rbp-8], 3
	mov qword [rbp-16], 8
	mov rax, qword [rbp-16]
	push rax
	mov rax, qword [rbp-8]
	push rax
	call _2
	add rsp, 16
	mov rcx, 0
	call ExitProcess
_2:
	push rbp
	mov rbp, rsp
	sub rsp, 0
	mov rax, qword [rbp+16]
	push rax
	mov rax, qword [rbp+24]
	push rax
	call _2
	add rsp, 16
	leave
	ret
