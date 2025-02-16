qemu-system-i386 -m 256M -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -kernel "$1"

exit $(($? >> 1))
