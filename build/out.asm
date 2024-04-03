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
	mov rdi,formatStr ; first argument: format string
	mov rsi,5 ; second argument (for format string below): integer to print
	mov al,0 ; magic for varargs (0==no magic, to prevent a crash!)
	call printf
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 64
	; '    let a: float = 2.43;'
	; [inline asm]
	mov rax, __float64__(2.43)
	mov qword [rbp-8], rax
	; '    let b: float = 3.21;'
	; [inline asm]
	mov rax, __float64__(3.21)
	mov qword [rbp-16], rax
	; ''
	; '    let c: float = a * b;'
	; [inline asm]
	movsd xmm0, qword [rbp-8]
	mulsd xmm0, qword [rbp-16]
	movsd qword [rbp-24], xmm0
	; '    let d: float = (b * a) + 0.001;'
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-16]
	mulsd xmm0, qword [rbp-8]
	movsd qword [rbp-40], xmm0
	; [inline asm]
	mov rax, __float64__(0.001)
	mov qword [rbp-48], rax
	; [inline asm]
	movsd xmm0, qword [rbp-40]
	addsd xmm0, qword [rbp-48]
	movsd qword [rbp-32], xmm0
	; ''
	; '    printf(c);'
	; [no return call] -37 , [(-24, 8)]
	sub rsp, 8
	mov rax, qword [rbp-24]
	mov qword [rbp-72], rax
	call __37
	add rsp, 8
	; ''
	; '    return 7;'
	; [inline asm]
	mov dword [rbp-56], 0x00000007
	mov dword [rbp-52], 0x00000000
	; [return] Some((-56, 8))
	mov rcx, qword [rbp-56]
	call ExitProcess
