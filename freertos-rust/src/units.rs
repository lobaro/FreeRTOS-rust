//! Expose time units type and implementation utilities.
use crate::base::FreeRtosTickType;
use crate::shim::*;

/// Internal usage: returns the tick period in milliseconds.
#[inline]
fn get_tick_period_ms() -> u32 {
    unsafe { freertos_rs_get_portTICK_PERIOD_MS() }
}

/// Internal usage: returns the maximum wait time in ticks.
fn get_max_wait() -> u32 {
    unsafe { freertos_rs_max_wait() }
}

/// A freeRTOS Duration, internally represented as ticks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    ticks: FreeRtosTickType,
}


impl Duration {
    /// A new duration from milliseconds.
    pub fn from_ms(milliseconds: u32) -> Self {
        Self::from_ticks(milliseconds / get_tick_period_ms())
    }

    /// A new duration from ticks.
    pub fn from_ticks(ticks: u32) -> Self {
        Self { ticks }
    }

    /// An infinite duration.
    pub fn infinite() -> Self {
        Self::from_ticks(get_max_wait())
    }

    /// A duration of zero, for non-blocking calls.
    pub fn zero() -> Self {
        Self::from_ticks(0)
    }

    /// Smallest unit of measurement, one tick.
    pub fn eps() -> Self {
        Self::from_ticks(1)
    }

    /// Return the duration in milliseconds
    pub fn ms(&self) -> u32 {
        self.ticks * get_tick_period_ms()
    }

    /// Return the duration in ticks.
    pub fn ticks(&self) -> FreeRtosTickType {
        self.ticks
    }
}
