mod address;
pub mod frame_allocator;
pub mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysPageNum, VirtAddr};
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{copy_byte_buffer, translate_byte_buffer};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.lock().activate();
}
