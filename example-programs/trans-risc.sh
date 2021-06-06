#!/bin/bash
riscv64-elf-objcopy -O binary -j .text /tmp/out.obj /tmp/res.bin
riscv64-elf-objdump -d /tmp/out.obj -M numeric
