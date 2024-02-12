    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__4:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rcx, rbp
	dec rcx
	dec rcx
	mov qword [rbp-16], 0h
	mov qword [rbp-8], 0000000000000D0Ah
	mov rbx, 10
	._4.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	test rax, rax
	jnz ._4.loop
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
	sub rsp, 24
	mov qword [rbp-8], 0
	main.0:
	mov rax, [rbp-8]
	cmp rax, 0
	jnz main.1
	mov qword [rbp-16], 1
	mov rax, qword [rbp-16]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-8], -1
	jmp main.0
	main.1:
	mov qword [rbp-24], 1
	mov rcx, [rbp-24]
	call ExitProcess
