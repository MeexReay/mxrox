.PHONY: clean run run-iso bin_dir

bin/main.iso: bin/kernel.elf bin_dir
	cp 'grub.cfg' bin/iso/boot/grub
	cp '$<' bin/iso/boot
	grub-mkrescue -o '$@' bin/iso

bin/boot.o: boot.s bin_dir
	nasm -f elf32 '$<' -o '$@'

bin/kernel.elf: linker.ld bin/boot.o bin/kernel.o
	i686-elf-ld -m elf_i386 -nostdlib -o '$@' -T $^

bin/kernel.o: bin_dir
	rustup override set nightly
	cargo build --release
	cp target/x86-unknown-bare_metal/release/deps/mxrox_kernel-*.o $@

bin_dir:
	mkdir -p bin
	mkdir -p bin/iso
	mkdir -p bin/iso/lib
	mkdir -p bin/iso/boot
	mkdir -p bin/iso/boot/grub

build: bin/main.iso

clean:
	rm -rf bin
	rm -rf target
	rm -rf Cargo.lock
	mkdir bin

run-kernel: bin/kernel.elf
	qemu-system-i386 -kernel '$<'

run: bin/main.iso
	qemu-system-i386 -cdrom '$<'