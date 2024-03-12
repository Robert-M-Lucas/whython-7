    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern GetProcessHeap
    section .text

__4: ; printi
	push rbp
	mov rbp, rsp
	sub rsp, 80
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
	sub rsp, 128
	; '    let ll: LL = LL#new();'
	call _1
	mov qword [rbp-16], rax
	; ''
	; '    ll.add(12);'
	mov rax, rbp
	add rax, -16
	mov qword [rbp-24], rax
	mov rax, qword 12
	mov qword [rbp-32], rax
	mov rax, qword [rbp-32]
	push rax
	mov rax, qword [rbp-24]
	push rax
	call _2
	add rsp, 16
	; ''
	; '    ll.add(18);'
	mov rax, rbp
	add rax, -16
	mov qword [rbp-40], rax
	mov rax, qword 18
	mov qword [rbp-48], rax
	mov rax, qword [rbp-48]
	push rax
	mov rax, qword [rbp-40]
	push rax
	call _2
	add rsp, 16
	; ''
	; '    let i: int = 0;'
	mov rax, qword 0
	mov qword [rbp-56], rax
	; ''
	; '    while (i < 5) {'
	main.0:
	mov rax, qword 5
	mov qword [rbp-64], rax
	mov rax, [rbp-56]
	mov rcx, [rbp-64]
	cmp rcx, rax
	mov qword [rbp-72], 0
	setle [rbp-72]
	mov rax, [rbp-72]
	cmp rax, 0
	jnz main.1
	; '        ll.add(i * 7);'
	mov rax, rbp
	add rax, -16
	mov qword [rbp-80], rax
	mov rax, qword 7
	mov qword [rbp-88], rax
	mov rax, [rbp-56]
	mov rcx, [rbp-88]
	mul rcx
	mov [rbp-96], rax
	mov rax, qword [rbp-96]
	push rax
	mov rax, qword [rbp-80]
	push rax
	call _2
	add rsp, 16
	; '        i += 1;'
	mov rax, qword 1
	mov qword [rbp-104], rax
	mov rax, [rbp-56]
	add rax, [rbp-104]
	mov [rbp-56], rax
	jmp main.0
	main.1:
	; '        ll.add(i * 7);'
	; '        i += 1;'
	; '    };'
	; ''
	; '    ll.print();'
	mov rax, rbp
	add rax, -16
	mov qword [rbp-112], rax
	push qword 0
	mov rax, qword [rbp-112]
	push rax
	call _3
	add rsp, 16
	; ''
	; '    return 7;'
	mov rax, qword 7
	mov qword [rbp-120], rax
	mov rcx, [rbp-120]
	call ExitProcess

_1: ; new
	push rbp
	mov rbp, rsp
	sub rsp, 16
	; '        return @LL {'
	mov rax, qword 0
	mov qword [rbp-16], rax
	mov qword [rbp-8], 1
	mov rax, [rbp-16]
	leave
	ret

_3: ; print
	push rbp
	mov rbp, rsp
	sub rsp, 112
	; '        if (!(*self.has_first)) {'
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-8], rax
	mov r9, qword [rbp-8]
	mov rax, qword [r9+0]
	mov qword [rbp-16], rax
	mov rax, [rbp-16]
	cmp rax, 0
	setz al
	mov qword [rbp-24], rax
	mov rax, [rbp-24]
	cmp rax, 0
	jnz .3.0
	; '            return;'
	leave
	ret
	.3.0:
	.3.1:
	; '            return;'
	; '        };'
	; ''
	; '        let curr: $Node = *self.base;'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-40], rax
	mov r9, qword [rbp-40]
	mov rax, qword [r9+0]
	mov qword [rbp-32], rax
	; '        printi(*curr.cur);'
	mov rax, qword [rbp-32]
	add rax, 0
	mov qword [rbp-48], rax
	mov r9, qword [rbp-48]
	mov rax, qword [r9+0]
	mov qword [rbp-56], rax
	push qword 0
	mov rax, qword [rbp-56]
	push rax
	call __4
	add rsp, 16
	; ''
	; '        while (!(*curr.last)) {'
	.3.2:
	mov rax, qword [rbp-32]
	add rax, 8
	mov qword [rbp-64], rax
	mov r9, qword [rbp-64]
	mov rax, qword [r9+0]
	mov qword [rbp-72], rax
	mov rax, [rbp-72]
	cmp rax, 0
	setz al
	mov qword [rbp-80], rax
	mov rax, [rbp-80]
	cmp rax, 0
	jnz .3.3
	; '            curr = *curr.next;'
	mov rax, qword [rbp-32]
	add rax, 16
	mov qword [rbp-88], rax
	mov r9, qword [rbp-88]
	mov rax, qword [r9+0]
	mov qword [rbp-32], rax
	; '            printi(*curr.cur);'
	mov rax, qword [rbp-32]
	add rax, 0
	mov qword [rbp-96], rax
	mov r9, qword [rbp-96]
	mov rax, qword [r9+0]
	mov qword [rbp-104], rax
	push qword 0
	mov rax, qword [rbp-104]
	push rax
	call __4
	add rsp, 16
	jmp .3.2
	.3.3:
	leave
	ret

_2: ; add
	push rbp
	mov rbp, rsp
	sub rsp, 160
	; '        let new_node: Node = @Node {'
	mov rax, qword [rbp+24]
	mov qword [rbp-24], rax
	mov qword [rbp-16], 0
	mov rax, qword 0
	mov qword [rbp-8], rax
	; '            next,'
	; '            true,'
	; '            0'
	; '        };'
	; ''
	; '        if (!(*self.has_first)) {'
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-32], rax
	mov r9, qword [rbp-32]
	mov rax, qword [r9+0]
	mov qword [rbp-40], rax
	mov rax, [rbp-40]
	cmp rax, 0
	setz al
	mov qword [rbp-48], rax
	mov rax, [rbp-48]
	cmp rax, 0
	jnz .2.0
	; '            *self.base = ^new_node;'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-56], rax
	mov r9, qword [rbp-56]
	mov rax, qword [r9+0]
	mov qword [rbp-64], rax
	call GetProcessHeap
	mov rcx, rax
	mov rdx, rax
	mov r8, 24
	call HeapAlloc
	mov qword [rbp-64], rax
	mov r9, qword [rbp-64]
	mov rax, qword [rbp-24]
	mov qword [r9+0], rax
	mov rax, qword [rbp-16]
	mov qword [r9+8], rax
	mov rax, qword [rbp-8]
	mov qword [r9+16], rax
	mov r9, qword [rbp-56]
	mov rax, qword [rbp-64]
	mov qword [r9+0], rax
	; '            *self.has_first = true;'
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-72], rax
	mov r9, qword [rbp-72]
	mov rax, qword [r9+0]
	mov qword [rbp-80], rax
	mov qword [rbp-80], 0
	mov r9, qword [rbp-72]
	mov rax, qword [rbp-80]
	mov qword [r9+0], rax
	; '            return;'
	leave
	ret
	.2.0:
	.2.1:
	; '            *self.base = ^new_node;'
	; '            *self.has_first = true;'
	; '            return;'
	; '        };'
	; ''
	; '        let curr: $Node = *self.base;'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-96], rax
	mov r9, qword [rbp-96]
	mov rax, qword [r9+0]
	mov qword [rbp-88], rax
	; '        while (!(*curr.last)) {'
	.2.2:
	mov rax, qword [rbp-88]
	add rax, 8
	mov qword [rbp-104], rax
	mov r9, qword [rbp-104]
	mov rax, qword [r9+0]
	mov qword [rbp-112], rax
	mov rax, [rbp-112]
	cmp rax, 0
	setz al
	mov qword [rbp-120], rax
	mov rax, [rbp-120]
	cmp rax, 0
	jnz .2.3
	; '            curr = *curr.next;'
	mov rax, qword [rbp-88]
	add rax, 16
	mov qword [rbp-128], rax
	mov r9, qword [rbp-128]
	mov rax, qword [r9+0]
	mov qword [rbp-88], rax
	jmp .2.2
	.2.3:
	; '            curr = *curr.next;'
	; '        };'
	; ''
	; '        *curr.last = false;'
	mov rax, qword [rbp-88]
	add rax, 8
	mov qword [rbp-136], rax
	mov r9, qword [rbp-136]
	mov rax, qword [r9+0]
	mov qword [rbp-144], rax
	mov qword [rbp-144], 1
	mov r9, qword [rbp-136]
	mov rax, qword [rbp-144]
	mov qword [r9+0], rax
	; '        *curr.next = ^new_node;'
	mov rax, qword [rbp-88]
	add rax, 16
	mov qword [rbp-152], rax
	mov r9, qword [rbp-152]
	mov rax, qword [r9+0]
	mov qword [rbp-160], rax
	call GetProcessHeap
	mov rcx, rax
	mov rdx, rax
	mov r8, 24
	call HeapAlloc
	mov qword [rbp-160], rax
	mov r9, qword [rbp-160]
	mov rax, qword [rbp-24]
	mov qword [r9+0], rax
	mov rax, qword [rbp-16]
	mov qword [r9+8], rax
	mov rax, qword [rbp-8]
	mov qword [r9+16], rax
	mov r9, qword [rbp-152]
	mov rax, qword [rbp-160]
	mov qword [r9+0], rax
	leave
	ret
