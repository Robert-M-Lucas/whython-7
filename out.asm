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

__14: ; printb
	push rbp
	mov rbp, rsp
	sub rsp, 64
	; [inline asm]
	mov qword [rbp-16], "true"
	mov qword [rbp-8], `\n\r`
	mov rax, qword [rbp+16]
	cmp rax, 0
	jz ._14.true
	mov qword [rbp-16], "fals"
	mov qword [rbp-8], `e\n\r`
	._14.true:
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

_1: ; new
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '        return @LL {'
	; [inline asm]
	mov rax, qword 0
	mov qword [rbp-16], rax
	; [inline asm]
	mov qword [rbp-8], 1
	; [inline asm]
	; [return] Some(-16)
	mov rax, qword [rbp-16]
	mov r14, qword [rbp-8]
	leave
	ret

_3: ; test
	push rbp
	mov rbp, rsp
	sub rsp, 48
	; '    let ll: LL = LL#new();'
	; [no return call] 1 , [], -16
	call _1
	mov qword [rbp-16], rax
	mov qword [rbp-8], r14
	; '    let b: bool = ll.has_first;'
	; [dyn to copy] -8 , -24, 8
	mov rax, qword [rbp-8]
	mov qword [rbp-24], rax
	mov r15, qword [rbp-24]
	; '    printb(b);'
	; [no return call] -14 , [(-24, 8)]
	push qword 0
	mov rax, qword [rbp-24]
	push rax
	call __14
	add rsp, 16
	; ''
	; '    let a: $int = 0;'
	; [inline asm]
	mov rax, qword 0
	mov qword [rbp-32], rax
	; '    *a = 1;'
	; [dyn from copy] -32 , -40, 8
	mov r9, qword [rbp-32]
	mov rax, qword [r9+0]
	mov qword [rbp-40], rax
	; [inline asm]
	mov rax, qword 1
	mov qword [rbp-40], rax
	; [dyn to copy] -40 , -32, 8
	mov r9, qword [rbp-32]
	mov rax, qword [rbp-40]
	mov qword [r9+0], rax
	; [inline asm]
	mov rax, rbp
	add rax, -16
	mov qword [rbp-48], rax
	; [no return call] 2 , [(-48, 8)]
	push qword 0
	mov rax, qword [rbp-48]
	push rax
	call _2
	add rsp, 16
	leave
	ret

_2: ; destroy
	push rbp
	mov rbp, rsp
	sub rsp, 48
	; '        if (!(*self.has_first)) {'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-8], rax
	; [dyn from copy] -8 , -16, 8
	mov r9, qword [rbp-8]
	mov rax, qword [r9+0]
	mov qword [rbp-16], rax
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-16]
	cmp rax, 0
	setz al
	mov qword [rbp-24], rax
	; [inline asm]
	mov rax, qword [rbp-24]
	cmp rax, 0
	jnz .2.0
	; '            printi(119);'
	; [inline asm]
	mov rax, qword 119
	mov qword [rbp-32], rax
	; [no return call] -4 , [(-32, 8)]
	push qword 0
	mov rax, qword [rbp-32]
	push rax
	call __4
	add rsp, 16
	; '            return;'
	; [return] None
	leave
	ret
	; [inline asm]
	.2.0:
	.2.1:
	; '            printi(119);'
	; '            return;'
	; '        };'
	; ''
	; '        printi(120);'
	; [inline asm]
	mov rax, qword 120
	mov qword [rbp-40], rax
	; [no return call] -4 , [(-40, 8)]
	push qword 0
	mov rax, qword [rbp-40]
	push rax
	call __4
	add rsp, 16
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '    test();'
	; [no return call] 3 , []
	call _3
	; ''
	; '    return 7;'
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-8], rax
	; [return] Some(-8)
	mov rcx, [rbp-8]
	call ExitProcess
