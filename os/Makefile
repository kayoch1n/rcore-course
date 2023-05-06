TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/rcore
KERNEL_BIN := $(KERNEL_ELF).bin

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

KERNEL_ENTRY := 0x80200000

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

QEMU_ARGS := -machine virt \
	-vnc :0 \
	-nographic \
	-bios $(BOOTLOADER) \
	-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY)

build: env $(KERNEL_BIN)

env:
	@echo Install dependencies
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add llvm-tools-preview
	rustup component add rust-src

$(KERNEL_BIN): kernel
	@echo Strip $(KERNEL_ELF)
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Build user apps
	@cd ../user && make clean && make build
	@echo Build rcore
	@cargo build $(MODE_ARG)

clean:
	@cargo clean

# type ctrl+a c in terminal
# QEMU 参数列表 https://www.qemu.org/docs/master/system/invocation.html
# 选择仿真机器为virt，启动一个VNC server，使用 rustsbi-qemu 作为 BIOS firmware
# 声明一个generic loader 作为 device，装载地址为 0x80200000

# 查看 loader 的 options qemu-system-riscv64 -device loader,help
run:
	@qemu-system-riscv64 $(QEMU_ARGS)

gdbserver:
	@echo listening for gdb client
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb \
		-ex "file $(KERNEL_ELF)" \
		-ex "set arch riscv:rv64" \
		-ex "target remote localhost:1234"

nm:
	@nm $(KERNEL_ELF) -n --demangle
