    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
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
	sub rsp, 80
	; '    let i: int = 1;'
	mov rax, qword 1
	mov qword [rbp-8], rax
	; ''
	; '    while (i < 10) {'
	main.0:
	mov rax, qword 10
	mov qword [rbp-16], rax
	mov rax, [rbp-8]
	mov rcx, [rbp-16]
	cmp rcx, rax
	mov qword [rbp-24], 0
	setle [rbp-24]
	mov rax, [rbp-24]
	cmp rax, 0
	jnz main.1
	; '        let f: Fib = @ Fib { 0, 1, i };'
	mov rax, qword 0
	mov qword [rbp-48], rax
	mov rax, qword 1
	mov qword [rbp-40], rax
	mov rax, qword [rbp-8]
	mov qword [rbp-32], rax
	; '        f.start();'
	mov rax, rbp
	add rax, -48
	mov qword [rbp-56], rax
	push qword 0
	mov rax, qword [rbp-56]
	push rax
	call _1
	add rsp, 16
	; '        i += 1;'
	mov rax, qword 1
	mov qword [rbp-64], rax
	mov rax, [rbp-8]
	add rax, [rbp-64]
	mov [rbp-8], rax
	jmp main.0
	main.1:
	; '        let f: Fib = @ Fib { 0, 1, i };'
	; '        f.start();'
	; '        i += 1;'
	; '    };'
	; ''
	; '    return 0;'
	mov rax, qword 0
	mov qword [rbp-72], rax
	mov rcx, [rbp-72]
	call ExitProcess

_1: ; start
	push rbp
	mov rbp, rsp
	sub rsp, 160
	; '        if ((*self.n) == 0) {'
	; '            printi(*self.a);'
	; '        }'
	; '        else {'
	mov rax, qword [rbp+16]
	add rax, 16
	mov qword [rbp-8], rax
	mov r9, qword [rbp-8]
	mov rax, qword [r9+0]
	mov qword [rbp-16], rax
	mov rax, qword 0
	mov qword [rbp-24], rax
	mov rax, [rbp-16]
	mov rcx, [rbp-24]
	cmp rcx, rax
	mov qword [rbp-32], 0
	setnz [rbp-32]
	mov rax, [rbp-32]
	cmp rax, 0
	jnz .1.0
	; '            printi(*self.a);'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-40], rax
	mov r9, qword [rbp-40]
	mov rax, qword [r9+0]
	mov qword [rbp-48], rax
	push qword 0
	mov rax, qword [rbp-48]
	push rax
	call __4
	add rsp, 16
	jmp .1.1
	.1.0:
	; '            let temp: int = (*self.a) + (*self.b);'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-64], rax
	mov r9, qword [rbp-64]
	mov rax, qword [r9+0]
	mov qword [rbp-72], rax
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-80], rax
	mov r9, qword [rbp-80]
	mov rax, qword [r9+0]
	mov qword [rbp-88], rax
	mov rax, [rbp-72]
	add rax, [rbp-88]
	mov [rbp-56], rax
	; '            *self.a = *self.b;'
	mov rax, qword [rbp+16]
	add rax, 0
	mov qword [rbp-96], rax
	mov r9, qword [rbp-96]
	mov rax, qword [r9+0]
	mov qword [rbp-104], rax
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-112], rax
	mov r9, qword [rbp-112]
	mov rax, qword [r9+0]
	mov qword [rbp-104], rax
	mov r9, qword [rbp-96]
	mov rax, qword [rbp-104]
	mov qword [r9+0], rax
	; '            *self.b = temp;'
	mov rax, qword [rbp+16]
	add rax, 8
	mov qword [rbp-120], rax
	mov r9, qword [rbp-120]
	mov rax, qword [r9+0]
	mov qword [rbp-128], rax
	mov rax, qword [rbp-56]
	mov qword [rbp-128], rax
	mov r9, qword [rbp-120]
	mov rax, qword [rbp-128]
	mov qword [r9+0], rax
	; '            *self.n -= 1;'
	mov rax, qword [rbp+16]
	add rax, 16
	mov qword [rbp-136], rax
	mov r9, qword [rbp-136]
	mov rax, qword [r9+0]
	mov qword [rbp-144], rax
	mov rax, qword 1
	mov qword [rbp-152], rax
	mov rax, [rbp-144]
	sub rax, [rbp-152]
	mov [rbp-144], rax
	mov r9, qword [rbp-136]
	mov rax, qword [rbp-144]
	mov qword [r9+0], rax
	; '            self.start();'
	push qword 0
	mov rax, qword [rbp+16]
	push rax
	call _1
	add rsp, 16
	.1.2:
	.1.1:
	leave
	ret
