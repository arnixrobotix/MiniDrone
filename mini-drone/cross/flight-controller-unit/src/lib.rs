#![no_main]
#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};
use defmt_brtt as _; // global logger

use panic_probe as _;

pub use stm32f4xx_hal::pac;
use stm32f4xx_hal as _; // memory layout

pub struct FCU {}

impl FCU {
    // same panicking *behavior* as `panic-probe` but doesn't print a panic message
    // this prevents the panic message being printed *twice* when `defmt::panic` is invoked
    #[defmt::panic_handler]
    pub fn panic() -> ! {
        cortex_m::asm::udf()
    }

    pub fn init() -> Self {
        static COUNT: AtomicUsize = AtomicUsize::new(0);
        defmt::timestamp!("{=usize}", {
            // NOTE(no-CAS) `timestamps` runs with interrupts disabled
            let n = COUNT.load(Ordering::Relaxed);
            COUNT.store(n + 1, Ordering::Relaxed);
            n
        });

        Self {}
    }


    /// Terminates the application and makes `probe-rs` exit with exit-code = 0
    pub fn exit() -> ! {
        loop {
            cortex_m::asm::bkpt();
        }
    }
}
