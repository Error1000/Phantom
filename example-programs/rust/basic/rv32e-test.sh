#!/bin/bash
riscv32-elf-objdump -d $1 -M numeric | \grep -oPn "[ ,(]x[0-9][0-9]?" | tr -d ",()\tx " | cut -d':' -f2 | awk '$1 > 15'

