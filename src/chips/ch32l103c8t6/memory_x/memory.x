MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH =   64K /* BANK_1 */
    RAM   : ORIGIN = 0x20000000, LENGTH =   20K
}
REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
    