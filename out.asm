    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    section .text

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
	; [return] Some((-16, 16))
	; [local copy] -16 , 16, 16
	mov rax, qword [rbp-16]
	mov qword [rbp+16], rax
	mov rax, qword [rbp-8]
	mov qword [rbp+24], rax
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '    test();'
	; [no return call] 4 , [], 0
	call _4
	; ''
	; '    return 7;'
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-8], rax
	; [return] Some((-8, 8))
	mov rcx, [rbp-8]
	call ExitProcess

_4: ; test
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; '    let ll: LL = LL#new();'
	; [return call] 1 , [], -16
	push 0
	push 0
	call _1
	; [local copy] -32 , -16, 16
	mov rax, qword [rbp-32]
	mov qword [rbp-16], rax
	mov rax, qword [rbp-24]
	mov qword [rbp-8], rax
	add rsp, 16
	; [inline asm]
	mov rax, rbp
	add rax, -16
	mov qword [rbp-24], rax
	; [no return call] 3 , [(-24, 8)], 0
	mov rax, qword [rbp-24]
	push rax
	call _3
	add rsp, 8
	leave
	ret

_3: ; destroy
	push rbp
	mov rbp, rsp
	sub rsp, 128
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
	jnz .3.0
	; '            return;'
	; [return] None
	leave
	ret
	; [inline asm]
	.3.0:
	.3.1:
	; '            return;'
	; '        };'
	; ''
	; '        let curr: $Node = *self.base;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-40], rax
	; [dyn from copy] -40 , -32, 8
	mov r9, qword [rbp-40]
	mov rax, qword [r9+0]
	mov qword [rbp-32], rax
	; ''
	; '        while (!(*curr.last)) {'
	; [inline asm]
	.3.2:
	; [inline asm]
	mov rax, qword [rbp-32]
	add rax, 8
	mov qword [rbp-48], rax
	; [dyn from copy] -48 , -56, 8
	mov r9, qword [rbp-48]
	mov rax, qword [r9+0]
	mov qword [rbp-56], rax
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-56]
	cmp rax, 0
	setz al
	mov qword [rbp-64], rax
	; [inline asm]
	mov rax, qword [rbp-64]
	cmp rax, 0
	jnz .3.3
	; '            let prev: $Node = curr;'
	; [local copy] -32 , -72, 8
	mov rax, qword [rbp-32]
	mov qword [rbp-72], rax
	; '            curr = *curr.next;'
	; [inline asm]
	mov rax, qword [rbp-32]
	add rax, 16
	mov qword [rbp-80], rax
	; [dyn from copy] -80 , -32, 8
	mov r9, qword [rbp-80]
	mov rax, qword [r9+0]
	mov qword [rbp-32], rax
	; '            ¬prev;'
	; [heap dealloc] -72 , -88
	call GetProcessHeap
	mov rcx, rax
	mov rdx, 0
	mov r8, qword [rbp-72]
	call HeapFree
	cmp rax, 0
	mov rcx, 0
	setz cl
	mov qword [rbp-88], rcx
	; [inline asm]
	jmp .3.2
	.3.3:
	; '            let prev: $Node = curr;'
	; '            curr = *curr.next;'
	; '            ¬prev;'
	; '        };'
	; ''
	; '        ¬curr;'
	; [heap dealloc] -32 , -96
	call GetProcessHeap
	mov rcx, rax
	mov rdx, 0
	mov r8, qword [rbp-32]
	call HeapFree
	cmp rax, 0
	mov rcx, 0
	setz cl
	mov qword [rbp-96], rcx
	; ''
	; '        *self.has_first = false;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-104], rax
	; [dyn from copy] -104 , -112, 8
	mov r9, qword [rbp-104]
	mov rax, qword [r9+0]
	mov qword [rbp-112], rax
	; [inline asm]
	mov qword [rbp-112], 1
	; [dyn to copy] -112 , -104, 8
	mov r9, qword [rbp-104]
	mov rax, qword [rbp-112]
	mov qword [r9+0], rax
	; '        *self.base = 0;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-120], rax
	; [dyn from copy] -120 , -128, 8
	mov r9, qword [rbp-120]
	mov rax, qword [r9+0]
	mov qword [rbp-128], rax
	; [inline asm]
	mov rax, qword 0
	mov qword [rbp-128], rax
	; [dyn to copy] -128 , -120, 8
	mov r9, qword [rbp-120]
	mov rax, qword [rbp-128]
	mov qword [r9+0], rax
	leave
	ret
