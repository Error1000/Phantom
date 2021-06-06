#!/bin/bash
riscv64-elf-gcc -Ofast -nostdlib -nostartfiles -march=rv32e -mabi=ilp32e -o /tmp/out.obj "$1"
./trans-risc.sh
