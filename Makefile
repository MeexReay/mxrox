.PHONY: clean run run-iso target_dir

target/main.iso: target/kernel.elf target_dir
	cp 'src/grub.cfg' target/iso/boot/grub
	cp '$<' target/iso/boot
	grub-mkrescue -o '$@' target/iso

target/boot.o: src/boot.s target_dir
	nasm -f elf32 '$<' -o '$@'

target/kernel.elf: src/linker.ld target/boot.o target/kernel.o
	i686-elf-ld -m elf_i386 -nostdlib -o '$@' -T $^

target/kernel.o: kernel/ target_dir
	cd kernel && cargo build --release
	cp kernel/target/x86-unknown-bare_metal/release/deps/kernel-*.o target/kernel.o

target_dir:
	mkdir -p target
	mkdir -p target/iso
	mkdir -p target/iso/lib
	mkdir -p target/iso/boot
	mkdir -p target/iso/boot/grub

build: target/main.iso

clean:
	rm -rf target
	rm -rf kernel/target
	rm -rf kernel/Cargo.lock
	mkdir target

run-kernel: target/kernel.elf
	qemu-system-i386 -kernel '$<'

run: target/main.iso
	qemu-system-i386 -cdrom '$<'