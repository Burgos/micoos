SECTIONS
{
    . = 0x0;
    __rodata_start = .;
    .rodata :
    {
        *(.rodata)
    }
	__rodata_end = .;

	/* starts at LOADER_ADDR. */
	. = 0x8000;
	__start = .;
    __text_start = .;

	.text :
	{
		KEEP(*(.text.boot))
		*(.text)
	}

	. = ALIGN(4096); /* align to page size */
	__text_end = .;
 
	__data_start = .;
	.data :
	{
	*(.data)
	}
	. = ALIGN(4096); /* align to page size */
	__data_end = .;
 
	__bss_start = .;
	.bss :
	{
	bss = .;
	*(.bss)
	}
	. = ALIGN(4096); /* align to page size */
	__bss_end = .;
	__end = .;


    . = 0x10140110;
    timer0_iv = .;
}
