
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use CrystalOS::{exit, QemuExitCode, serial_println, serial_print};


#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_print!("stack_overflow::stack_overflow...\t");

	CrystalOS::kernel::gdt::init();
	init_test_idt();

	stack_overflow();

	panic!("stack overflow did not stop execution!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	CrystalOS::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
	stack_overflow();
	volatile::Volatile::new(0).read();
}

lazy_static! {
	static ref TEST_IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		unsafe {
			idt.double_fault.set_handler_fn(test_double_fault_handler).set_stack_index(CrystalOS::kernel::gdt::DOUBLE_FAULT_IST_INDEX);
		}
		idt
	};
}

extern "x86-interrupt" fn test_double_fault_handler(
	_stack_frame: InterruptStackFrame,
	_error_code: u64,
) -> ! {
	serial_println!("OK");
	exit(QemuExitCode::Ok);
	loop {}
}


pub fn init_test_idt() {
	TEST_IDT.load();
}
