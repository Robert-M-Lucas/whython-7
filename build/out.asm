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

_1: ; fact
	push rbp
	mov rbp, rsp
	sub rsp, 80
	; '	if (x == 0.0) {'
	; [inline asm]
	mov rax, __float64__(0.0)
	mov qword [rbp-8], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp+24]
	ucomisd xmm0, qword [rbp-8]
	mov qword [rbp-16], 0
	setne [rbp-16]
	; [inline asm]
	mov rax, qword [rbp-16]
	cmp rax, 0
	jnz .1.0
	; '        return 1.0;'
	; [inline asm]
	mov rax, __float64__(1.0)
	mov qword [rbp-24], rax
	; [return] Some((-24, 8))
	; [local copy] -24 , 16, 8
	mov rax, qword [rbp-24]
	mov qword [rbp+16], rax
	leave
	ret
	; [inline asm]
	.1.0:
	.1.1:
	; '        return 1.0;'
	; '    };'
	; ''
	; '    let f: float = 1.0;'
	; [inline asm]
	mov rax, __float64__(1.0)
	mov qword [rbp-32], rax
	; '    let i: float = 1.0;'
	; [inline asm]
	mov rax, __float64__(1.0)
	mov qword [rbp-40], rax
	; ''
	; '    while (i < (x + 0.99)) {'
	; [inline asm]
	.1.2:
	; [inline asm]
	mov rax, __float64__(0.99)
	mov qword [rbp-48], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp+24]
	addsd xmm0, qword [rbp-48]
	movsd qword [rbp-56], xmm0
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-40]
	ucomisd xmm0, qword [rbp-56]
	mov qword [rbp-64], 0
	seta [rbp-64]
	; [inline asm]
	mov rax, qword [rbp-64]
	cmp rax, 0
	jnz .1.3
	; '        f *= i;'
	; [inline asm]
	movsd xmm0, qword [rbp-32]
	mulsd xmm0, qword [rbp-40]
	movsd qword [rbp-32], xmm0
	; '        i += 1.0;'
	; [inline asm]
	mov rax, __float64__(1.0)
	mov qword [rbp-72], rax
	; [inline asm]
	movsd xmm0, qword [rbp-40]
	addsd xmm0, qword [rbp-72]
	movsd qword [rbp-40], xmm0
	; [inline asm]
	jmp .1.2
	.1.3:
	; '        f *= i;'
	; '        i += 1.0;'
	; '    };'
	; ''
	; '    return f;'
	; [local copy] -32 , -80, 8
	mov rax, qword [rbp-32]
	mov qword [rbp-80], rax
	; [return] Some((-80, 8))
	; [local copy] -80 , 16, 8
	mov rax, qword [rbp-80]
	mov qword [rbp+16], rax
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 80
	; '    let e: float = 0.0;'
	; [inline asm]
	mov rax, __float64__(0.0)
	mov qword [rbp-8], rax
	; ''
	; '    let i: float = 0.0;'
	; [inline asm]
	mov rax, __float64__(0.0)
	mov qword [rbp-16], rax
	; '    while (i < 10.0) {'
	; [inline asm]
	main.0:
	; [inline asm]
	mov rax, __float64__(10.0)
	mov qword [rbp-24], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-16]
	ucomisd xmm0, qword [rbp-24]
	mov qword [rbp-32], 0
	seta [rbp-32]
	; [inline asm]
	mov rax, qword [rbp-32]
	cmp rax, 0
	jnz main.1
	; '        e += 1.0 / fact(i);'
	; [inline asm]
	mov rax, __float64__(1.0)
	mov qword [rbp-40], rax
	; [return call] 1 , [(-16, 8)], -48
	sub rsp, 8
	mov rax, qword [rbp-16]
	mov qword [rbp-88], rax
	sub rsp, 8
	call _1
	; [local copy] -96 , -48, 8
	mov rax, qword [rbp-96]
	mov qword [rbp-48], rax
	add rsp, 16
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-40]
	divsd xmm0, qword [rbp-48]
	movsd qword [rbp-56], xmm0
	; [inline asm]
	movsd xmm0, qword [rbp-8]
	addsd xmm0, qword [rbp-56]
	movsd qword [rbp-8], xmm0
	; '        printf(e);'
	; [no return call] -37 , [(-8, 8)]
	sub rsp, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-88], rax
	call __37
	add rsp, 8
	; '        i += 1.0;'
	; [inline asm]
	mov rax, __float64__(1.0)
	mov qword [rbp-64], rax
	; [inline asm]
	movsd xmm0, qword [rbp-16]
	addsd xmm0, qword [rbp-64]
	movsd qword [rbp-16], xmm0
	; [inline asm]
	jmp main.0
	main.1:
	; '        e += 1.0 / fact(i);'
	; '        printf(e);'
	; '        i += 1.0;'
	; '    };'
	; ''
	; '    printf(e);'
	; [no return call] -37 , [(-8, 8)]
	sub rsp, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-88], rax
	call __37
	add rsp, 8
	; ''
	; '    return 7;'
	; [inline asm]
	mov dword [rbp-72], 0x00000007
	mov dword [rbp-68], 0x00000000
	; [return] Some((-72, 8))
	mov rcx, qword [rbp-72]
	call ExitProcess

formatStr:
	db `The int is %d\n`,0