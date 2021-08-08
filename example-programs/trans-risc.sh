#!/bin/bash
echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
riscv64-elf-objdump -d "$1" -M numeric
echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
riscv64-elf-objdump -s "$1"
riscv64-elf-objcopy --reverse-bytes=4 -O binary -j .text -j .rodata -j .data "$1" /tmp/res.bin
