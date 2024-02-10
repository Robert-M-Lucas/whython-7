	global main
	extern ExitProcess
	section .text
.6:
	push rbp
	mov rbp, rsp
	sub rsp, 0
	mov rcx, [rbp+24]
	call ExitProcess
	leave
	ret
.2:
	push rbp
	mov rbp, rsp
	sub rsp, 0
	leave
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	call .2
	add rsp, 0
	mov qword [rbp-8], 2
	mov rax, qword [rbp-8]
	push rax
	call .4
	add rsp, 8
	mov rcx, 0
	call ExitProcess
.4:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 0
	mov rax, qword [rbp+16]
	push rax
	mov rax, qword [rbp-8]
	push rax
	call .6
	add rsp, 16
	leave
	ret
