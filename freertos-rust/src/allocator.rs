use core::alloc::{Layout, GlobalAlloc};
use crate::shim::*;
use crate::base::*;

/**
Use with:

    #[global_allocator]
    static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;
*/

pub struct FreeRtosAllocator;

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = freertos_rs_pvPortMalloc(layout.size() as u32);
        return res as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        freertos_rs_vPortFree(ptr as FreeRtosVoidPtr)
    }
}

