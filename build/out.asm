    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    extern printf
    section .text

__2: ; printi
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; [inline asm]
	mov dword [rbp-4], 0x000a
	mov dword [rbp-8], 0x646C6C25
	mov rcx, rbp
	sub rcx, 8
	mov rdx, qword [rbp+16]
	sub rsp, 40
	call printf
	add rsp, 40
	leave
	ret

__3: ; printb
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; [inline asm]
	mov dword [rbp-8], 0x65757274
	mov dword [rbp-4], 0x0D0A
	mov rax, qword [rbp+16]
	cmp rax, 0
	jz ._3.true
	mov dword [rbp-8], 0x736C6166
	mov dword [rbp-4], 0x0D0A65
	._3.true:
	mov rcx, rbp
	sub rcx, 8
	mov rdx, qword [rbp+16]
	sub rsp, 40
	call printf
	add rsp, 40
	leave
	ret

__37: ; printf
	push rbp
	mov rbp, rsp
	sub rsp, 16
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
	sub rsp, 112
	; '    printb(true);'
	; [inline asm]
	mov qword [rbp-8], 0
	; [no return call] -3 , [(-8, 8)]
	sub rsp, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-120], rax
	call __3
	add rsp, 8
	; '    printb(true);'
	; [inline asm]
	mov qword [rbp-16], 0
	; [no return call] -3 , [(-16, 8)]
	sub rsp, 8
	mov rax, qword [rbp-16]
	mov qword [rbp-120], rax
	call __3
	add rsp, 8
	; '    printb(false);'
	; [inline asm]
	mov qword [rbp-24], 1
	; [no return call] -3 , [(-24, 8)]
	sub rsp, 8
	mov rax, qword [rbp-24]
	mov qword [rbp-120], rax
	call __3
	add rsp, 8
	; '    printb(false);'
	; [inline asm]
	mov qword [rbp-32], 1
	; [no return call] -3 , [(-32, 8)]
	sub rsp, 8
	mov rax, qword [rbp-32]
	mov qword [rbp-120], rax
	call __3
	add rsp, 8
	; '    printi(9223372036854775808);'
	; [inline asm]
	mov dword [rbp-40], 0x00000000
	mov dword [rbp-36], 0x80000000
	; [no return call] -2 , [(-40, 8)]
	sub rsp, 8
	mov rax, qword [rbp-40]
	mov qword [rbp-120], rax
	call __2
	add rsp, 8
	; '    printi(-9223372036854775807);'
	; [inline asm]
	mov dword [rbp-48], 0xffffffff
	mov dword [rbp-44], 0x7fffffff
	; [inline asm]
	mov dword [rbp-56], 0x00000000
	mov dword [rbp-52], 0x00000000
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-56]
	sub rax, [rbp-48]
	mov [rbp-64], rax
	; [no return call] -2 , [(-64, 8)]
	sub rsp, 8
	mov rax, qword [rbp-64]
	mov qword [rbp-120], rax
	call __2
	add rsp, 8
	; '    printf(1.12351234123);'
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-72], rax
	; [no return call] -37 , [(-72, 8)]
	sub rsp, 8
	mov rax, qword [rbp-72]
	mov qword [rbp-120], rax
	call __37
	add rsp, 8
	; '    printf(1.12351234123 - 2.0);'
	; [inline asm]
	mov rax, __float64__(1.12351234123)
	mov qword [rbp-80], rax
	; [inline asm]
	mov rax, __float64__(2.0)
	mov qword [rbp-88], rax
	; [inline asm]
	; [inline asm]
	movsd xmm0, qword [rbp-80]
	subsd xmm0, qword [rbp-88]
	movsd qword [rbp-96], xmm0
	; [no return call] -37 , [(-96, 8)]
	sub rsp, 8
	mov rax, qword [rbp-96]
	mov qword [rbp-120], rax
	call __37
	add rsp, 8
	; ''
	; ''
	; '    return 7;'
	; [inline asm]
	mov dword [rbp-104], 0x00000007
	mov dword [rbp-100], 0x00000000
	; [return] Some((-104, 8))
	mov rcx, qword [rbp-104]
	call ExitProcess

formatStr:
	db `The int is %d\n`,0