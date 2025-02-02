.PHONY: clean run run-iso build-dir clean-build

TARGET_DIR = target/x86-unknown-bare_metal/release

build/mxrox.iso: build/kernel.elf build-dir
	cp 'grub.cfg' build/iso/boot/grub
	cp '$<' build/iso/boot
	grub-mkrescue -o '$@' build/iso

build/boot.o: boot.s build-dir
	nasm -f elf32 '$<' -o '$@'

build/kernel.elf: linker.ld build/boot.o build/libkernel.a
	i686-elf-gcc -nostdlib -o $@ -T $^

build/libkernel.a: build-dir
	rm -rf $(TARGET_DIR)
	rustup override set nightly
	cargo build --release
	cp $(TARGET_DIR)/libmxrox.a build/libkernel.a

build-dir:
	mkdir -p build
	mkdir -p build/iso
	mkdir -p build/iso/boot
	mkdir -p build/iso/boot/grub

build: build/mxrox.iso

clean-build:
	rm -rf build
	mkdir build

clean: clean-build
	cargo clean

run-kernel: build/kernel.elf
	qemu-system-i386 -kernel '$<' -m 512M

run: build/mxrox.iso
	qemu-system-i386 -cdrom '$<' -m 512M