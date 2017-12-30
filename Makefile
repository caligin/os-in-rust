.PHONY: all run clean

all: os.iso

%.o: %.asm
	nasm -f elf64 $<

isofiles/boot/kernel.bin: linker.ld multiboot_header.o boot.o
	ld -n -o $@ -T linker.ld multiboot_header.o boot.o

os.iso: isofiles/boot/kernel.bin isofiles/boot/grub/grub.cfg
	grub-mkrescue /usr/lib/grub/i386-pc -o os.iso isofiles

run: os.iso
	qemu-system-x86_64 -cdrom os.iso

clean:
	rm *.o os.iso isofiles/boot/kernel.bin