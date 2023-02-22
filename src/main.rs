#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrystalOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CrystalOS::{println, print, println_log, print_log};
use CrystalOS::kernel::tasks::{Task, executor::Executor, keyboard};
use bootloader::{BootInfo, entry_point};
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string, string::String};
use CrystalOS::shell;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	println!("{}", _info);
	CrystalOS::hlt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	CrystalOS::test_panic_handler(info)
}

entry_point!(main);


fn main(boot_info: &'static BootInfo) -> ! {
	use CrystalOS::kernel::allocator;
	use CrystalOS::kernel::memory;
	use CrystalOS::kernel::memory::BootInfoFrameAllocator;
	use x86_64::{structures::paging::{Page, Translate}, VirtAddr};
	
	CrystalOS::init();
	
	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(physical_memory_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};

	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");

	let mut executor = Executor::new();

	executor.spawn(Task::new(shell::command_handler()));

	executor.run();
	
	#[cfg(test)]
	test_main();

	loop {}
}


