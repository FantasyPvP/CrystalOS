
use x86_64::{
	structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator, PageTable},
	VirtAddr, 
	PhysAddr
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::OffsetPageTable;

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {

	use x86_64::registers::control::Cr3;

	let (level_4_table_frame, _) = Cr3::read();
	let phys = level_4_table_frame.start_address();
	let virt = physical_memory_offset + phys.as_u64();
	let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

	&mut *page_table_ptr
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
	let level_4_table = active_level_4_table(physical_memory_offset);
	OffsetPageTable::new(level_4_table, physical_memory_offset)
}


pub struct BootInfoFrameAllocator {
	memory_map: &'static MemoryMap,
	next: usize,
}

impl BootInfoFrameAllocator {
	pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
		BootInfoFrameAllocator {
			memory_map,
			next: 0,
		}
	}

	fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
		let regions = self.memory_map.iter();
		let usable_regions = regions.filter(|r| 
			r.region_type == MemoryRegionType::Usable
		);

		let address_ranges = usable_regions.map(|r|
			r.range.start_addr()..r.range.end_addr()
		);
	
		let frame_addresses = address_ranges.flat_map(|r| r.step_by(4096));

		frame_addresses.map(|a| PhysFrame::containing_address(PhysAddr::new(a)))
	}
	
}


pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
	fn allocate_frame(&mut self) -> Option<PhysFrame> {
		None
	}
 }

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
	fn allocate_frame(&mut self) -> Option<PhysFrame> {
		let frame = self.usable_frames().nth(self.next);
		self.next += 1;
		frame
	}
}

/*

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackBounds {
	start: VirtAddr,
	end:   VirtAddr,
}

impl StackBounds {
	pub fn start(&self) -> VirtAddr {
		self.start
	}

	pub fn end(&self) -> VirtAddr {
		self.end
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(u64);

impl ThreadId {
	pub fn as_u64(&self) -> u64 {
		self.0
	}

	fn new() -> Self {
		use core::sync::atomic::{AtomicU64, Ordering};
		static NEXT_THREAD_ID: AtomicU64 = AtomicU64::new(1);
		ThreadId(NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed))
	}
}

fn reserve_stack_memory(size_in_pages: u64) -> Page {
	use core::sync::atomic::{AtomicU64, Ordering};

	static STACK_ALLOC_NEXT: AtomicU64  = AtomicU64::new(0x_5555_5555_0000);
	let start_addr = VirtAddr::new(STACK_ALLOC_NEXT.fetch_add(
		size_in_pages * Page::<Size4KiB>::SIZE,
		Ordering::Relaxed,
	));
	Page::from_start_address(start_addr).expect("STACK_ALLOC_NEXT: not page aligned")
}

pub fn alloc_stack(
	size_in_pages: u64, mapper: &mut impl Mapper<Size4KiB>,
	frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<StackBounds, mapper::MapToError> {
	use x86_64::structures::paging::PageTableFlags as Flags;

	let guard_page = reserve_stack_memory(size_in_pages + 1);
	let stack_start = guard_page + 1;
	let stack_end = stack_start + size_in_pages;

	for page in Page::range(stack_start, stack_end) {
		let frame = frame_allocator.allocate_frame().ok_or(mapper.MapToError::FrameAllocatorFailed)?;
		let flags = Flags::PRESENT | Flags::WRITABLE;
		mapper.map_to(page, frame, flags, frame_allocator)?.flush();
	}

	Ok(StackBounds {
		start: stack_start.start_address(),
		end: stack_end.start_address(),
	})
}
*/
