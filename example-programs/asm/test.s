_start:
lui x14, 0x12345 # 20 bits
addi x14, x14, 0x678 # 12 bits
sw x14, 4*8(x0)
lui x14, 0x1
addi x14, x14, 0x2ab
sh x14, 4*9(x0)
hlt: j hlt
