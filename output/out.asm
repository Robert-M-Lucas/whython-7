    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern WriteConsoleA
    extern WriteConsoleW
    section .text
._1:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov byte [rbp-4], 4Eh
	sub     rsp, 56
	mov     qword [rsp + 40], 0eh 
	mov     ecx, -11
	call    GetStdHandle
	mov     rcx, rax
	mov     rdx, [rbp - 8]
	mov     r8, [rsp + 40]
	mov     r9, 01h
	mov     qword [rsp + 32], 00h 
	call    WriteFile
	add     rsp, 56
	leave
	ret
main:
	push rbp
	mov rbp, rsp
	sub rsp, 8
	mov qword [rbp-8], 2
	mov rax, qword [rbp-8]
	push rax
	call ._1
	add rsp, 8
	mov rcx, 0
	call ExitProcess
