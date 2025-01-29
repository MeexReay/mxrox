.PHONY: clean run run-iso build_dir

build/mxrox.iso: build/kernel.elf build_dir
	cp 'grub.cfg' build/iso/boot/grub
	cp '$<' build/iso/boot
	grub-mkrescue -o '$@' build/iso

build/boot.o: boot.s build_dir
	nasm -f elf32 '$<' -o '$@'

build/kernel.elf: linker.ld build/boot.o build/kernel.o
	i686-elf-ld -m elf_i386 -o '$@' -T $^

build/kernel.o: build_dir
	rustup override set nightly
	cargo build --release
	cp target/x86-unknown-bare_metal/release/deps/mxrox_kernel-*.o $@

build_dir:
	mkdir -p build
	mkdir -p build/iso
	mkdir -p build/iso/boot
	mkdir -p build/iso/boot/grub

build: build/mxrox.iso

clean:
	rm -rf build
	rm -rf target
#   rm -rf Cargo.lock
	mkdir build

run-kernel: build/kernel.elf
	qemu-system-i386 -kernel '$<'

run: build/mxrox.iso
	qemu-system-i386 -cdrom '$<'