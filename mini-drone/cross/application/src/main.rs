#![no_main]
#![no_std]
// #![feature(type_alias_impl_trait)]

use flight_controller_unit::{FCU}; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = flight_controller_unit::pac,
    dispatchers = [TIM2]
)]
mod app {
    // Shared resources go here
    #[shared]
    struct Shared {
        fcu: flight_controller_unit::FCU,
    }

    // Local resources go here
    #[local]
    struct Local {
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        // TODO setup monotonic if used
        // let sysclk = { /* clock setup + returning sysclk as an u32 */ };
        // let token = rtic_monotonics::create_systick_token!();
        // rtic_monotonics::systick::Systick::new(cx.core.SYST, sysclk, token);

        task1::spawn().ok();

        (
            Shared {
                fcu: flight_controller_unit::FCU::init(),
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");


        loop {
            continue;
        }
    }

    #[task(shared = [&fcu], priority = 1)]
    async fn task1(cx: task1::Context) {
        let _fcu = cx.shared.fcu;
        defmt::info!("Hello from task1!");
    }
}
