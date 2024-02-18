    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text

__4: ; printi
	push rbp
	mov rbp, rsp
	sub rsp, 80
	mov rcx, rbp
	dec rcx
	mov rax, qword [rbp+16]
	mov qword [rbp-24], ""
	mov qword [rbp-16], ""
	mov dword [rbp-8], ""
	mov dword [rbp-4], `\0\0\0\n`
	cmp rax, 0
	jge ._4.positive
	mov dword [rbp-20], "-"
	mov r8, rax
	mov rax, 0
	sub rax, r8
	._4.positive:
	mov rbx, 10
	._4.loop:
	xor rdx, rdx
	div rbx
	dec rcx
	add rdx, '0'
	mov [rcx], dl
	test rax, rax
	jnz ._4.loop
	mov ecx, -11
	call GetStdHandle
	mov rcx, rax
	mov rdx, rbp
	sub rdx, 24
	mov r8, 24
	mov qword [rbp - 40], 0
	mov r9, rbp
	sub r9, 24
	mov qword [rbp - 48], 0
	call WriteFile
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 160
	mov qword [rbp-8], 0
	mov qword [rbp-16], 1
	mov rax, qword [rbp-16]
	push rax
	mov rax, qword [rbp-8]
	push rax
	call _6
	add rsp, 16
	mov qword [rbp-24], rax
	push qword 0
	mov rax, qword [rbp-24]
	push rax
	call __4
	add rsp, 16
	mov qword [rbp-32], 0
	mov qword [rbp-40], 1
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp-32]
	push rax
	call _6
	add rsp, 16
	mov qword [rbp-48], rax
	push qword 0
	mov rax, qword [rbp-48]
	push rax
	call __4
	add rsp, 16
	mov qword [rbp-56], 0
	mov qword [rbp-64], 1
	mov rax, qword [rbp-64]
	push rax
	mov rax, qword [rbp-56]
	push rax
	call _6
	add rsp, 16
	mov qword [rbp-72], rax
	push qword 0
	mov rax, qword [rbp-72]
	push rax
	call __4
	add rsp, 16
	call _1
	add rsp, 0
	mov rax, qword 0
	mov qword [rbp-80], rax
	main.0:
	mov rax, qword 100
	mov qword [rbp-88], rax
	mov rax, [rbp-80]
	mov rcx, [rbp-88]
	cmp rcx, rax
	mov qword [rbp-96], 0
	setle [rbp-96]
	mov rax, [rbp-96]
	cmp rax, 0
	jnz main.1
	mov rax, qword 0
	mov qword [rbp-104], rax
	mov rax, qword 1
	mov qword [rbp-112], rax
	push qword 0
	mov rax, qword [rbp-80]
	push rax
	mov rax, qword [rbp-112]
	push rax
	mov rax, qword [rbp-104]
	push rax
	call _4
	add rsp, 32
	mov qword [rbp-120], rax
	push qword 0
	mov rax, qword [rbp-120]
	push rax
	call __4
	add rsp, 16
	mov rax, qword 1
	mov qword [rbp-128], rax
	mov rax, [rbp-80]
	add rax, [rbp-128]
	mov [rbp-80], rax
	jmp main.0
	main.1:
	mov rax, qword 0
	mov qword [rbp-144], rax
	mov rax, qword 1
	mov qword [rbp-152], rax
	mov rax, qword 20
	mov qword [rbp-160], rax
	push qword 0
	mov rax, qword [rbp-160]
	push rax
	mov rax, qword [rbp-152]
	push rax
	mov rax, qword [rbp-144]
	push rax
	call _4
	add rsp, 32
	mov qword [rbp-136], rax
	mov rcx, [rbp-136]
	call ExitProcess

_4: ; fibb
	push rbp
	mov rbp, rsp
	sub rsp, 64
	mov rax, qword 0
	mov qword [rbp-8], rax
	mov rax, [rbp+32]
	mov rcx, [rbp-8]
	cmp rax, rcx
	mov qword [rbp-16], 0
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
	mov rax, qword 1
	mov qword [rbp-48], rax
	mov rax, [rbp+32]
	sub rax, [rbp-48]
	mov [rbp-56], rax
	push qword 0
	mov rax, qword [rbp-56]
	push rax
	mov rax, qword [rbp-40]
	push rax
	mov rax, qword [rbp+24]
	push rax
	call _4
	add rsp, 32
	mov qword [rbp-32], rax
	mov rax, [rbp-32]
	leave
	ret

_1: ; extern
	push rbp
	mov rbp, rsp
	sub rsp, 16
	mov rax, qword 101
	mov qword [rbp-8], rax
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	call __4
	add rsp, 16
	leave
	ret

_6: ; add
	push rbp
	mov rbp, rsp
	sub rsp, 32
	mov rax, qword 0
	mov qword [rbp-8], rax
	mov rax, [rbp+16]
	cmp rax, 0
	jnz .6.0
	mov rax, qword 1
	mov qword [rbp-16], rax
	mov rax, [rbp-8]
	add rax, [rbp-16]
	mov [rbp-8], rax
	.6.0:
	.6.1:
	mov rax, [rbp+24]
	cmp rax, 0
	jnz .6.2
	mov rax, qword 1
	mov qword [rbp-24], rax
	mov rax, [rbp-8]
	add rax, [rbp-24]
	mov [rbp-8], rax
	.6.2:
	.6.3:
	mov rax, [rbp-8]
	leave
	ret
