#!/bin/sh
git clone https://github.com/rust-lang/rust.git
cd rust
git submodule update --init --recursive
cp ../config.toml .
git apply ../rust-patch.diff
cd src/llvm-project
git apply ../../../llvm-patch.diff
cd ../..
./x.py build compiler library/core src/llvm-project src/llvm-project/lld
./x.py build --stage 2 library/std 
./x.py build --stage 2 library/core
ln -s $(pwd)/build/x86_64-unknown-linux-gnu/lld/bin/lld     $(pwd)/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld
rustup toolchain remove stage2
rustup toolchain link stage2 build/x86_64-unknown-linux-gnu/stage2
rustup default stage2
