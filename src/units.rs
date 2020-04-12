use crate::prelude::v1::*;
use crate::shim::*;
use crate::base::FreeRtosTickType;

pub trait FreeRtosTimeUnits {
    fn get_tick_period_ms() -> u32;
    fn get_max_wait() -> u32;
}

#[derive(Copy, Clone, Default)]
pub struct FreeRtosTimeUnitsShimmed;
impl FreeRtosTimeUnits for FreeRtosTimeUnitsShimmed {
    #[inline]
    fn get_tick_period_ms() -> u32 {
        unsafe { freertos_rs_get_portTICK_PERIOD_MS() }
    }
    #[inline]
    fn get_max_wait() -> u32 {
        unsafe { freertos_rs_max_wait() }
    }
}

pub trait DurationTicks : Copy + Clone {
    /// Convert to ticks, the internal time measurement unit of FreeRTOS
    fn to_ticks(&self) -> FreeRtosTickType;
}

pub type Duration = DurationImpl<FreeRtosTimeUnitsShimmed>;

/// Time unit used by FreeRTOS, passed to the scheduler as ticks.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DurationImpl<T> {
    ticks: u32,
    _time_units: PhantomData<T>
}

impl<T> DurationImpl<T> where T: FreeRtosTimeUnits + Copy {
    /// Milliseconds constructor
    pub fn ms(milliseconds: u32) -> Self {
        Self::ticks(milliseconds / T::get_tick_period_ms())
    }

    pub fn ticks(ticks: u32) -> Self {
        DurationImpl { ticks: ticks, _time_units: PhantomData }
    }

    /// An infinite duration
    pub fn infinite() -> Self {
        Self::ticks(T::get_max_wait())
    }

    /// A duration of zero, for non-blocking calls
    pub fn zero() -> Self {
        Self::ticks(0)
    }

    /// Smallest unit of measurement, one tick
    pub fn eps() -> Self {
        Self::ticks(1)
    }

    pub fn to_ms(&self) -> u32 {
        self.ticks * T::get_tick_period_ms()
    }
}

impl<T> DurationTicks for DurationImpl<T> where T: FreeRtosTimeUnits + Copy {
    fn to_ticks(&self) -> FreeRtosTickType {
        self.ticks
    }
}