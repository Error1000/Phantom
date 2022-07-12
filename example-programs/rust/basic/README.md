To compile build the riscv32e toolchain
and then simply run: `cargo build`, or `cargo build --release`,
you can then run `../../trans-risc.sh` on the binaries to transform them into raw binary,
adn then `../../run-digital.py` will load the raw binary into the memory of the cpu in digital automatically.

