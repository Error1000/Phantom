#!/bin/bash
riscv64-elf-as -mabi=ilp32e -march=rv32e $1 -o a.out
../trans-risc.sh a.out
