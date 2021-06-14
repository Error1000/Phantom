#!/bin/bash
# riscv64-elf-objdump -d "$1" -M numeric
riscv64-unknown-elf-objcopy -O binary -j .text "$1" /tmp/res.bin
