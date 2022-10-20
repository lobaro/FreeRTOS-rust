use crate::base::*;
use crate::prelude::v1::*;
use crate::shim::*;

#[derive(Debug, Copy, Clone)]
pub struct TypeSizeError {
    id: usize,
    c_size: usize,
    rust_size: usize,
}

#[cfg(feature = "cpu_clock")]
pub fn cpu_clock_hz() -> u32 {
  unsafe { freertos_rs_get_configCPU_CLOCK_HZ() }
}

/// Perform checks whether the C FreeRTOS shim and Rust agree on the sizes of used types.
pub fn shim_sanity_check() -> Result<(), TypeSizeError> {
    let checks = [
        (0, mem::size_of::<FreeRtosVoidPtr>()),
        (1, mem::size_of::<FreeRtosCharPtr>()),
        (2, mem::size_of::<FreeRtosChar>()),
        (10, mem::size_of::<FreeRtosBaseType>()),
        (11, mem::size_of::<FreeRtosUBaseType>()),
        (12, mem::size_of::<FreeRtosTickType>()),
        (20, mem::size_of::<FreeRtosTaskHandle>()),
        (21, mem::size_of::<FreeRtosQueueHandle>()),
        (22, mem::size_of::<FreeRtosSemaphoreHandle>()),
        (23, mem::size_of::<FreeRtosTaskFunction>()),
        (24, mem::size_of::<FreeRtosTimerHandle>()),
        (25, mem::size_of::<FreeRtosTimerCallback>()),
        (30, mem::size_of::<FreeRtosTaskStatusFfi>()),
        (31, mem::size_of::<FreeRtosTaskState>()),
        (32, mem::size_of::<FreeRtosUnsignedLong>()),
        (33, mem::size_of::<FreeRtosUnsignedShort>()),
    ];

    for check in &checks {
        let c_size = unsafe { freertos_rs_sizeof(check.0) };

        if c_size != check.1 as u8 {
            return Err(TypeSizeError {
                id: check.0 as usize,
                c_size: c_size as usize,
                rust_size: check.1,
            });
        }
    }

    Ok(())
}

#[cfg(any(feature = "time", feature = "hooks", feature = "sync"))]
pub unsafe fn str_from_c_string(str: *const u8) -> Result<String, FreeRtosError> {
    let mut buf = Vec::new();

    let mut p = str;
    loop {
        if *p == 0 {
            break;
        }
        buf.push(*p);
        p = p.offset(1);
    }

    match String::from_utf8(buf) {
        Ok(s) => Ok(s),
        Err(_) => Err(FreeRtosError::StringConversionError),
    }
}
