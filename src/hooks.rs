use crate::base::*;
use crate::utils::*;
use crate::prelude::v1::String;

type Callback = fn();

pub struct FreeRtosHooks {
    on_assert: Callback,
}

impl FreeRtosHooks {
    pub fn set_on_assert(&mut self, c: Callback) {
        self.on_assert = c;
    }

    fn do_on_assert(&self) {
        (self.on_assert)();
    }
}

// TODO: It's unsafe to use, we should build some safe wrapper around
pub static mut FREERTOS_HOOKS: FreeRtosHooks = FreeRtosHooks { on_assert: || {} };

#[allow(unused_doc_comments)]
#[no_mangle]
pub extern "C" fn freerots_rs_assert_called(file_name_ptr: FreeRtosCharPtr, line: FreeRtosUBaseType) {
    let file_name: String;
    unsafe {
        file_name = str_from_c_string(file_name_ptr).unwrap();
    }

    unsafe { FREERTOS_HOOKS.do_on_assert(); }

    // we can't print without std yet.
    // TODO: make the macro work for debug UART? Or use Panic here?
    // println!("ASSERT: {} {}", line, file_name);
    panic!("FreeRTOS ASSERT: {}:{}", file_name, line);
    //loop {}
}