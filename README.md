# MxRox

Mixray's small x86_64 OS

## How to build

```bash
make clean      # removes target/, build/
make build      # builds iso image (build/mxrox.iso)
make run        # runs iso image in QEMU emulator
make run-kernel # runs only kernel in QEMU emulator
```

## Roadmap

- [x] Hello World
- [ ] Keyboard & Mouse (PS/2)
- [ ] ACPI
- [ ] Threads
- [ ] File systems (FAT32)
- [ ] Disk management
- [ ] Executable files
- [ ] Basic shell
- [ ] Internet
- [ ] Time
- [ ] Video graphics
- [ ] Audio

### Resources

Internet resources where I found most information about OS dev

- https://github.com/cirosantilli/x86-bare-metal-examples/tree/master/multiboot/hello-world
- http://wiki.osdev.org/Bare_Bones
- https://gitea.bedohswe.eu.org/bedohswe/bootsector_tictactoe/
- https://wiki.osdev.org/PS/2
- https://os.phil-opp.com/
- https://wiki.osdev.org/Interrupts_Tutorial

### Contributing

If you would like to contribute to the project, feel free to fork the repository and submit a pull request.

### License
This project is licensed under the WTFPL License
