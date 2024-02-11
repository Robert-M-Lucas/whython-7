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

__14:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rcx, rbp
	sub rcx, 8
	mov qword [rbp-16], rcx
	mov qword [rbp-8], 0000657572740A0Dh
	mov rax, [rbp+16]
	cmp rax, 0
	jz ._14.true
	mov qword [rbp-8], 0065736C61660A0Dh
	._14.true:
	sub rsp, 48
	mov ecx, -11
	call GetStdHandle
	mov rcx, rax
	mov rdx, [rbp-16]
	mov qword [rsp + 40], 08h
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
	sub rsp, 168
	mov qword [rbp-8], -1
	mov rax, qword [rbp-8]
	push rax
	call __14
	add rsp, 8
	mov qword [rbp-16], -1
	mov rax, [rbp-16]
	not rax
	mov [rbp-24], rax
	mov rax, qword [rbp-24]
	push rax
	call __14
	add rsp, 8
	mov qword [rbp-32], 0
	mov rax, qword [rbp-32]
	push rax
	call __14
	add rsp, 8
	mov qword [rbp-40], 0
	mov rax, [rbp-40]
	not rax
	mov [rbp-48], rax
	mov rax, qword [rbp-48]
	push rax
	call __14
	add rsp, 8
	mov qword [rbp-56], 13
	mov qword [rbp-64], 3
	mov rax, [rbp-56]
	mov rcx, [rbp-64]
	mul rcx
	mov [rbp-72], rax
	mov rax, qword [rbp-72]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-80], 13
	mov qword [rbp-88], 3
	mov rax, [rbp-80]
	mov rcx, [rbp-88]
	div rcx
	mov [rbp-96], rax
	mov rax, qword [rbp-96]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-104], 13
	mov qword [rbp-112], 3
	mov rax, [rbp-104]
	add rax, [rbp-112]
	mov [rbp-120], rax
	mov rax, qword [rbp-120]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-128], 13
	mov qword [rbp-136], 3
	mov rax, [rbp-128]
	sub rax, [rbp-136]
	mov [rbp-144], rax
	mov rax, qword [rbp-144]
	push rax
	call __4
	add rsp, 8
	mov qword [rbp-160], 1
	mov qword [rbp-168], 3
	mov rax, [rbp-160]
	sub rax, [rbp-168]
	mov [rbp-152], rax
	mov rcx, [rbp-152]
	call ExitProcess
