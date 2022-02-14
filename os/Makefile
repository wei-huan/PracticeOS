# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
LOG ?= DEFAULT
TEST ?= 0

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

kernel:
ifeq ($(LOG),INFO)
	@cargo build --release --features "$(LOG)"
else
	@cargo build --release
endif

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

build: $(KERNEL_BIN)

run: build
	qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

debug:
	riscv64-unknown-elf-gdb -ex 'file target/riscv64gc-unknown-none-elf/release/os' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

clean:
	rm -rf target/*

