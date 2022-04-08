#![feature(abi_c_cmse_nonsecure_call)]
#![feature(cmse_nonsecure_entry)]
#![no_main]
#![no_std]

use core::ops::Range;
use cortex_m_rt::{exception, ExceptionFrame};
use defmt::println;
use defmt_rtt as _;
use frumsceaft::stm32l562::{Peripheral, GTZC};
use panic_probe as _;
use stm32_hal2::gpio::{Pin, PinMode, Port};

const NON_SECURE_START: u32 = 0x08040000;
const NON_SECURE_STOP: u32 = 0x0807FFFF;
const NON_SECURE_SRAM_START: u32 = 0x20018000;
const NON_SECURE_SRAM_STOP: u32 = 0x2003FFFF;
const NSC_RANGE: Range<u32> = 0x0803E000..0x0803FFFF;

#[cortex_m_rt::entry]
fn main() -> ! {
    println!("boot");
    let _ = Pin::new(Port::G, 12, PinMode::Output);
    let _ = Pin::new(Port::D, 3, PinMode::Output);

    frumsceaft::boot(
        &GTZC,
        frumsceaft::MemoryLayout {
            secure_flash_region: 0..NON_SECURE_START,
            non_secure_flash_region: NON_SECURE_START..NON_SECURE_STOP,
            secure_ram_region: 0..NON_SECURE_SRAM_START,
            non_secure_ram_region: NON_SECURE_SRAM_START..NON_SECURE_SRAM_STOP,
            nsc_flash_region: Some(NSC_RANGE),
        },
        &[Peripheral::GPIOG(12), Peripheral::GPIOD(3)],
    )
}

#[no_mangle]
#[cmse_nonsecure_entry]
pub extern "C" fn secure_test_fn(input: u32) -> u32 {
    input + 6
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    println!("pc {:?}", ef.pc());
    println!("r0 {:?}", ef.r0());
    println!("r1 {:?}", ef.r1());
    println!("r2 {:?}", ef.r2());
    println!("r3 {:?}", ef.r3());
    loop {}
}
