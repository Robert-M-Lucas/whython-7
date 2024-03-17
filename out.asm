    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    section .text

__4: ; printi
	push rbp
	mov rbp, rsp
	sub rsp, 80
	; [inline asm]
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

_1: ; test
	push rbp
	mov rbp, rsp
	sub rsp, 0
	; '    printi(a.a);'
	; [no return call] -4 , [(16, 8)], 0
	push qword 0
	mov rax, qword [rbp+16]
	push rax
	call __4
	add rsp, 16
	; '    printi(a.b);'
	; [no return call] -4 , [(24, 8)], 0
	push qword 0
	mov rax, qword [rbp+24]
	push rax
	call __4
	add rsp, 16
	; '    printi(a.c);'
	; [no return call] -4 , [(32, 8)], 0
	push qword 0
	mov rax, qword [rbp+32]
	push rax
	call __4
	add rsp, 16
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; '    let a: A = @A { 1, 2, 3 };'
	; [inline asm]
	mov rax, qword 1
	mov qword [rbp-24], rax
	; [inline asm]
	mov rax, qword 2
	mov qword [rbp-16], rax
	; [inline asm]
	mov rax, qword 3
	mov qword [rbp-8], rax
	; [inline asm]
	; '    test(a);'
	; [no return call] 1 , [(-24, 24)], 0
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	mov rax, qword [rbp-16]
	push rax
	mov rax, qword [rbp-24]
	push rax
	call _1
	add rsp, 32
	; '    test(a);'
	; [no return call] 1 , [(-24, 24)], 0
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	mov rax, qword [rbp-16]
	push rax
	mov rax, qword [rbp-24]
	push rax
	call _1
	add rsp, 32
	; ''
	; '    return 7;'
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-32], rax
	; [return] Some((-32, 8))
	mov rcx, [rbp-32]
	call ExitProcess
