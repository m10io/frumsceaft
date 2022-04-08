# Frumsceaft 

Frumsceaft (pronounced from-shaft) is a Rust library for building a bootloader / secure partition in a TrustZone-M enabled environment. The goal is to provide a minimal set of abstractions that make it easy to build a bootloader. At the moment it supports the Nordic nRF5340 and the STM32L5, but it should be relatively easy to add support for other Cortex-M processors. It provides utilities for setting TrustZone-M memory regions, and passing peripherals. For a more complete description of TrustZone-M read Dimitrios Slamaris's fantastic book  <https://embeddedsecurity.io/>


## Usage

Using Frumsceaft is quite simple, you just need to run `boot` with the appropriate options. Frumscaeft needs a runtime setup. We recommend using [cortex-m-rt](https://github.com/rust-embedded/cortex-m-rt). A more complete example is available in `examples`

``` rust
frumsceaft::boot(
    spu,
    frumsceaft::MemoryLayout {
        secure_flash_region: 0..NON_SECURE_START,
        non_secure_flash_region: NON_SECURE_START..ROM_SIZE,
        secure_ram_region: 0..NON_SECURE_SRAM_START,
        non_secure_ram_region: NON_SECURE_SRAM_START..RAM_SIZE,
        nsc_flash_region: None
    },
    &[
        nrf5340_app_pac::P0_NS::perph(),
        nrf5340_app_pac::MUTEX_NS::perph(),
        nrf5340_app_pac::UARTE0_NS::perph(),
        nrf5340_app_pac::TIMER0_NS::perph(),
    ],
)
```

## Name

Frumsceaft is an Anglo-Saxon word that means "creation" or "origin". Since Frumsceaft will be one of the first things that run on your device it seems fitting.

## Coming Soon (TM)
- [x] Support for the SM32L5
- [x] Build helpers and scripts to make linking veneer implibs easier.
- [ ] Non-secure image signature verification
- [ ] KMU and CryptoCell support libraries
