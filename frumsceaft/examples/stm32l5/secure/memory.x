MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
  ROM_NSC : ORIGIN = 0x0803E000, LENGTH = 8K
}

SECTIONS
{
  .gnu.sgstubs : ALIGN(64)
  {
    __sg_start = .;
   *(.gnu.sgstubs*)
  } > ROM_NSC
  __sg_end = .;
} INSERT AFTER .rodata;
