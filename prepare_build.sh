#!/bin/bash

version=$(rustc --version | awk '{print $2}')
required_version="1.81.0"

if printf "%s\n%s" "$required_version" "$version" | sort -V | head -n1 | grep -r "$required_version"; then
    echo "rustc is up to date."
else
    if ! command -v rustup &> /dev/null
    then
        echo "ERROR: rustup is not installed..."
        exit 1
    else
        rustup default 1.81
    fi
fi
