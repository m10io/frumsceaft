use crate::RegionParams;
use core::ops::Range;
use cortex_m::peripheral::sau::{SauRegion, SauRegionAttribute};
use stm32l5::stm32l562::Interrupt;

pub struct GTZC;

impl crate::IDAU for GTZC {
    type Peripheral = Peripheral;
    fn set_flash_region_params(&self, region: Range<u32>, params: RegionParams) {
        let p = unsafe { cortex_m::peripheral::Peripherals::steal() };
        let mut sau = p.SAU;
        if !params.secure {
            defmt::println!("ns region start: {:x} end: {:x}", region.start, region.end);
            sau.set_region(
                0,
                SauRegion {
                    base_address: region.start,
                    limit_address: region.end,
                    attribute: SauRegionAttribute::NonSecure,
                },
            )
            .unwrap();
        }
    }

    fn set_memory_region_params(&self, region: Range<u32>, params: RegionParams) {
        let p = unsafe { cortex_m::peripheral::Peripherals::steal() };
        let mpcbb1 = unsafe { &*stm32l5::stm32l562::SEC_GTZC_MPCBB1::PTR };
        mpcbb1
            .cr
            .write(|w| w.srwiladis().set_bit().invsecstate().clear_bit());
        mpcbb1.lckvtr1.write(|w| unsafe { w.bits(0x0) });
        for i in 0..12 {
            mpcbb1.vctr[i].write(|w| unsafe { w.bits(0xFFFFFFFF) });
        }
        for i in 12..24 {
            mpcbb1.vctr[i].write(|w| unsafe { w.bits(0x0) });
        }
        let mut sau = p.SAU;
        if !params.secure {
            sau.set_region(
                1,
                SauRegion {
                    base_address: region.start,
                    limit_address: region.end,
                    attribute: SauRegionAttribute::NonSecure,
                },
            )
            .unwrap();
        }
    }

    fn set_nsc_region(&self, region: Range<u32>) {
        let p = unsafe { cortex_m::peripheral::Peripherals::steal() };
        let mut sau = p.SAU;
        sau.set_region(
            2,
            SauRegion {
                base_address: region.start,
                limit_address: region.end,
                attribute: SauRegionAttribute::NonSecureCallable,
            },
        )
        .unwrap();
    }

    fn pass_peripheral_non_secure(&self, perph: &Self::Peripheral) {
        let tzsc = unsafe { &*stm32l5::stm32l562::SEC_GTZC_TZSC::PTR };
        match perph {
            Peripheral::ADC => {
                tzsc.seccfgr2.write(|w| w.adcsec().clear_bit());
            }
            Peripheral::AES => {
                tzsc.seccfgr2.write(|w| w.aessec().clear_bit());
            }
            Peripheral::CRC => {
                tzsc.seccfgr2.write(|w| w.crcsec().clear_bit());
            }
            Peripheral::DFSDM => {
                tzsc.seccfgr2.write(|w| w.dfsdm1sec().clear_bit());
            }
            Peripheral::FSMCReg => {
                tzsc.seccfgr2.write(|w| w.dfsdm1sec().clear_bit());
            }
            Peripheral::Hash => {
                tzsc.seccfgr2.write(|w| w.hashsec().clear_bit());
            }
            Peripheral::Icache => {
                tzsc.seccfgr2.write(|w| w.icachesec().clear_bit());
            }
            Peripheral::OctoSPI1 => {
                tzsc.seccfgr2.write(|w| w.octospi1_regsec().clear_bit());
            }
            Peripheral::PKA => {
                tzsc.seccfgr2.write(|w| w.pkasec().clear_bit());
            }
            Peripheral::RNG => {
                tzsc.seccfgr2.write(|w| w.rngsec().clear_bit());
            }
            Peripheral::SAI1 => {
                tzsc.seccfgr2.write(|w| w.sai1sec().clear_bit());
            }
            Peripheral::SAI2 => {
                tzsc.seccfgr2.write(|w| w.sai2sec().clear_bit());
            }
            Peripheral::SDMMC1 => {
                tzsc.seccfgr2.write(|w| w.sdmmc1sec().clear_bit());
            }
            Peripheral::Comp => {
                tzsc.seccfgr1.write(|w| w.compsec().clear_bit());
            }
            Peripheral::Crs => {
                tzsc.seccfgr1.write(|w| w.crssec().clear_bit());
            }
            Peripheral::Dac => {
                tzsc.seccfgr1.write(|w| w.dacsec().clear_bit());
            }
            Peripheral::FdCan => {
                tzsc.seccfgr1.write(|w| w.fdcan1sec().clear_bit());
            }
            Peripheral::I2C1 => {
                tzsc.seccfgr1.write(|w| w.i2c1sec().clear_bit());
            }
            Peripheral::I2C2 => {
                tzsc.seccfgr1.write(|w| w.i2c2sec().clear_bit());
            }
            Peripheral::I2C3 => {
                tzsc.seccfgr1.write(|w| w.i2c3sec().clear_bit());
            }
            Peripheral::I2C4 => {
                tzsc.seccfgr1.write(|w| w.i2c4sec().clear_bit());
            }
            Peripheral::IWDG => {
                tzsc.seccfgr1.write(|w| w.iwdgsec().clear_bit());
            }
            Peripheral::LPTIM1 => {
                tzsc.seccfgr1.write(|w| w.lptim1sec().clear_bit());
            }
            Peripheral::LPTIM2 => {
                tzsc.seccfgr1.write(|w| w.lptim2sec().clear_bit());
            }
            Peripheral::LPUart => {
                tzsc.seccfgr1.write(|w| w.lpuart1sec().clear_bit());
            }
            Peripheral::OPAMP => {
                tzsc.seccfgr1.write(|w| w.opampsec().clear_bit());
            }
            Peripheral::SPI1 => {
                tzsc.seccfgr1.write(|w| w.spi1sec().clear_bit());
            }
            Peripheral::SPI2 => {
                tzsc.seccfgr1.write(|w| w.spi2sec().clear_bit());
            }
            Peripheral::SPI3 => {
                tzsc.seccfgr1.write(|w| w.spi3sec().clear_bit());
            }
            Peripheral::Tim1 => {
                tzsc.seccfgr1.write(|w| w.tim1sec().clear_bit());
            }
            Peripheral::Tim2 => {
                tzsc.seccfgr1.write(|w| w.tim2sec().clear_bit());
            }
            Peripheral::Tim3 => {
                tzsc.seccfgr1.write(|w| w.tim3sec().clear_bit());
            }
            Peripheral::Tim4 => {
                tzsc.seccfgr1.write(|w| w.tim4sec().clear_bit());
            }
            Peripheral::Tim5 => {
                tzsc.seccfgr1.write(|w| w.tim5sec().clear_bit());
            }
            Peripheral::Tim6 => {
                tzsc.seccfgr1.write(|w| w.tim6sec().clear_bit());
            }
            Peripheral::Tim7 => {
                tzsc.seccfgr1.write(|w| w.tim7sec().clear_bit());
            }
            Peripheral::Tim8 => {
                tzsc.seccfgr2.write(|w| w.tim8sec().clear_bit());
            }
            Peripheral::Tim15 => {
                tzsc.seccfgr2.write(|w| w.tim15sec().clear_bit());
            }
            Peripheral::Tim16 => {
                tzsc.seccfgr2.write(|w| w.tim16sec().clear_bit());
            }
            Peripheral::Tim17 => {
                tzsc.seccfgr2.write(|w| w.tim17sec().clear_bit());
            }
            Peripheral::TSC => {
                tzsc.seccfgr2.write(|w| w.tscsec().clear_bit());
            }
            Peripheral::Uart4 => {
                tzsc.seccfgr1.write(|w| w.uart4sec().clear_bit());
            }
            Peripheral::Uart5 => {
                tzsc.seccfgr1.write(|w| w.uart5sec().clear_bit());
            }
            Peripheral::Ucpd1 => {
                tzsc.seccfgr1.write(|w| w.ucpd1sec().clear_bit());
            }
            Peripheral::Usart1 => {
                tzsc.seccfgr2.write(|w| w.usart1sec().clear_bit());
            }
            Peripheral::Usart2 => {
                tzsc.seccfgr1.write(|w| w.usart2sec().clear_bit());
            }
            Peripheral::Usart3 => {
                tzsc.seccfgr1.write(|w| w.usart3sec().clear_bit());
            }
            Peripheral::USBFS => {
                tzsc.seccfgr1.write(|w| w.usbfssec().clear_bit());
            }
            Peripheral::VrefBuf => {
                tzsc.seccfgr1.write(|w| w.vrefbufsec().clear_bit());
            }
            Peripheral::WWDG => {
                tzsc.seccfgr1.write(|w| w.wwdgsec().clear_bit());
            }
            Peripheral::GPIOA(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOA::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOB(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOB::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOC(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOC::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOD(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOD::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOE(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOE::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOF(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOF::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOG(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOG::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::GPIOH(p) => {
                let gpio = unsafe { &*stm32l5::stm32l562::SEC_GPIOH::PTR };
                gpio.seccfgr.write(|w| unsafe { w.sec(*p).clear_bit() });
            }
            Peripheral::DMA1 => {
                let dma1 = unsafe { &*stm32l5::stm32l562::SEC_DMA1::PTR };
                dma1.ccr1.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr2.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr3.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr4.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr5.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr6.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr7.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma1.ccr8.write(|w| w.secm().clear_bit().dsec().clear_bit());
            }
            Peripheral::DMA2 => {
                let dma2 = unsafe { &*stm32l5::stm32l562::SEC_DMA2::PTR };
                dma2.ccr1.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr2.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr3.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr4.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr5.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr6.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr7.write(|w| w.secm().clear_bit().dsec().clear_bit());
                dma2.ccr8.write(|w| w.secm().clear_bit().dsec().clear_bit());
            }
        }
        perph.enable_interrupt();
    }

    fn prepare_boot(&self) {
        let p = unsafe { cortex_m::peripheral::Peripherals::steal() };
        let mut sau = p.SAU;
        let syscfg = unsafe { &*stm32l5::stm32l562::SYSCFG::PTR };
        syscfg
            .seccfgr
            .write(|w| w.syscfgsec().clear_bit().classbsec().clear_bit());
        // set all peripheral memory blocks as non-secure GTZC
        sau.set_region(
            3,
            SauRegion {
                base_address: 0x40000000,
                limit_address: 0x4FFFFFFF,
                attribute: SauRegionAttribute::NonSecure,
            },
        )
        .unwrap();
        // set external flash as non-secure
        // TODO: Make this optional somehow (maybe an enum for the periph)
        sau.set_region(
            4,
            SauRegion {
                base_address: 0x60000000,
                limit_address: 0x9FFFFFFF,
                attribute: SauRegionAttribute::NonSecure,
            },
        )
        .unwrap();
        // NOTE(sphw): not sure what this is, should probably figure it out
        sau.set_region(
            5,
            SauRegion {
                base_address: 0x0BF90000,
                limit_address: 0x0BFA8FFF,
                attribute: SauRegionAttribute::NonSecure,
            },
        )
        .unwrap();
        sau.enable();
    }
}

pub enum Peripheral {
    ADC,
    AES,
    CRC,
    DFSDM,
    FSMCReg,
    Hash,
    Icache,
    OctoSPI1,
    PKA,
    RNG,
    SAI1,
    SAI2,
    SDMMC1,
    Comp,
    Crs,
    Dac,
    FdCan,
    I2C1,
    I2C2,
    I2C3,
    I2C4,
    IWDG,
    LPTIM1,
    LPTIM2,
    LPUart,
    OPAMP,
    SPI1,
    SPI2,
    SPI3,
    Tim1,
    Tim2,
    Tim3,
    Tim4,
    Tim5,
    Tim6,
    Tim7,
    Tim8,
    Tim15,
    Tim16,
    Tim17,
    TSC,
    Uart4,
    Uart5,
    Ucpd1,
    Usart1,
    Usart2,
    Usart3,
    USBFS,
    VrefBuf,
    WWDG,
    GPIOA(usize),
    GPIOB(usize),
    GPIOC(usize),
    GPIOD(usize),
    GPIOE(usize),
    GPIOF(usize),
    GPIOG(usize),
    GPIOH(usize),
    DMA1,
    DMA2,
}
impl Peripheral {
    pub fn all() -> &'static [Peripheral] {
        use Peripheral::*;
        &[
            ADC,
            AES,
            CRC,
            DFSDM,
            FSMCReg,
            Hash,
            Icache,
            OctoSPI1,
            PKA,
            RNG,
            SAI1,
            SAI2,
            SDMMC1,
            Comp,
            Crs,
            Dac,
            FdCan,
            I2C1,
            I2C2,
            I2C3,
            I2C4,
            IWDG,
            LPTIM1,
            LPTIM2,
            LPUart,
            OPAMP,
            SPI1,
            SPI2,
            SPI3,
            Tim1,
            Tim2,
            Tim3,
            Tim4,
            Tim5,
            Tim6,
            Tim7,
            Tim8,
            Tim15,
            Tim16,
            Tim17,
            TSC,
            Uart4,
            Uart5,
            Ucpd1,
            Usart1,
            Usart2,
            Usart3,
            USBFS,
            VrefBuf,
            WWDG,
            GPIOA(1),
            GPIOA(2),
            GPIOA(3),
            GPIOA(4),
            GPIOA(5),
            GPIOA(6),
            GPIOA(7),
            GPIOA(8),
            GPIOA(9),
            GPIOA(10),
            GPIOA(11),
            GPIOA(12),
            GPIOA(13),
            GPIOA(14),
            GPIOA(15),
            GPIOB(1),
            GPIOB(2),
            GPIOB(3),
            GPIOB(4),
            GPIOB(5),
            GPIOB(6),
            GPIOB(7),
            GPIOB(8),
            GPIOB(9),
            GPIOB(10),
            GPIOB(11),
            GPIOB(12),
            GPIOB(13),
            GPIOB(14),
            GPIOB(15),
            GPIOC(1),
            GPIOC(2),
            GPIOC(3),
            GPIOC(4),
            GPIOC(5),
            GPIOC(6),
            GPIOC(7),
            GPIOC(8),
            GPIOC(9),
            GPIOC(10),
            GPIOC(11),
            GPIOC(12),
            GPIOC(13),
            GPIOC(14),
            GPIOC(15),
            GPIOD(1),
            GPIOD(2),
            GPIOD(3),
            GPIOD(4),
            GPIOD(5),
            GPIOD(6),
            GPIOD(7),
            GPIOD(8),
            GPIOD(9),
            GPIOD(10),
            GPIOD(11),
            GPIOD(12),
            GPIOD(13),
            GPIOD(14),
            GPIOD(15),
            GPIOE(1),
            GPIOE(2),
            GPIOE(3),
            GPIOE(4),
            GPIOE(5),
            GPIOE(6),
            GPIOE(7),
            GPIOE(8),
            GPIOE(9),
            GPIOE(10),
            GPIOE(11),
            GPIOE(12),
            GPIOE(13),
            GPIOE(14),
            GPIOE(15),
            GPIOF(1),
            GPIOF(2),
            GPIOF(3),
            GPIOF(4),
            GPIOF(5),
            GPIOF(6),
            GPIOF(7),
            GPIOF(8),
            GPIOF(9),
            GPIOF(10),
            GPIOF(11),
            GPIOF(12),
            GPIOF(13),
            GPIOG(1),
            GPIOG(2),
            GPIOG(3),
            GPIOG(4),
            GPIOG(5),
            GPIOG(6),
            GPIOG(7),
            GPIOG(8),
            GPIOG(9),
            GPIOG(10),
            GPIOG(11),
            GPIOG(12),
            GPIOG(13),
            GPIOG(14),
            GPIOG(15),
            GPIOH(1),
            GPIOH(2),
            GPIOH(3),
            GPIOH(4),
            GPIOH(5),
            GPIOH(6),
            GPIOH(7),
            GPIOH(8),
            GPIOH(9),
            GPIOH(10),
            GPIOH(11),
            GPIOH(12),
            GPIOH(13),
            GPIOH(14),
            GPIOH(15),
            DMA1,
            DMA2,
        ]
    }
}

impl Peripheral {
    fn enable_interrupt(&self) {
        match self {
            Peripheral::Icache => enable_int(Interrupt::ICACHE as usize),
            Peripheral::OctoSPI1 => {
                enable_int(Interrupt::OCTOSPI1 as usize);
            }
            Peripheral::PKA => {
                enable_int(Interrupt::PKA as usize);
            }
            Peripheral::RNG => {
                enable_int(Interrupt::RNG as usize);
            }
            Peripheral::SAI1 => {
                enable_int(Interrupt::SAI1 as usize);
            }
            Peripheral::SAI2 => {
                enable_int(Interrupt::SAI2 as usize);
            }
            Peripheral::SDMMC1 => {
                enable_int(Interrupt::SDMMC1 as usize);
            }
            Peripheral::Comp => {
                enable_int(Interrupt::COMP as usize);
            }
            Peripheral::FdCan => {
                enable_int(Interrupt::FDCAN1_IT0 as usize);
                enable_int(Interrupt::FDCAN1_IT1 as usize);
            }
            Peripheral::I2C1 => {
                enable_int(Interrupt::I2C1_EV as usize);
                enable_int(Interrupt::I2C1_ER as usize);
            }
            Peripheral::I2C2 => {
                enable_int(Interrupt::I2C2_EV as usize);
                enable_int(Interrupt::I2C2_ER as usize);
            }
            Peripheral::I2C3 => {
                enable_int(Interrupt::I2C3_EV as usize);
                enable_int(Interrupt::I2C3_ER as usize);
            }
            Peripheral::I2C4 => {
                enable_int(Interrupt::I2C4_EV as usize);
                enable_int(Interrupt::I2C4_ER as usize);
            }
            Peripheral::LPTIM1 => enable_int(Interrupt::LPTIM1 as usize),
            Peripheral::LPTIM2 => enable_int(Interrupt::LPTIM2 as usize),
            Peripheral::LPUart => enable_int(Interrupt::LPUART1 as usize),
            Peripheral::SPI1 => enable_int(Interrupt::SPI1 as usize),
            Peripheral::SPI2 => enable_int(Interrupt::SPI2 as usize),
            Peripheral::SPI3 => enable_int(Interrupt::SPI3 as usize),
            Peripheral::Tim1 => {
                enable_int(Interrupt::TIM1_BRK as usize);
                enable_int(Interrupt::TIM1_CC as usize);
                enable_int(Interrupt::TIM1_TRG_COM as usize);
                enable_int(Interrupt::TIM1_UP as usize);
            }
            Peripheral::Tim2 => enable_int(Interrupt::TIM2 as usize),
            Peripheral::Tim3 => enable_int(Interrupt::TIM3 as usize),
            Peripheral::Tim4 => enable_int(Interrupt::TIM4 as usize),
            Peripheral::Tim5 => enable_int(Interrupt::TIM5 as usize),
            Peripheral::Tim6 => enable_int(Interrupt::TIM6 as usize),
            Peripheral::Tim7 => enable_int(Interrupt::TIM7 as usize),
            Peripheral::Tim8 => {
                enable_int(Interrupt::TIM8_BRK as usize);
                enable_int(Interrupt::TIM8_CC as usize);
                enable_int(Interrupt::TIM8_TRG_COM as usize);
                enable_int(Interrupt::TIM8_UP as usize);
            }
            Peripheral::Tim15 => enable_int(Interrupt::TIM15 as usize),
            Peripheral::Tim16 => enable_int(Interrupt::TIM16 as usize),
            Peripheral::Tim17 => enable_int(Interrupt::TIM17 as usize),
            Peripheral::TSC => enable_int(Interrupt::TSC as usize),
            Peripheral::Uart4 => enable_int(Interrupt::UART4 as usize),
            Peripheral::Uart5 => enable_int(Interrupt::UART5 as usize),
            Peripheral::Ucpd1 => enable_int(Interrupt::UCPD1 as usize),
            Peripheral::Usart1 => enable_int(Interrupt::USART1 as usize),
            Peripheral::Usart2 => enable_int(Interrupt::USART2 as usize),
            Peripheral::Usart3 => enable_int(Interrupt::USART3 as usize),
            Peripheral::USBFS => enable_int(Interrupt::USB_FS as usize),
            Peripheral::WWDG => enable_int(Interrupt::WWDG as usize),
            Peripheral::DMA1 => {
                enable_int(Interrupt::DMA1_CH1 as usize);
                enable_int(Interrupt::DMA1_CH2 as usize);
                enable_int(Interrupt::DMA1_CH3 as usize);
                enable_int(Interrupt::DMA1_CH4 as usize);
                enable_int(Interrupt::DMA1_CH5 as usize);
                enable_int(Interrupt::DMA1_CH6 as usize);
                enable_int(Interrupt::DMA1_CH7 as usize);
                enable_int(Interrupt::DMA1_CHANNEL8 as usize);
                enable_int(Interrupt::DMAMUX1_OVR as usize);
            }
            Peripheral::DMA2 => {
                enable_int(Interrupt::DMA2_CH1 as usize);
                enable_int(Interrupt::DMA2_CH2 as usize);
                enable_int(Interrupt::DMA2_CH3 as usize);
                enable_int(Interrupt::DMA2_CH4 as usize);
                enable_int(Interrupt::DMA2_CH5 as usize);
                enable_int(Interrupt::DMA2_CH6 as usize);
                enable_int(Interrupt::DMA2_CH7 as usize);
                enable_int(Interrupt::DMA2_CH8 as usize);
            }
            _ => {}
        };
        let tzic = unsafe { &*stm32l5::stm32l562::SEC_GTZC_TZIC::PTR };
        tzic.ier1.write(|w| unsafe { w.bits(0xFFFFFFFF) });
        tzic.ier2.write(|w| unsafe { w.bits(0x3FFFFFFF) });
        tzic.ier3.write(|w| unsafe { w.bits(0x000000FF) });
        let rcc = unsafe { &*stm32l5::stm32l562::SEC_RCC::PTR };
        rcc.seccfgr.reset();
        rcc.seccfgr.write(|w| unsafe { w.bits(0x0) });
        enable_int(Interrupt::RTC as usize);
        enable_int(Interrupt::EXTI0 as usize);
        enable_int(Interrupt::EXTI1 as usize);
        enable_int(Interrupt::EXTI2 as usize);
        enable_int(Interrupt::EXTI3 as usize);
        enable_int(Interrupt::EXTI4 as usize);
        enable_int(Interrupt::EXTI5 as usize);
        enable_int(Interrupt::EXTI6 as usize);
        enable_int(Interrupt::EXTI7 as usize);
        enable_int(Interrupt::EXTI8 as usize);
        enable_int(Interrupt::EXTI9 as usize);
        enable_int(Interrupt::EXTI10 as usize);
        enable_int(Interrupt::EXTI11 as usize);
        enable_int(Interrupt::EXTI12 as usize);
        enable_int(Interrupt::EXTI13 as usize);
        enable_int(Interrupt::EXTI14 as usize);
        fn enable_int(id: usize) {
            let peripherals = unsafe { cortex_m::Peripherals::steal() };
            unsafe {
                peripherals.NVIC.icer[id / 32].write(1 << (id % 32));
            }
            unsafe {
                peripherals.NVIC.itns[id / 32].modify(|w| w | 1 << (id & 0x1F));
            }
        }
    }
}
