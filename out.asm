    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern GetProcessHeap
    section .text

_1: ; add
	push rbp
	mov rbp, rsp
	sub rsp, 128
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
	setz [rax]
	mov [rbp-48], rax
	mov rax, [rbp-48]
	cmp rax, 0
	jnz .1.0
	; '            self.base = ^new_node;'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-56], rax
	call GetProcessHeap
	mov rcx, rax
	mov rdx, rax
	mov r8, 24
	call HeapAlloc
	mov qword [rbp-56], rax
	mov r9, qword [rbp-56]
	mov rax, qword [rbp-24]
	mov qword [r9+0], rax
	mov rax, qword [rbp-16]
	mov qword [r9+8], rax
	mov rax, qword [rbp-8]
	mov qword [r9+16], rax
	; '            return;'
	leave
	ret
	.1.0:
	.1.1:
	; '            self.base = ^new_node;'
	; '            return;'
	; '        };'
	; ''
	; '        let curr: $Node = self.base;'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-72], rax
	mov rax, qword [rbp-72]
	mov qword [rbp-64], rax
	; '        while (!(*curr.last)) {'
	.1.2:
	mov rax, qword [rbp-64]
	add rax, 8
	mov qword [rbp-80], rax
	mov r9, qword [rbp-80]
	mov rax, qword [r9+0]
	mov qword [rbp-88], rax
	mov rax, [rbp-88]
	cmp rax, 0
	setz [rax]
	mov [rbp-96], rax
	mov rax, [rbp-96]
	cmp rax, 0
	jnz .1.3
	; '            curr = curr.next;'
	mov rax, qword [rbp-64]
	add rax, 16
	mov qword [rbp-104], rax
	mov rax, qword [rbp-104]
	mov qword [rbp-64], rax
	jmp .1.2
	.1.3:
	; '            curr = curr.next;'
	; '        };'
	; ''
	; '        *curr.last = false;'
	mov rax, qword [rbp-64]
	add rax, 8
	mov qword [rbp-112], rax
	mov r9, qword [rbp-112]
	mov rax, qword [r9+0]
	mov qword [rbp-120], rax
	mov qword [rbp-120], 1
	mov r9, qword [rbp-112]
	mov rax, qword [rbp-120]
	mov qword [r9+0], rax
	; '        curr.next = ^new_node;'
	mov rax, qword [rbp-64]
	add rax, 16
	mov qword [rbp-128], rax
	call GetProcessHeap
	mov rcx, rax
	mov rdx, rax
	mov r8, 24
	call HeapAlloc
	mov qword [rbp-128], rax
	mov r9, qword [rbp-128]
	mov rax, qword [rbp-24]
	mov qword [r9+0], rax
	mov rax, qword [rbp-16]
	mov qword [r9+8], rax
	mov rax, qword [rbp-8]
	mov qword [r9+16], rax
	leave
	ret

main: ; main
	push rbp
	mov rbp, rsp
	sub rsp, 48
	; '    let ll: LL = @LL {'
	mov rax, qword 0
	mov qword [rbp-16], rax
	mov qword [rbp-8], 1
	; '        0,'
	; '        false'
	; '    };'
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
	call _1
	add rsp, 16
	; ''
	; '    return 7;'
	mov rax, qword 7
	mov qword [rbp-40], rax
	mov rcx, [rbp-40]
	call ExitProcess
