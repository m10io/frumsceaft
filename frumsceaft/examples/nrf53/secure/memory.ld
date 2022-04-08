/* Linker script for the nRF9160 in Non-secure mode. It assumes you have the
Nordic Secure Partition Manager installed at the bottom of flash and that
the SPM is set to boot a non-secure application from the FLASH origin below. */

MEMORY
{
    /*
     * This is where the Bootloader, Secure Partition Manager or
     * Trusted-Firmware-M lives.
     */
    /*
     * This is where your non-secure Rust application lives. Note that SPM must agree this
     * is where your application lives, or it will jump to garbage and crash the CPU.
     SIG          : ORIGIN =  0x00050000, LENGTH = 1K
     */
    FLASH        : ORIGIN = 0x00000000, LENGTH = 256K
    /*
     * This RAM is reserved for the Secure-Mode code located in the `SECURE_FLASH` region.
     */
    /* SECURE_RAM   : ORIGIN = 0x20000000, LENGTH = 64K */
    /*
     * This RAM is available to both the Cortex-M33 and the LTE core (well,
        technically anything between `0x2000_0000` and `0x2001_FFFF` is
        shareable, but we just gave the first 64 KiB to Secure Mode). Shared
        buffers must be placed here.
     */
    SHARED_RAM   : ORIGIN = 0x20010000, LENGTH = 64K
    /*
     * This RAM is available to your non-secure Rust application.
    RAM          : ORIGIN = 0x20020000, LENGTH = 128K
     */
    RAM          : ORIGIN = 0x20000000, LENGTH = 128K
}


#define CONFIG_NRF_SPU_FLASH_REGION_SIZE 0x4000

#define __NSC_ALIGN (ALIGN(CONFIG_NRF_SPU_FLASH_REGION_SIZE)  - MAX(32, (1 << LOG2CEIL(__sg_size))))
#define NSC_ALIGN . =  (__NSC_ALIGN + ((ABSOLUTE(.) > __NSC_ALIGN)  ? CONFIG_NRF_SPU_FLASH_REGION_SIZE : 0))
#define NSC_ALIGN_END . = ALIGN(CONFIG_NRF_SPU_FLASH_REGION_SIZE)

SECTIONS
{
  .gnu.sgstubs :
  {
    NSC_ALIGN;
    __sg_start = .;
    } > FLASH
    __sg_end = .;
    __sg_size = __sg_end - __sg_start;
    NSC_ALIGN_END;
    __nsc_size = . - __sg_start;
} INSERT AFTER .rodata;

#define NRF_SG_START (__sg_start % CONFIG_NRF_SPU_FLASH_REGION_SIZE)
#define NRF_SG_SIZE (CONFIG_NRF_SPU_FLASH_REGION_SIZE - NRF_SG_START)
ASSERT((__sg_size == 0)
    || (((1 << LOG2CEIL(NRF_SG_SIZE)) == NRF_SG_SIZE) /* Pow of 2 */
        && (NRF_SG_SIZE >= 32)
        && (NRF_SG_SIZE <= 4096)),
    "The Non-Secure Callable region size must be a power of 2 \
between 32 and 4096 bytes.")
