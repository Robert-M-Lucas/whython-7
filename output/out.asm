	global main
	extern ExitProcess
	section .text
main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 13
	mov rcx, [rbp-8]
	call ExitProcess
