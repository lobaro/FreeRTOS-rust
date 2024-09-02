use crate::base::{freertos_rs_pvPortMalloc, freertos_rs_vPortFree, FreeRtosVoidPtr};
use core::alloc::{GlobalAlloc, Layout};

/**
Use with:

    #[global_allocator]
    static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;
*/

pub struct FreeRtosAllocator;

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = freertos_rs_pvPortMalloc(layout.size() as u32);
        res as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        freertos_rs_vPortFree(ptr as FreeRtosVoidPtr)
    }
}
