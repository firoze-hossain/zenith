#!/bin/sh
# Builds zenith for NovaOS (i686 ring-3 ELF32 executable)
# and installs it into NovaOS fixtures as ZENITH.ELF, OFFICE.ELF, WRITER.ELF, CALC.ELF, PRESENT.ELF,
# and generates ZENITH.PKG and OFFICE.PKG.
set -e
cd "$(dirname "$0")"

NOVAOS_ROOT="${NOVAOS_DIR:-/home/firoze/projects/others/novaos}"
if [ ! -d "$NOVAOS_ROOT" ]; then
    echo "Error: NovaOS directory not found at $NOVAOS_ROOT" >&2
    exit 1
fi

RUST_SYSROOT="$NOVAOS_ROOT/tools/rust-sysroot"
LIBRUST="$RUST_SYSROOT/sysroot/lib/rustlib/i686-novaos/lib"
if [ ! -f "$LIBRUST/libcore.rlib" ]; then
    echo "Building NovaOS Rust sysroot..."
    "$RUST_SYSROOT/build-sysroot.sh"
fi

LIBC="$NOVAOS_ROOT/userland/libc"
CC="gcc -m32 -ffreestanding -fno-stack-protector -fno-pie -Wall -Wextra -I$LIBC/include"

nasm -f elf32 "$LIBC/crt0.asm" -o crt0.o
$CC -c "$LIBC/syscall.c" -o syscall.o
$CC -c "$LIBC/stdlib.c" -o stdlib.o
$CC -c "$LIBC/string.c" -o string.o

echo "Compiling Zenith Office Suite for NovaOS (pure Rust i686)..."
RUSTC_CMD="${RUSTC:-rustc}"
BOOTSTRAP_ENV="RUSTC_BOOTSTRAP=1"

env $BOOTSTRAP_ENV $RUSTC_CMD --edition 2021 -Z unstable-options \
    --target "$RUST_SYSROOT/i686-novaos.json" \
    --crate-type bin -C panic=abort -C opt-level=2 \
    --emit obj=zenith_main.o \
    -L "$LIBRUST" \
    --extern core="$LIBRUST/libcore.rlib" \
    --extern compiler_builtins="$LIBRUST/libcompiler_builtins.rlib" \
    -C link-dead-code=no \
    zenith.rs

ld -m elf_i386 -Ttext=0x08048000 --entry=_start -static \
    -o zenith.elf crt0.o zenith_main.o syscall.o string.o stdlib.o \
    "$LIBRUST/libcore.rlib" "$LIBRUST/libcompiler_builtins.rlib"

mkdir -p "$NOVAOS_ROOT/tools/fixtures"
cp zenith.elf "$NOVAOS_ROOT/tools/fixtures/ZENITH.ELF"
cp zenith.elf "$NOVAOS_ROOT/tools/fixtures/OFFICE.ELF"
cp zenith.elf "$NOVAOS_ROOT/tools/fixtures/WRITER.ELF"
cp zenith.elf "$NOVAOS_ROOT/tools/fixtures/SHEETS.ELF"
cp zenith.elf "$NOVAOS_ROOT/tools/fixtures/PRESENT.ELF"

# Generate ZENITH.PKG and OFFICE.PKG metadata fixtures
cat << 'EOF' > "$NOVAOS_ROOT/tools/fixtures/ZENITH.PKG"
[package]
id = "org.novaos.zenith"
name = "Zenith Office Suite"
version = "1.0.0"
category = "Productivity"
app_filename = "ZENITH.ELF"
summary = "Sovereign pure-Rust office suite: Writer, Calc, Present & Base"
EOF

cp "$NOVAOS_ROOT/tools/fixtures/ZENITH.PKG" "$NOVAOS_ROOT/tools/fixtures/OFFICE.PKG"

echo "✅ Built and installed ZENITH.ELF, OFFICE.ELF, WRITER.ELF, CALC.ELF, PRESENT.ELF and packages into $NOVAOS_ROOT/tools/fixtures/"
