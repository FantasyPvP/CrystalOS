#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrystalOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CrystalOS::{println, print};
use CrystalOS::tasks::{Task, executor::Executor, keyboard};
use bootloader::{BootInfo, entry_point};
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string, string::String};


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

	println!("Starting <CrystalOS> ...");
	CrystalOS::init();

	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(physical_memory_offset) };

	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialisation failed");
	
	let mut executor = Executor::new();
	executor.spawn(Task::new(get_addition(5, 9)));
	executor.spawn(Task::new(keyboard::print_keypresses()));

	
	executor.run();
	
	println!("ayyo");

	#[cfg(test)]
	test_main();
	
	CrystalOS::hlt();
}


async fn add(x: u32, y: u32) -> u32 {
	x + y
}

async fn get_addition(x: u32, y: u32) {
	print!("performing calculation, {} + {} ...    ", x, y);
	let z = add(x, y).await;
	println!("[{}]", z);
}

