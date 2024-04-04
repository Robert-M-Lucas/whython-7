    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    extern printf
    section .text

__37: ; printf
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; [inline asm]
	mov dword [rbp-4], 0x00
	mov dword [rbp-8], 0x0a666C25
	mov rcx, rbp
	sub rcx, 8
	movq xmm1, qword [rbp+16]
	movq rdx, xmm1
	sub rsp, 40
	call printf
	add rsp, 40
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 48
	; '    let a: float = 2.43;'
	; [inline asm]
	mov rax, __float64__(2.43)
	mov qword [rbp-8], rax
	; ''
	; ''
	; '    while (a < 10.0) {'
	; [inline asm]
	main.0:
	; [inline asm]
	mov rax, __float64__(10.0)
	mov qword [rbp-16], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-8]
	ucomisd xmm0, qword [rbp-16]
	mov qword [rbp-24], 0
	seta [rbp-24]
	; [inline asm]
	mov rax, qword [rbp-24]
	cmp rax, 0
	jnz main.1
	; '        a += 0.5;'
	; [inline asm]
	mov rax, __float64__(0.5)
	mov qword [rbp-32], rax
	; [inline asm]
	movsd xmm0, qword [rbp-8]
	addsd xmm0, qword [rbp-32]
	movsd qword [rbp-8], xmm0
	; '        printf(a);'
	; [no return call] -37 , [(-8, 8)]
	sub rsp, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-56], rax
	call __37
	add rsp, 8
	; [inline asm]
	jmp main.0
	main.1:
	; '        a += 0.5;'
	; '        printf(a);'
	; '    };'
	; ''
	; ''
	; '    return 7;'
	; [inline asm]
	mov dword [rbp-40], 0x00000007
	mov dword [rbp-36], 0x00000000
	; [return] Some((-40, 8))
	mov rcx, qword [rbp-40]
	call ExitProcess

formatStr:
	db `The int is %d\n`,0