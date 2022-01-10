use super::IDAU;

const REGION_SIZE: u32 = 0x4000;
const SRAM_REGION_SIZE: u32 = 0x2000;
// pulled from https://docs.zephyrproject.org/latest/reference/kconfig/CONFIG_NRF_SPU_FLASH_REGION_SIZE.html

pub struct NSPeripheral(u8);

impl IDAU for nrf5340_app_pac::spu_s::RegisterBlock {
    type Peripheral = NSPeripheral;

    fn set_flash_region_params(&self, region: core::ops::Range<u32>, params: crate::RegionParams) {
        for i in (region.start / REGION_SIZE)..(region.end / REGION_SIZE) {
            self.flashregion[i as usize].perm.write(|w| {
                if params.write {
                    w.write().enable();
                } else {
                    w.write().disable();
                }
                if params.read {
                    w.read().enable();
                } else {
                    w.read().disable();
                }
                if params.execute {
                    w.execute().enable();
                } else {
                    w.execute().disable();
                }
                if params.lock {
                    w.lock().locked();
                } else {
                    w.lock().unlocked();
                }
                if params.secure {
                    w.secattr().secure();
                } else {
                    w.secattr().non_secure();
                }
                w
            })
        }
    }

    fn set_memory_region_params(&self, region: core::ops::Range<u32>, params: crate::RegionParams) {
        for i in (region.start / SRAM_REGION_SIZE)..(region.end / SRAM_REGION_SIZE) {
            self.ramregion[i as usize].perm.write(|w| {
                if params.write {
                    w.write().enable();
                } else {
                    w.write().disable();
                }
                if params.read {
                    w.read().enable();
                } else {
                    w.read().disable();
                }
                if params.execute {
                    w.execute().enable();
                } else {
                    w.execute().disable();
                }
                if params.lock {
                    w.lock().locked();
                } else {
                    w.lock().unlocked();
                }
                if params.secure {
                    w.secattr().secure();
                } else {
                    w.secattr().non_secure();
                }
                w
            })
        }
    }

    fn set_nsc_region(&self, region: core::ops::Range<u32>) {
        let sg_start = region.start;
        let nsc_size = REGION_SIZE - (sg_start % REGION_SIZE);
        let size_reg = (31 - nsc_size.leading_zeros()) - 4;
        let region_reg = (sg_start as u32 / REGION_SIZE) & 0x3F; // x << SPU_FLASHNSC_REGION_REGION_Pos & SPU_FLASHNSC_REGION_REGION_Msk
        self.flashnsc[0].size.write(|w| {
            unsafe {
                w.bits(size_reg);
            }
            w
        });
        self.flashnsc[0].region.write(|w| {
            unsafe {
                w.bits(region_reg);
            }
            w
        });
    }

    fn pass_peripheral_non_secure(&self, perph: &Self::Peripheral) {
        self.periphid[perph.0 as usize].perm.write(|w| {
            w.secattr().non_secure().lock().locked();
            w
        });
    }

    fn prepare_boot(&self) {
        unsafe {
            self.gpioport[0].perm.write_with_zero(|w| w);

            let sau = &*cortex_m::peripheral::SAU::ptr();
            // disable SAU
            sau.ctrl.modify(|mut ctrl| {
                ctrl.0 &= !1;
                ctrl.0 |= 1 << 1;
                ctrl
            });
        }
    }
}

fn get_perph_id<T>(reg_block: *const T) -> u8 {
    let base_addr = reg_block as u32;
    (base_addr >> 12) as u8
}

pub trait PerphExt {
    fn perph() -> NSPeripheral;
}

macro_rules! impl_perph {
    ($s:ty) => {
        impl PerphExt for $s {
            fn perph() -> NSPeripheral {
                NSPeripheral(get_perph_id(<$s>::ptr()))
            }
        }
    };
}

impl_perph! { nrf5340_app_pac::CLOCK_NS }
impl_perph! { nrf5340_app_pac::COMP_NS }
impl_perph! { nrf5340_app_pac::CTRLAP_NS }
impl_perph! { nrf5340_app_pac::DCNF_NS }
impl_perph! { nrf5340_app_pac::DPPIC_NS }
impl_perph! { nrf5340_app_pac::DWT }
impl_perph! { nrf5340_app_pac::EGU0_NS }
impl_perph! { nrf5340_app_pac::EGU1_NS }
impl_perph! { nrf5340_app_pac::EGU2_NS }
impl_perph! { nrf5340_app_pac::EGU3_NS }
impl_perph! { nrf5340_app_pac::EGU4_NS }
impl_perph! { nrf5340_app_pac::EGU5_NS }
impl_perph! { nrf5340_app_pac::FPU_NS }
impl_perph! { nrf5340_app_pac::GPIOTE1_NS }
impl_perph! { nrf5340_app_pac::I2S0_NS }
impl_perph! { nrf5340_app_pac::IPC_NS }
impl_perph! { nrf5340_app_pac::KMU_NS }
impl_perph! { nrf5340_app_pac::LPCOMP_NS }
impl_perph! { nrf5340_app_pac::MPU }
impl_perph! { nrf5340_app_pac::MUTEX_NS }
impl_perph! { nrf5340_app_pac::NFCT_NS }
impl_perph! { nrf5340_app_pac::NVIC }
impl_perph! { nrf5340_app_pac::NVMC_NS }
impl_perph! { nrf5340_app_pac::OSCILLATORS_NS }
impl_perph! { nrf5340_app_pac::P0_NS }
impl_perph! { nrf5340_app_pac::P1_NS }
impl_perph! { nrf5340_app_pac::PDM0_NS }
impl_perph! { nrf5340_app_pac::POWER_NS }
impl_perph! { nrf5340_app_pac::PWM0_NS }
impl_perph! { nrf5340_app_pac::PWM1_NS }
impl_perph! { nrf5340_app_pac::PWM2_NS }
impl_perph! { nrf5340_app_pac::PWM3_NS }
impl_perph! { nrf5340_app_pac::QDEC0_NS }
impl_perph! { nrf5340_app_pac::QDEC1_NS }
impl_perph! { nrf5340_app_pac::QSPI_NS }
impl_perph! { nrf5340_app_pac::REGULATORS_NS }
impl_perph! { nrf5340_app_pac::RESET_NS }
impl_perph! { nrf5340_app_pac::RTC0_NS }
impl_perph! { nrf5340_app_pac::RTC1_NS }
impl_perph! { nrf5340_app_pac::SAADC_NS }
impl_perph! { nrf5340_app_pac::SPIM0_NS }
impl_perph! { nrf5340_app_pac::SPIM1_NS }
impl_perph! { nrf5340_app_pac::SPIM2_NS }
impl_perph! { nrf5340_app_pac::SPIM3_NS }
impl_perph! { nrf5340_app_pac::SPIM4_NS }
impl_perph! { nrf5340_app_pac::SPIS0_NS }
impl_perph! { nrf5340_app_pac::SPIS1_NS }
impl_perph! { nrf5340_app_pac::SPIS2_NS }
impl_perph! { nrf5340_app_pac::SPIS3_NS }
impl_perph! { nrf5340_app_pac::TIMER0_NS }
impl_perph! { nrf5340_app_pac::TIMER1_NS }
impl_perph! { nrf5340_app_pac::TIMER2_NS }
impl_perph! { nrf5340_app_pac::TPIU }
impl_perph! { nrf5340_app_pac::TWIM0_NS }
impl_perph! { nrf5340_app_pac::TWIM1_NS }
impl_perph! { nrf5340_app_pac::TWIM2_NS }
impl_perph! { nrf5340_app_pac::TWIM3_NS }
impl_perph! { nrf5340_app_pac::TWIS0_NS }
impl_perph! { nrf5340_app_pac::TWIS1_NS }
impl_perph! { nrf5340_app_pac::TWIS2_NS }
impl_perph! { nrf5340_app_pac::TWIS3_NS }
impl_perph! { nrf5340_app_pac::UARTE0_NS }
impl_perph! { nrf5340_app_pac::UARTE1_NS }
impl_perph! { nrf5340_app_pac::UARTE2_NS }
impl_perph! { nrf5340_app_pac::UARTE3_NS }
impl_perph! { nrf5340_app_pac::USBD_NS }
impl_perph! { nrf5340_app_pac::USBREGULATOR_NS }
impl_perph! { nrf5340_app_pac::VMC_NS }
impl_perph! { nrf5340_app_pac::WDT0_NS }
impl_perph! { nrf5340_app_pac::WDT1_NS }
