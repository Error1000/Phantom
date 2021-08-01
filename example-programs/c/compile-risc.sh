#!/bin/bash
rm a.out
rm ld.script
# For putting _start at the beginning
touch ld.script
echo "ENTRY(_start)" >> ld.script
echo "SECTIONS" >> ld.script
echo "{" >> ld.script
echo "  .text :" >> ld.script
echo "  {" >> ld.script
echo "   *(.text._start);" >> ld.script
echo "   *(.text*);" >> ld.script
echo "  }" >> ld.script
echo "}" >> ld.script

# Bare metal options
# No deps options
# Linker options
# Arch options and output
riscv64-unknown-elf-gcc -Os -ffreestanding -nostartfiles \
-nostdlib -nolibc -nodefaultlibs \
-ffunction-sections -Wl,-gc-sections -Wl,-Tld.script \
-march=rv32e -mabi=ilp32e -o a.out "$@"
../trans-risc.sh a.out

