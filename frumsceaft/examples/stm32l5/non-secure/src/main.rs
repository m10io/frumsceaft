#![no_std]
#![no_main]

use panic_probe as _;

use cortex_m_rt::entry;
use defmt_rtt as _;
use stm32_hal2::{
    self,
    gpio::{Pin, PinMode, Port},
};

#[link(name = "nsclib")]
extern "C" {
    pub fn secure_test_fn(input: u32) -> u32;
}

#[entry]
fn main() -> ! {
    let mut led1 = Pin::new(Port::G, 12, PinMode::Output);
    let mut led2 = Pin::new(Port::D, 3, PinMode::Output);
    defmt::println!("Hello, world!");

    //let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    //clock_cfg.setup().unwrap();

    // Setup a delay, based on the Cortex-m systick.
    //let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    defmt::println!("Our demo is alive");
    let res = unsafe { secure_test_fn(10) };
    defmt::println!("secure value: {:?}", res);
    // Now, enjoy the lightshow!
    let mut i = 0;
    loop {
        i = (i + 1) % 1000;
        if i > 500 {
            led1.set_low();
            led2.set_high();
        } else {
            led1.set_high();
            led2.set_low();
        }
        // delay.delay_ms(1000_u32);
    }
}
