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
	sub rsp, 48
	; [inline asm]
	sub rsp, 32
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

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 64
	; '    let ll: LL = LL#new();'
	; [return call] 1 , [], -16
	sub rsp, 8
	sub rsp, 8
	call _1
	; [local copy] -80 , -16, 16
	mov rax, qword [rbp-80]
	mov qword [rbp-16], rax
	mov rax, qword [rbp-72]
	mov qword [rbp-8], rax
	add rsp, 16
	; ''
	; '    ll.add(12);'
	; [inline asm]
	mov rax, rbp
	add rax, -16
	mov qword [rbp-24], rax
	; [inline asm]
	mov rax, qword 12
	mov qword [rbp-32], rax
	; [no return call] 2 , [(-24, 8), (-32, 8)]
	sub rsp, 8
	mov rax, qword [rbp-32]
	mov qword [rbp-72], rax
	sub rsp, 8
	mov rax, qword [rbp-24]
	mov qword [rbp-80], rax
	call _2
	add rsp, 16
	; ''
	; '    ll.add(18);'
	; [inline asm]
	mov rax, rbp
	add rax, -16
	mov qword [rbp-40], rax
	; [inline asm]
	mov rax, qword 18
	mov qword [rbp-48], rax
	; [no return call] 2 , [(-40, 8), (-48, 8)]
	sub rsp, 8
	mov rax, qword [rbp-48]
	mov qword [rbp-72], rax
	sub rsp, 8
	mov rax, qword [rbp-40]
	mov qword [rbp-80], rax
	call _2
	add rsp, 16
	; ''
	; '    ll.print();'
	; [inline asm]
	mov rax, rbp
	add rax, -16
	mov qword [rbp-56], rax
	; [no return call] 3 , [(-56, 8)]
	sub rsp, 8
	mov rax, qword [rbp-56]
	mov qword [rbp-72], rax
	call _3
	add rsp, 8
	; ''
	; '    return 7;'
	; [inline asm]
	mov rax, qword 7
	mov qword [rbp-64], rax
	; [return] Some((-64, 8))
	mov rcx, qword [rbp-64]
	call ExitProcess

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

_3: ; print
	push rbp
	mov rbp, rsp
	sub rsp, 112
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
	; '        printi(*curr.cur);'
	; [inline asm]
	mov rax, qword [rbp-32]
	add rax, 0
	mov qword [rbp-48], rax
	; [dyn from copy] -48 , -56, 8
	mov r9, qword [rbp-48]
	mov rax, qword [r9+0]
	mov qword [rbp-56], rax
	; [no return call] -4 , [(-56, 8)]
	sub rsp, 8
	mov rax, qword [rbp-56]
	mov qword [rbp-120], rax
	call __4
	add rsp, 8
	; '        while (!(*curr.last)) {'
	; [inline asm]
	.3.2:
	; [inline asm]
	mov rax, qword [rbp-32]
	add rax, 8
	mov qword [rbp-64], rax
	; [dyn from copy] -64 , -72, 8
	mov r9, qword [rbp-64]
	mov rax, qword [r9+0]
	mov qword [rbp-72], rax
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-72]
	cmp rax, 0
	setz al
	mov qword [rbp-80], rax
	; [inline asm]
	mov rax, qword [rbp-80]
	cmp rax, 0
	jnz .3.3
	; '            curr = *curr.next;'
	; [inline asm]
	mov rax, qword [rbp-32]
	add rax, 16
	mov qword [rbp-88], rax
	; [dyn from copy] -88 , -32, 8
	mov r9, qword [rbp-88]
	mov rax, qword [r9+0]
	mov qword [rbp-32], rax
	; '            printi(*curr.cur);'
	; [inline asm]
	mov rax, qword [rbp-32]
	add rax, 0
	mov qword [rbp-96], rax
	; [dyn from copy] -96 , -104, 8
	mov r9, qword [rbp-96]
	mov rax, qword [r9+0]
	mov qword [rbp-104], rax
	; [no return call] -4 , [(-104, 8)]
	sub rsp, 8
	mov rax, qword [rbp-104]
	mov qword [rbp-120], rax
	call __4
	add rsp, 8
	; [inline asm]
	jmp .3.2
	.3.3:
	leave
	ret

_2: ; add
	push rbp
	mov rbp, rsp
	sub rsp, 160
	; '        let new_node: Node = @Node {'
	; [local copy] 24 , -24, 8
	mov rax, qword [rbp+24]
	mov qword [rbp-24], rax
	; [inline asm]
	mov qword [rbp-16], 0
	; [inline asm]
	mov rax, qword 0
	mov qword [rbp-8], rax
	; [inline asm]
	; '            next,'
	; '            true,'
	; '            0'
	; '        };'
	; ''
	; '        if (!(*self.has_first)) {'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-32], rax
	; [dyn from copy] -32 , -40, 8
	mov r9, qword [rbp-32]
	mov rax, qword [r9+0]
	mov qword [rbp-40], rax
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-40]
	cmp rax, 0
	setz al
	mov qword [rbp-48], rax
	; [inline asm]
	mov rax, qword [rbp-48]
	cmp rax, 0
	jnz .2.0
	; '            *self.base = ^new_node;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-56], rax
	; [dyn from copy] -56 , -64, 8
	mov r9, qword [rbp-56]
	mov rax, qword [r9+0]
	mov qword [rbp-64], rax
	; [heap alloc] 24 , -64
	sub rsp, 32
	call GetProcessHeap
	mov rcx, rax
	mov rdx, 0
	mov r8, 24
	call HeapAlloc
	mov qword [rbp-64], rax
	add rsp, 32
	; [dyn to copy] -24 , -64, 24
	mov r9, qword [rbp-64]
	mov rax, qword [rbp-24]
	mov qword [r9+0], rax
	mov rax, qword [rbp-16]
	mov qword [r9+8], rax
	mov rax, qword [rbp-8]
	mov qword [r9+16], rax
	; [dyn to copy] -64 , -56, 8
	mov r9, qword [rbp-56]
	mov rax, qword [rbp-64]
	mov qword [r9+0], rax
	; '            *self.has_first = true;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-72], rax
	; [dyn from copy] -72 , -80, 8
	mov r9, qword [rbp-72]
	mov rax, qword [r9+0]
	mov qword [rbp-80], rax
	; [inline asm]
	mov qword [rbp-80], 0
	; [dyn to copy] -80 , -72, 8
	mov r9, qword [rbp-72]
	mov rax, qword [rbp-80]
	mov qword [r9+0], rax
	; '            return;'
	; [return] None
	leave
	ret
	; [inline asm]
	.2.0:
	.2.1:
	; '            *self.base = ^new_node;'
	; '            *self.has_first = true;'
	; '            return;'
	; '        };'
	; ''
	; '        let curr: $Node = *self.base;'
	; [inline asm]
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-96], rax
	; [dyn from copy] -96 , -88, 8
	mov r9, qword [rbp-96]
	mov rax, qword [r9+0]
	mov qword [rbp-88], rax
	; '        while (!(*curr.last)) {'
	; [inline asm]
	.2.2:
	; [inline asm]
	mov rax, qword [rbp-88]
	add rax, 8
	mov qword [rbp-104], rax
	; [dyn from copy] -104 , -112, 8
	mov r9, qword [rbp-104]
	mov rax, qword [r9+0]
	mov qword [rbp-112], rax
	; [inline asm]
	; [inline asm]
	mov rax, qword [rbp-112]
	cmp rax, 0
	setz al
	mov qword [rbp-120], rax
	; [inline asm]
	mov rax, qword [rbp-120]
	cmp rax, 0
	jnz .2.3
	; '            curr = *curr.next;'
	; [inline asm]
	mov rax, qword [rbp-88]
	add rax, 16
	mov qword [rbp-128], rax
	; [dyn from copy] -128 , -88, 8
	mov r9, qword [rbp-128]
	mov rax, qword [r9+0]
	mov qword [rbp-88], rax
	; [inline asm]
	jmp .2.2
	.2.3:
	; '            curr = *curr.next;'
	; '        };'
	; ''
	; '        *curr.last = false;'
	; [inline asm]
	mov rax, qword [rbp-88]
	add rax, 8
	mov qword [rbp-136], rax
	; [dyn from copy] -136 , -144, 8
	mov r9, qword [rbp-136]
	mov rax, qword [r9+0]
	mov qword [rbp-144], rax
	; [inline asm]
	mov qword [rbp-144], 1
	; [dyn to copy] -144 , -136, 8
	mov r9, qword [rbp-136]
	mov rax, qword [rbp-144]
	mov qword [r9+0], rax
	; '        *curr.next = ^new_node;'
	; [inline asm]
	mov rax, qword [rbp-88]
	add rax, 16
	mov qword [rbp-152], rax
	; [dyn from copy] -152 , -160, 8
	mov r9, qword [rbp-152]
	mov rax, qword [r9+0]
	mov qword [rbp-160], rax
	; [heap alloc] 24 , -160
	sub rsp, 32
	call GetProcessHeap
	mov rcx, rax
	mov rdx, 0
	mov r8, 24
	call HeapAlloc
	mov qword [rbp-160], rax
	add rsp, 32
	; [dyn to copy] -24 , -160, 24
	mov r9, qword [rbp-160]
	mov rax, qword [rbp-24]
	mov qword [r9+0], rax
	mov rax, qword [rbp-16]
	mov qword [r9+8], rax
	mov rax, qword [rbp-8]
	mov qword [r9+16], rax
	; [dyn to copy] -160 , -152, 8
	mov r9, qword [rbp-152]
	mov rax, qword [rbp-160]
	mov qword [r9+0], rax
	leave
	ret
