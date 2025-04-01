#!/bin/bash

set -e

rustup override unset

rustup show active-toolchain

echo "Cleaning previous build..."
cargo clean

echo "Building project..."
cargo build --release
