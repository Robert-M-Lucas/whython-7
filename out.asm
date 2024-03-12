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
	; '    let test: Test = @Test {'
	mov rax, qword 2
	mov qword [rbp-16], rax
	mov rax, qword 7
	mov qword [rbp-8], rax
	; '        2,'
	; '        7'
	; '    };'
	; ''
	; '    let lest: $Test = &test;'
	mov rax, rbp
	add rax, -16
	mov qword [rbp-24], rax
	; '    let b: $int = lest.b;'
	mov rax, qword [rbp-24]
	add rax, 8
	mov qword [rbp-40], rax
	mov rax, qword [rbp-40]
	mov qword [rbp-32], rax
	; ''
	; '    printi(*b);'
	mov r9, qword [rbp-32]
	mov rax, qword [r9+0]
	mov qword [rbp-48], rax
	push qword 0
	mov rax, qword [rbp-48]
	push rax
	call __4
	add rsp, 16
	; ''
	; '    test.b += 1;'
	mov rax, qword 1
	mov qword [rbp-56], rax
	mov rax, [rbp-8]
	add rax, [rbp-56]
	mov [rbp-8], rax
	; ''
	; '    printi(*b);'
	mov r9, qword [rbp-32]
	mov rax, qword [r9+0]
	mov qword [rbp-64], rax
	push qword 0
	mov rax, qword [rbp-64]
	push rax
	call __4
	add rsp, 16
	; ''
	; '    return 0;'
	mov rax, qword 0
	mov qword [rbp-72], rax
	mov rcx, [rbp-72]
	call ExitProcess
