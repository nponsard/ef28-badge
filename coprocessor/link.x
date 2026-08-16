/*
 * SPDX-FileCopyrightText: 2022 Espressif Systems (Shanghai) CO LTD
 *
 * SPDX-License-Identifier: Apache-2.0
 */

ENTRY(reset_vector)

CONFIG_ULP_COPROC_RESERVE_MEM = 8 * 1024;

SHARED_LENGTH = 32;
SHARED_START = CONFIG_ULP_COPROC_RESERVE_MEM - SHARED_LENGTH;
__shared_length = SHARED_LENGTH;
__shared_start = SHARED_START;

MEMORY
{
    ram(RW) : ORIGIN = 0, LENGTH = CONFIG_ULP_COPROC_RESERVE_MEM - SHARED_LENGTH
    shared(RW) : ORIGIN = SHARED_START, LENGTH = SHARED_LENGTH
}

SECTIONS
{
    . = ORIGIN(ram);

    .text :
    {
        *(.text.vectors) /* Default reset vector must link to offset 0x0 */

        KEEP(*(.init));
        KEEP(*(.init.rust));
        *(.text)
        *(.text*)
    } >ram

    .rodata ALIGN(4):
    {
        *(.rodata)
        *(.rodata*)
    } > ram

    .data ALIGN(4):
    {
        *(.data)
        *(.data*)
        *(.sdata)
        *(.sdata*)
    } > ram

    .bss ALIGN(4) :
    {
        *(.bss)
        *(.bss*)
        *(.sbss)
        *(.sbss*)
    } >ram

    __stack_top = ORIGIN(ram) + LENGTH(ram);

}
