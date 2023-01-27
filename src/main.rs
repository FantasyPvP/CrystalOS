#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrystalOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CrystalOS::{println, print, println_log, print_log};
use CrystalOS::tasks::{Task, executor::Executor, keyboard};
use bootloader::{BootInfo, entry_point};
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string, string::String};
use CrystalOS::vga_buffer;
use CrystalOS::applications::shell;

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
	use CrystalOS::allocator;
	use CrystalOS::memory;
	use CrystalOS::memory::BootInfoFrameAllocator;
	use x86_64::{structures::paging::{Page, Translate}, VirtAddr};

	println_log!("    [Starting CrystalOS]\n\n");
	print_log!("CrystalOS::init...   ");
	CrystalOS::init();
	println_log!("[OK]");
 
	print_log!("CrystalOS::memory::init...   ");
	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(physical_memory_offset) };
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};
	println_log!("[OK]");
	
	print_log!("CrystalOS::allocator::init...   ");
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
	println_log!("[OK]");
	
	print_log!("CrystalOS::tasks::executor...   ");
	let mut executor = Executor::new();
	println_log!("[OK]");

	print_log!("CrystalOS::applications::shell::command_handler...   ");
	executor.spawn(Task::new(shell::command_handler()));
	println_log!("[OK]");
	
	println_log!("Welcome To CrystalOS!");
	executor.run();
	
	#[cfg(test)]
	test_main();
}


async fn add(x: u32, y: u32) -> u32 {
	x + y
}

async fn get_addition(x: u32, y: u32) {
	print!("performing calculation, {} + {} ...    ", x, y);
	let z = add(x, y).await;
	println!("[{}]", z);
}

