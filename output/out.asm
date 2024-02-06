	global main
	extern ExitProcess
	section .text
main:
	push rbp
	mov rbp, rsp
	sub rsp, 0
	mov rcx, 0
	call ExitProcess
