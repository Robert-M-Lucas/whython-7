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

_5: ; test
	push rbp
	mov rbp, rsp
	sub rsp, 32
	; '    let ll: LL = LL#new();'
	; [no return call] 1 , [], -16
	call _1
	mov qword [rbp-16], rax
	; [inline asm]
	mov rax, rbp
	add rax, -16
	mov qword [rbp-24], rax
	; [no return call] 4 , [(-24, 8)]
	push qword 0
	mov rax, qword [rbp-24]
	push rax
	call _4
	add rsp, 16
	leave
	ret

_4: ; destroy
	push rbp
	mov rbp, rsp
	sub rsp, 144
	; '        printi(123);'
	; [inline asm]
	mov rax, qword 123
	mov qword [rbp-8], rax
	; [no return call] -4 , [(-8, 8)]
	push qword 0
	mov rax, qword [rbp-8]
	push rax
	call __4
	add rsp, 16
	; '        if (!(*self.has_first)) {'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-16], rax
	; [dyn from copy] -16 , -24, 8
	mov r9, qword [rbp-16]
	mov rax, qword [r9+0]
	mov qword [rbp-24], rax
	; [inline asm]
	; [inline asm]
	mov rax, [rbp-24]
	cmp rax, 0
	setz al
	mov qword [rbp-32], rax
	; [inline asm]
	mov rax, [rbp-32]
	cmp rax, 0
	jnz .4.0
	; '            return;'
	; [return] None
	leave
	ret
	; [inline asm]
	.4.0:
	.4.1:
	; '            return;'
	; '        };'
	; ''
	; '        let curr: $Node = *self.base;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-48], rax
	; [dyn from copy] -48 , -40, 8
	mov r9, qword [rbp-48]
	mov rax, qword [r9+0]
	mov qword [rbp-40], rax
	; ''
	; '        while (!(*curr.last)) {'
	; [inline asm]
	.4.2:
	; [inline asm]
	mov rax, qword [rbp-40]
	add rax, 8
	mov qword [rbp-56], rax
	; [dyn from copy] -56 , -64, 8
	mov r9, qword [rbp-56]
	mov rax, qword [r9+0]
	mov qword [rbp-64], rax
	; [inline asm]
	; [inline asm]
	mov rax, [rbp-64]
	cmp rax, 0
	setz al
	mov qword [rbp-72], rax
	; [inline asm]
	mov rax, [rbp-72]
	cmp rax, 0
	jnz .4.3
	; '            let prev: $Node = curr;'
	; [dyn to copy] -40 , -80, 8
	mov rax, qword [rbp-40]
	mov qword [rbp-80], rax
	; '            curr = *curr.next;'
	; [inline asm]
	mov rax, qword [rbp-40]
	add rax, 16
	mov qword [rbp-88], rax
	; [dyn from copy] -88 , -40, 8
	mov r9, qword [rbp-88]
	mov rax, qword [r9+0]
	mov qword [rbp-40], rax
	; '            ¬prev;'
	; [heap dealloc] -80 , -96
	call GetProcessHeap
	mov rcx, rax
	mov rdx, 0
	mov r8, qword [rbp-80]
	call HeapFree
	cmp rax, 0
	mov rcx, 0
	setz cl
	mov qword [rbp-96], rcx
	; [inline asm]
	jmp .4.2
	.4.3:
	; '            let prev: $Node = curr;'
	; '            curr = *curr.next;'
	; '            ¬prev;'
	; '        };'
	; ''
	; '        ¬curr;'
	; [heap dealloc] -40 , -104
	call GetProcessHeap
	mov rcx, rax
	mov rdx, 0
	mov r8, qword [rbp-40]
	call HeapFree
	cmp rax, 0
	mov rcx, 0
	setz cl
	mov qword [rbp-104], rcx
	; ''
	; '        *self.has_first = false;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-112], rax
	; [dyn from copy] -112 , -120, 8
	mov r9, qword [rbp-112]
	mov rax, qword [r9+0]
	mov qword [rbp-120], rax
	; [inline asm]
	mov qword [rbp-120], 1
	; [dyn to copy] -120 , -112, 8
	mov r9, qword [rbp-112]
	mov rax, qword [rbp-120]
	mov qword [r9+0], rax
	; '        *self.base = 0;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-128], rax
	; [dyn from copy] -128 , -136, 8
	mov r9, qword [rbp-128]
	mov rax, qword [r9+0]
	mov qword [rbp-136], rax
	; [inline asm]
	mov rax, qword 0
	mov qword [rbp-136], rax
	; [dyn to copy] -136 , -128, 8
	mov r9, qword [rbp-128]
	mov rax, qword [rbp-136]
	mov qword [r9+0], rax
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
	mov rax, [rbp-16]
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '    test();'
	; [no return call] 5 , []
	call _5
	; ''
	; '    return 7;'
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-8], rax
	; [return] Some(-8)
	mov rcx, [rbp-8]
	call ExitProcess
