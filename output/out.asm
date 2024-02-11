    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text
__1:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rcx, rbp
	mov qword [rbp-16], 0D0Ah
	mov qword [rbp-8], 0h
	mov rbx, 10
	._1.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	test rax, rax
	jnz ._1.loop
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
_2:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rax, [rbp+16]
	add rax, [rbp+24]
	mov [rbp-16], rax
	mov rax, [rbp-16]
	add rax, [rbp+24]
	mov [rbp-8], rax
	mov rax, [rbp-8]
	leave
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 48
	mov qword [rbp-8], 2
	mov qword [rbp-16], 4
	mov rax, qword [rbp-16]
	push rax
	mov rax, qword [rbp-8]
	push rax
	call _2
	add rsp, 16
	mov qword [rbp-24], rax
	mov rax, qword [rbp-24]
	push rax
	call __1
	add rsp, 8
	mov qword [rbp-40], 1
	mov qword [rbp-48], 3
	mov rax, [rbp-40]
	add rax, [rbp-48]
	mov [rbp-32], rax
	mov rcx, [rbp-32]
	call ExitProcess
