
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrystalOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use alloc::{ boxed::Box, vec::Vec };


entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
	use CrystalOS::kernel::allocator;
	use CrystalOS::kernel::memory::{self, BootInfoFrameAllocator};
	use x86_64::VirtAddr;

	CrystalOS::init();

	let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(physical_memory_offset)};
	let mut frame_allocator = unsafe {
		BootInfoFrameAllocator::init(&boot_info.memory_map)
	};
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("failed to initialise heap");

	test_main();

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	CrystalOS::test_panic_handler(info)
}

#[test_case]
fn box_allocation() {
	let heap_1 = Box::new(69);
	let heap_2 = Box::new(420);
	assert_eq!(*heap_1, 69);
	assert_eq!(*heap_2, 420);
}

#[test_case]
fn vec_allocation() {
	let x = 1000;
	let mut vector = Vec::new();
	for i in 0..x {
		vector.push(i);
	}
	assert_eq!(vector.iter().sum::<u64>(), (x-1) * x/2);
	
}

#[test_case]
fn reallocation() {
	use CrystalOS::kernel::allocator::HEAP_SIZE;

	for i in 0..HEAP_SIZE {
		let x = Box::new(i);
		assert_eq!(*x, i);
	}
}
