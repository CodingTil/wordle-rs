#!/bin/bash

# Set option to exit on any error
set -e

# Get Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the cargo env
source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Get cargo binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Get trunk
cargo binstall trunk -y

# Get wasm-opt
# Get latest wasm-opt (Binaryen) locally, since crates.io version is outdated
# renovate: datasource=github-releases depName=WebAssembly/binaryen versioning=regex:^version_(?<major>\d+)$
WASM_OPT_VERSION=version_124
curl -L -o binaryen.tar.gz "https://github.com/WebAssembly/binaryen/releases/download/${WASM_OPT_VERSION}/binaryen-${WASM_OPT_VERSION}-x86_64-linux.tar.gz"
tar -xzf binaryen.tar.gz
mv binaryen-${WASM_OPT_VERSION}/bin/wasm-opt ./wasm-opt
chmod +x ./wasm-opt
export PATH="$(pwd):$PATH"

# Clean the project
cd wordle_web
trunk clean
cargo clean

# Build the project
trunk build --release --no-sri

# Find all .wasm files in the dist directory and subdirectories
find dist/ -name "*.wasm" | while read wasm_file; do
    # Execute wasm-opt command on each file
    wasm-opt -Oz -o "$wasm_file" "$wasm_file"
done
