RUST_SRC := $(shell find src/ -name '*.rs') x86_64-rust_os.json Cargo.toml
BINARIES := long_mode_init.o multiboot_header.o boot.o target/x86_64-rust_os/debug/librust_os.a

.PHONY: all run clean ubuntu-deps

all: os.iso

%.o: %.asm
	nasm -f elf64 $<

isofiles/boot/kernel.bin: linker.ld $(BINARIES)
	ld -n --gc-sections -o $@ -T linker.ld $(BINARIES)

os.iso: isofiles/boot/kernel.bin isofiles/boot/grub/grub.cfg
	grub-mkrescue /usr/lib/grub/i386-pc -o os.iso isofiles

target/x86_64-rust_os/debug/librust_os.a: $(RUST_SRC)
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-rust_os

run: os.iso
	qemu-system-x86_64 -cdrom os.iso

clean:
	rm *.o os.iso isofiles/boot/kernel.bin || true
	xargo clean

ubuntu-deps:
	sudo apt-get install -y nasm qemu grub-pc-bin xorriso
	rustup override add nightly
	rustup component add rust-src
	which xargo || cargo install xargo
