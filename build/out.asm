    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    section .text

__3: ; printb
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; [inline asm]
	sub rsp, 32
	mov qword [rbp-16], "true"
	mov qword [rbp-8], `\r\n`
	mov rax, qword [rbp+16]
	cmp rax, 0
	jz ._3.true
	mov qword [rbp-16], "fals"
	mov qword [rbp-8], `e\r\n`
	._3.true:
	mov ecx, -11
	call GetStdHandle
	mov rcx, rax
	mov rdx, rbp
	sub rdx, 16
	mov r8, 16
	mov qword [rbp - 24], 0
	mov r9, rbp
	sub r9, 24
	mov qword [rbp - 32], 0
	call WriteFile
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
	; '    printb(c == d);'
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-24]
	ucomisd xmm0, qword [rbp-32]
	mov qword [rbp-56], 0
	setne [rbp-56]
	; [no return call] -3 , [(-56, 8)]
	sub rsp, 8
	mov rax, qword [rbp-56]
	mov qword [rbp-72], rax
	call __3
	add rsp, 8
	; ''
	; '    return 7;'
	; [inline asm]
	mov dword [rbp-64], 0x00000007
	mov dword [rbp-60], 0x00000000
	; [return] Some((-64, 8))
	mov rcx, qword [rbp-64]
	call ExitProcess
