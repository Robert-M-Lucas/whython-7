    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    section .text

main: ; main
	push rbp
	mov rbp, rsp
	; '    let a: A = test();'
	; [inline asm]
	sub rsp, 24
	; [return call] 1 , [], -24
	push 0
	push 0
	push 0
	call _1
	; [local copy] -48 , -24, 24
	mov rax, qword [rbp-48]
	mov qword [rbp-24], rax
	mov rax, qword [rbp-40]
	mov qword [rbp-16], rax
	mov rax, qword [rbp-32]
	mov qword [rbp-8], rax
	add rsp, 24
	; ''
	; ''
	; '    let b: A = test();'
	; [inline asm]
	sub rsp, 24
	; [return call] 1 , [], -48
	push 0
	push 0
	push 0
	call _1
	; [local copy] -72 , -48, 24
	mov rax, qword [rbp-72]
	mov qword [rbp-48], rax
	mov rax, qword [rbp-64]
	mov qword [rbp-40], rax
	mov rax, qword [rbp-56]
	mov qword [rbp-32], rax
	add rsp, 24
	; ''
	; '    return b.c;'
	; [inline asm]
	sub rsp, 8
	; [local copy] -32 , -56, 8
	mov rax, qword [rbp-32]
	mov qword [rbp-56], rax
	; [return] Some((-56, 8))
	mov rcx, [rbp-56]
	call ExitProcess

_1: ; test
	push rbp
	mov rbp, rsp
	; '    return @A {7, 8, 9};'
	; [inline asm]
	sub rsp, 24
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-24], rax
	; [inline asm]
	mov rax, qword 8
	mov qword [rbp-16], rax
	; [inline asm]
	mov rax, qword 9
	mov qword [rbp-8], rax
	; [inline asm]
	; [return] Some((-24, 24))
	; [local copy] -24 , 16, 24
	mov rax, qword [rbp-24]
	mov qword [rbp+16], rax
	mov rax, qword [rbp-16]
	mov qword [rbp+24], rax
	mov rax, qword [rbp-8]
	mov qword [rbp+32], rax
	leave
	ret
