#![no_main]
#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
use alloc::{format, alloc::{GlobalAlloc, Layout}};
use core::panic::PanicInfo;

/// Just to have an allocator so that we can compile the code.
/// It doesn't do anything correct but only gets the code compile.
struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        return 0x2001000 as *mut u8;
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static DUMMY_ALLOCATOR: DummyAllocator = DummyAllocator;

#[start]
#[export_name = "ENTRY_FUNC"]
pub extern "C" fn main() {
    // Here the problem occurs. The code in `alloc` crate will
    // change R9 register.
    let _s = format!("{}", "boom!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
