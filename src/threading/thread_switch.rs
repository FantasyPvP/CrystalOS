use core::arch::global_asm;

global_asm!(include_str!("thread_switch.s"));
