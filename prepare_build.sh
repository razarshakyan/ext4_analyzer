#!/bin/bash

if [[ "$(uname -s)" == "Linux" ]]; then
    cp rust-toolchain-linux.toml rust-toolchain.toml
    /mnt/c/Users/lyonc/.cargo/bin/rustup.exe override set 1.81.0
else
    echo "ERROR: Script should be called on Linux"
    exit 1
fi
