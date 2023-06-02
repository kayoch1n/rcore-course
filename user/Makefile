TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
# 获取所有 src/bin 下面所有的 *.rs 列表
APPS := $(wildcard $(APP_DIR)/*.rs)

APP_NAMES := $(patsubst $(APP_DIR)/%.rs, %, $(APPS))
# elfs的名字列表
# ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
# .bin的名字列表
# BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

# OBJDUMP := rust-objdump --arch-name=riscv64
# OBJCOPY := rust-objcopy --binary-architecture=riscv64

build:
	@$(foreach app, $(APP_NAMES), cargo build --bin $(app) --release;)

clean:
	@cargo clean