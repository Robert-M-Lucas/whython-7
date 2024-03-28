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
	sub rsp, 48
	; [inline asm]
	sub rsp, 32
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
	; '    printi(a.b);'
	; [no return call] -4 , [(16, 8)]
	sub rsp, 8
	mov rax, qword [rbp+16]
	mov qword [rbp-8], rax
	call __4
	add rsp, 8
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '    test(@A { 12 });'
	; [inline asm]
	mov rax, qword 12
	mov qword [rbp-8], rax
	; [inline asm]
	; [no return call] 1 , [(-8, 8)]
	sub rsp, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-24], rax
	call _1
	add rsp, 8
	; '    return 13;'
	; [inline asm]
	mov rax, qword 13
	mov qword [rbp-16], rax
	; [return] Some((-16, 8))
	mov rcx, qword [rbp-16]
	call ExitProcess
