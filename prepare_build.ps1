if ($env:OS -eq "Windows_NT") {
    Copy-Item -Path "rust-toolchain-windows.toml" -Destination "rust-toolchain.toml" -Force
    Write-Output "Using rust-toolchain-windows.toml (channel 1.77.0)"
} else {
    Write-Error "This script is intended for Windows."
    exit 1
}

