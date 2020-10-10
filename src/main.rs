#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f7xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let gpiob = p.GPIOB.split();
    let mut led = gpiob.pb0.into_push_pull_output();

    // Constrain clocking registers
    let rcc = p.RCC.constrain();

    // Configure clock and freeze it
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    // Get delay provider
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        rprintln!("loop!");
        led.set_high().expect("GPIO can never fail");
        delay.delay_ms(1000_u16);

        led.set_low().expect("GPIO can never fail");
        delay.delay_ms(1000_u16);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
