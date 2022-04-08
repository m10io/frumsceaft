#![feature(abi_c_cmse_nonsecure_call)]
#![feature(cmse_nonsecure_entry)]
#![no_main]
#![no_std]

use rtt_target::{rprintln, rtt_init_print};

const NON_SECURE_START: u32 = 0x00050000u32;
const NON_SECURE_SRAM_START: u32 = 0x10000u32;
const ROM_SIZE: u32 = 0x100000;
const RAM_SIZE: u32 = 0x80000;

use frumsceaft::nrf53::PerphExt;

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// values imported from linker script
extern "C" {
    static __sg_start: u8;
    static __sg_end: u8;
    static __sg_size: u8;
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("secure start");
    let sg_start = unsafe { &__sg_start as *const u8 as u32 };
    let sg_end = unsafe { &__sg_end as *const u8 as u32 };
    let spu = unsafe { &*nrf5340_app_pac::SPU_S::ptr() };
    frumsceaft::boot(
        spu,
        frumsceaft::MemoryLayout {
            secure_flash_region: 0..NON_SECURE_START,
            non_secure_flash_region: NON_SECURE_START..ROM_SIZE,
            secure_ram_region: 0..NON_SECURE_SRAM_START,
            non_secure_ram_region: NON_SECURE_SRAM_START..RAM_SIZE,
            nsc_flash_region: Some(sg_start..sg_end),
        },
        &[
            nrf5340_app_pac::P0_NS::perph(),
            nrf5340_app_pac::MUTEX_NS::perph(),
            nrf5340_app_pac::UARTE0_NS::perph(),
            nrf5340_app_pac::TIMER0_NS::perph(),
        ],
    )
}

#[no_mangle]
#[cmse_nonsecure_entry]
pub extern "C" fn secure_test_fn(input: u32) -> u32 {
    input + 6
}
