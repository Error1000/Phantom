#!/bin/bash
riscv32-elf-as -mabi=ilp32e -march=rv32e $1 -o a.out

