use stm32f3_discovery::stm32f3xx_hal as hal;

use cortex_m::peripheral::DWT;
use hal::{
    rcc::Clocks,
    time::rate::Hertz,
};

/// A monotonic nondecreasing timer. This is a resurrection of MonoTimer from
/// the stm32f3xx-hal where it got removed after 0.6.1.
#[derive(Clone, Copy)]
pub struct MonoTimer {
    frequency: Hertz,
}

// TODO: What about a refactoring to implement Clock from embedded-time?
impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(mut dwt: DWT, clocks: Clocks) -> Self {
        dwt.enable_cycle_counter();

        // now the CYCCNT counter can't be stopped or resetted
        drop(dwt);

        MonoTimer {
            frequency: clocks.hclk(),
        }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(self) -> Hertz {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(self) -> Instant {
        Instant {
            now: DWT::get_cycle_count(),
        }
    }
}

/// A measurement of a monotonically nondecreasing clock
#[derive(Clone, Copy)]
pub struct Instant {
    now: u32,
}

impl Instant {
    /// Ticks elapsed since the `Instant` was created
    pub fn elapsed(self) -> u32 {
        DWT::get_cycle_count().wrapping_sub(self.now)
    }
}
