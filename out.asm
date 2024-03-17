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

_3: ; test_two
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '    return 103;'
	; [inline asm]
	mov rax, qword 103
	mov qword [rbp-8], rax
	; [return] Some((-8, 8))
	; [local copy] -8 , 16, 8
	mov rax, qword [rbp-8]
	mov qword [rbp+16], rax
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '    let t_t: int = test_two();'
	; [return call] 3 , [], -8
	push 0
	call _3
	; [local copy] -40 , -8, 8
	mov rax, qword [rbp-40]
	mov qword [rbp-8], rax
	; ''
	; '    printi(t_t);'
	; [no return call] -4 , [(-8, 8)], 0
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	call __4
	add rsp, 16
	; ''
	; '    return 7;'
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-16], rax
	; [return] Some((-16, 8))
	mov rcx, [rbp-16]
	call ExitProcess
