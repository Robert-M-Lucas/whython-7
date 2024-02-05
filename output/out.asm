	global main
	section .text
main:
	push rbp
	mov rbp, rsp
	sub rsp, 64
	leave
	ret
