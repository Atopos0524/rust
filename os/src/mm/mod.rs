mod heap_allocator;
mod page_table;
mod address;
mod frame_allocator;
mod memory_set;

use address::{VPNRange, StepByOne};
use page_table::{PTEFlags};
use page_table::{PageTable, PTEFlags};

pub use frame_allocator::{FrameTracker, frame_alloc};
pub use page_table::{PageTableEntry, translated_byte_buffer};
pub use memory_set::{MemorySet, KERNEL_SPACE, MapPermission};
pub use memory_set::remap_test;

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}



pub use address::{PhysAddr, VirtAddr, PhysPageNum, VirtPageNum};
pub use frame_allocator::{FrameTracker, frame_alloc};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}

pub use page_table::{PageTableEntry};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}
