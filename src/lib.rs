
#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(async_fn_in_trait)]
#![feature(global_asm)]

use core::panic::PanicInfo;
use spin::Mutex;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod tasks;
pub mod applications;
pub mod os;
pub mod threading;
extern crate alloc;
//extern crate fatfs;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};


#[cfg(test)]
entry_point!(test_kernel_main);

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
	panic!("error while allocating: {:?}", layout)
}



pub fn init() {
	gdt::init();
	interrupts::init_idt();
	unsafe { interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}


pub fn hlt() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}

pub trait Testable {
	fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {
	fn run(&self) {
		serial_print!("{}...\t", core::any::type_name::<T>());
		self();
		serial_println!("OK");
	}
}

pub fn test_runner(tests: &[&dyn Testable]) {
	serial_println!("Running {} tests", tests.len());
	for test in tests {
		test.run();
	}
	exit(QemuExitCode::Ok);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
	serial_println!("ERR");
	serial_println!("Error: {}\n", info);
	exit(QemuExitCode::Err);
	hlt(); 
}

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
	init();
	test_main();
	hlt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
	Ok = 0x10,
	Err = 0x11,
}

pub fn exit(code: QemuExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(code as u32);
	}
}
