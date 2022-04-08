#![no_std]
#![feature(abi_c_cmse_nonsecure_call)]
#![doc = include_str!("../README.md")]
use cortex_m::cmse::{AccessType, TestTarget};

use core::ops::Range;

/// The IDAU (Implementation Defined Attribution Unit), is the portion of a TrustZone-M processor
/// responsible for defining memory regions, and managing peripheral use. This trait, allows the user to
/// implement a shared abstraction for an IDAU, so it can be used by Frumscaeft to initialize
/// the non-secure partition.
pub trait IDAU {
    type Peripheral;
    fn set_flash_region_params(&self, region: Range<u32>, params: RegionParams);
    fn set_memory_region_params(&self, region: Range<u32>, params: RegionParams);
    fn set_nsc_region(&self, region: Range<u32>);
    fn pass_peripheral_non_secure(&self, perph: &Self::Peripheral);
    fn prepare_boot(&self);
}

/// RegionParams defines permissions for a flash or RAM region.
#[derive(Debug)]
pub struct RegionParams {
    pub write: bool,
    pub execute: bool,
    pub read: bool,
    pub lock: bool,
    pub secure: bool,
}

/// MemoryLayout specifies boundaries for the non-secure and secure regions.
///
/// Each of these regions should be non-overlapping. It is undefined behavior if there is an overlap between regions.
pub struct MemoryLayout {
    pub secure_flash_region: Range<u32>,
    pub non_secure_flash_region: Range<u32>,
    pub secure_ram_region: Range<u32>,
    pub non_secure_ram_region: Range<u32>,
    pub nsc_flash_region: Option<Range<u32>>,
}

/// Uses the IDAU to set permissions on each flash region, and to mark the passed peripherals as non-secure. Once done, `boot`
/// jumps to the `reset` handler of the non-secure firmware.
///
/// # Example
/// ```no_run
/// use frumsceaft::nrf53::PerphExt;
///
/// let spu = unsafe { &*nrf5340_app_pac::SPU_S::PTR };
/// frumscaeft::boot(
///     spu,
///     boot_oxide::MemoryLayout {
///         secure_flash_region: 0..NON_SECURE_START,
///         non_secure_flash_region: NON_SECURE_START..ROM_SIZE,
///         secure_ram_region: 0..NON_SECURE_SRAM_START,
///         non_secure_ram_region: NON_SECURE_SRAM_START..RAM_SIZE,
///         nsc_flash_region: None,
///     },
///     &[
///         nrf5340_app_pac::P0_NS::perph(),
///         nrf5340_app_pac::MUTEX_NS::perph(),
///         nrf5340_app_pac::UARTE0_NS::perph(),
///         nrf5340_app_pac::TIMER0_NS::perph(),
///     ],
/// )
/// ```
pub fn boot<I: IDAU>(idau: &I, layout: MemoryLayout, peripherals: &[I::Peripheral]) -> ! {
    let non_secure_start = layout.non_secure_flash_region.start;
    idau.set_flash_region_params(
        layout.secure_flash_region,
        RegionParams {
            write: true,
            read: true,
            lock: true,
            secure: true,
            execute: true,
        },
    );

    idau.set_flash_region_params(
        layout.non_secure_flash_region,
        RegionParams {
            write: true,
            read: true,
            lock: true,
            secure: false,
            execute: true,
        },
    );

    idau.set_memory_region_params(
        layout.non_secure_ram_region,
        RegionParams {
            write: true,
            read: true,
            lock: true,
            execute: true,
            secure: false,
        },
    );

    idau.set_memory_region_params(
        layout.secure_ram_region,
        RegionParams {
            write: true,
            read: true,
            lock: true,
            execute: true,
            secure: true,
        },
    );
    if let Some(nsc_flash_region) = layout.nsc_flash_region {
        idau.set_nsc_region(nsc_flash_region);
    }

    for p in peripherals {
        idau.pass_peripheral_non_secure(p);
    }

    idau.prepare_boot();

    unsafe {
        let ns_vector_table = non_secure_start as *const u32;
        // get scb (system control block) peripheral
        let scb = &*cortex_m::peripheral::SCB::PTR;

        // set VTOR_NS
        core::ptr::write_volatile(0xE002ED08 as *mut u32, non_secure_start);

        // set aircr to a pre-calculated value
        // this value is equivalent to enabling two settings: priotize secure exceptions
        // & send non-banked exceptions to non-secure
        scb.aircr.write(0x5FA56000);

        // ensure that flash region has the appropriate permissions
        let region_access = TestTarget::check(
            non_secure_start as *mut u32,
            AccessType::NonSecureUnprivileged,
        );
        if region_access.secure() {
            panic!()
        }

        // do some pointer math to get the address of the "reset vector"
        let ns_reset_vector = *((non_secure_start + 4) as *const u32);
        // set the non-secure MSP
        cortex_m::register::msp::write_ns(*ns_vector_table);

        // cast our reset vector, as a non-secure function call
        let reset_ns: extern "C-cmse-nonsecure-call" fn() -> ! =
            core::mem::transmute(ns_reset_vector);
        reset_ns()
    }
}

#[cfg(feature = "nrf53")]
pub mod nrf53;
