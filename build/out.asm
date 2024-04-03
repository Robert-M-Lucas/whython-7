    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    extern printf
    section .text

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
	; '    \ printf(c);'
	; ''
	; [inline asm]
	mov dword [rbp-56], 0x00000007
	mov dword [rbp-52], 0x00000000
	; [return] Some((-56, 8))
	mov rcx, qword [rbp-56]
	call ExitProcess
