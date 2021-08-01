#!/bin/bash
echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
riscv64-unknown-elf-objdump -d "$1" -M numeric
echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
riscv64-unknown-elf-objdump -s "$1"
riscv64-unknown-elf-objcopy --reverse-bytes=4 -O binary -j .text -j .rodata -j .data "$1" /tmp/res.bin
