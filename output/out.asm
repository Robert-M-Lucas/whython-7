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
	sub rsp, 24
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

_1:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 101
	mov rax, qword [rbp-8]
	push rax
	call __4
	add rsp, 8
	leave
	ret

_6:
	push rbp
	mov rbp, rsp
	sub rsp, 40
	mov qword [rbp-8], 0
	mov rax, [rbp+16]
	cmp rax, 0
	jnz .6.0
	mov qword [rbp-16], 1
	mov rax, [rbp-8]
	add rax, [rbp-16]
	mov [rbp-8], rax
	.6.0:
	.6.1:
	mov rax, [rbp+24]
	cmp rax, 0
	jnz .6.2
	mov qword [rbp-24], 1
	mov rax, [rbp-8]
	add rax, [rbp-24]
	mov [rbp-8], rax
	.6.2:
	.6.3:
	mov rax, [rbp-8]
	leave
	ret

_4:
	push rbp
	mov rbp, rsp
	sub rsp, 56
	mov qword [rbp-8], 0
	mov rax, [rbp+32]
	mov rcx, [rbp-8]
	cmp rax, rcx
	setnle [rbp-16]
	mov rax, [rbp-16]
	cmp rax, 0
	jnz .4.0
	mov rax, [rbp+16]
	leave
	ret
	.4.0:
	.4.1:
	mov rax, [rbp+16]
	add rax, [rbp+24]
	mov [rbp-40], rax
	mov qword [rbp-48], 1
	mov rax, [rbp+32]
	sub rax, [rbp-48]
	mov [rbp-56], rax
	mov rax, qword [rbp-56]
	push rax
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp+24]
	push rax
	call _4
	add rsp, 24
	mov qword [rbp-32], rax
	mov rax, [rbp-32]
	leave
	ret

main:
	push rbp
	mov rbp, rsp
	sub rsp, 168
	mov qword [rbp-8], 0
	mov qword [rbp-16], 1
	mov rax, qword [rbp-16]
	push rax
	mov rax, qword [rbp-8]
	push rax
	call _6
	add rsp, 16
	mov qword [rbp-24], rax
	mov rax, qword [rbp-24]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-32], 0
	mov qword [rbp-40], 1
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp-32]
	push rax
	call _6
	add rsp, 16
	mov qword [rbp-48], rax
	mov rax, qword [rbp-48]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-56], 0
	mov qword [rbp-64], 1
	mov rax, qword [rbp-64]
	push rax
	mov rax, qword [rbp-56]
	push rax
	call _6
	add rsp, 16
	mov qword [rbp-72], rax
	mov rax, qword [rbp-72]
	push rax
	call __4
	add rsp, 8
	call _1
	add rsp, 0
	mov qword [rbp-80], 0
	main.0:
	mov qword [rbp-88], 20
	mov rax, [rbp-80]
	mov rcx, [rbp-88]
	cmp rcx, rax
	setle [rbp-96]
	mov rax, [rbp-96]
	cmp rax, 0
	jnz main.1
	mov qword [rbp-104], 0
	mov qword [rbp-112], 1
	mov rax, qword [rbp-80]
	push rax
	mov rax, qword [rbp-112]
	push rax
	mov rax, qword [rbp-104]
	push rax
	call _4
	add rsp, 24
	mov qword [rbp-120], rax
	mov rax, qword [rbp-120]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-128], 1
	mov rax, [rbp-80]
	add rax, [rbp-128]
	mov [rbp-80], rax
	jmp main.0
	main.1:
	mov qword [rbp-144], 0
	mov qword [rbp-152], 1
	mov qword [rbp-160], 20
	mov rax, qword [rbp-160]
	push rax
	mov rax, qword [rbp-152]
	push rax
	mov rax, qword [rbp-144]
	push rax
	call _4
	add rsp, 24
	mov qword [rbp-136], rax
	mov rcx, [rbp-136]
	call ExitProcess
