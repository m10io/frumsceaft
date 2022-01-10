#![no_main]
#![no_std]

use core::fmt::Write;
use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::uarte;
use hal::Timer;
use hal::Uarte;
use nb::block;
use nrf5340_app_hal as hal;
use nrf5340_app_hal::gpio::Level;

#[link(name = "nsclib")]
extern "C" {
    pub fn secure_test_fn(input: u32) -> u32;
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let pins0 = hal::gpio::p0::Parts::new(p.P0_NS);
    let button = pins0.p0_23.into_pullup_input();
    let mut cdc_uart = Uarte::new(
        p.UARTE0_NS,
        uarte::Pins {
            txd: pins0.p0_20.into_push_pull_output(Level::High).degrade(),
            rxd: pins0.p0_22.into_floating_input().degrade(),
            cts: Some(pins0.p0_21.into_floating_input().degrade()),
            rts: Some(pins0.p0_19.into_push_pull_output(Level::High).degrade()),
        },
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );
    writeln!(cdc_uart, "Hello, world!").unwrap();
    let res = unsafe { secure_test_fn(10) };
    writeln!(cdc_uart, "Result called from secure: {:?}", res).unwrap();

    let mut timer = Timer::new(p.TIMER0_NS);
    let mut leds = [
        pins0.p0_28.degrade().into_push_pull_output(Level::Low),
        pins0.p0_29.degrade().into_push_pull_output(Level::Low),
        pins0.p0_31.degrade().into_push_pull_output(Level::Low),
        pins0.p0_30.degrade().into_push_pull_output(Level::Low),
    ];
    let mut led_index = 0;
    loop {
        if button.is_high().unwrap() {
            for (i, led) in leds.iter_mut().enumerate() {
                if led_index == i {
                    led.set_low().unwrap();
                } else {
                    led.set_high().unwrap();
                }
            }
            led_index = (led_index + 1) % leds.len();
        }

        timer.start(200_000_u32);
        block!(timer.wait()).unwrap();
    }
}
