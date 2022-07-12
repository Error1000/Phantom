#!/bin/bash
if [ -z "$XLEN" ]; then
	XLEN=32
fi
echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
riscv$XLEN-elf-objdump -d "$1" -M numeric
echo "-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-"
riscv$XLEN-elf-objdump -s "$1"
riscv$XLEN-elf-objcopy --reverse-bytes=4 --output-target binary -j .text -j .rodata -j .data "$1" /tmp/res.bin
