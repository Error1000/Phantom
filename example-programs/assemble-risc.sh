#!/bin/bash
riscv64-elf-as -mabi=ilp32e -march=rv32e $1 -o /tmp/out.obj
./trans-risc.sh
