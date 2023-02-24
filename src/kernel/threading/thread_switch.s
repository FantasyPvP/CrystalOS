

asm_thread_switch:
	pushfq

	mov rax, rsp
	mov rsp, rdi
	
	mov rdi, rax
	call add_paused_thread

	popfq
	ret
