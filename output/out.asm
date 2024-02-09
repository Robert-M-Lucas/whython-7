	global main
	extern ExitProcess
	section .text
main:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov qword [rbp-8], 10
	mov rcx, [rbp-8]
	call ExitProcess
	mov qword [rbp-16], 20
	mov rcx, [rbp-16]
	call ExitProcess
	mov rcx, 0
	call ExitProcess
