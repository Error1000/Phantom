#!/bin/bash
rm a.out
riscv64-elf-gcc -Ofast -mstrict-align -ffreestanding -nostdlib -nolibc -nostartfiles -nodefaultlibs -march=rv32e -mabi=ilp32e -o a.out "$1"
if [ "$?" -eq "0" ]; then
	../trans-risc.sh a.out
fi
