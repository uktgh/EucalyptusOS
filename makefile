BOOTLOADER_DIR := bootloader
KERNEL_DIR := kernel
BUILD_DIR := build

BOOTLOADER_SRC := $(wildcard $(BOOTLOADER_DIR)/*.asm)
KERNEL_SRC := $(wildcard $(KERNEL_DIR)/**/*.rs)

BOOTLOADER_BIN := $(BUILD_DIR)/bootloader.bin
KERNEL_BIN := $(BUILD_DIR)/kernel.bin
OS_IMAGE := $(BUILD_DIR)/os-image.bin

ASM_COMPILER := nasm
ASM_FLAGS := -f bin
RUST_COMPILER := rustc
RUST_FLAGS := --target i386-unknown-none-elf -C opt-level=2 -C panic=abort
LD_SCRIPT := link.ld

all: $(OS_IMAGE)

$(BOOTLOADER_BIN): $(BOOTLOADER_SRC) | $(BUILD_DIR)
    $(ASM_COMPILER) $(ASM_FLAGS) -o $@ $(BOOTLOADER_DIR)/boot.asm

$(KERNEL_BIN): $(KERNEL_SRC) $(LD_SCRIPT) | $(BUILD_DIR)
    $(RUST_COMPILER) $(RUST_FLAGS) --crate-type=staticlib --emit=obj $(KERNEL_DIR)/main.rs -o $@ -C link-args="-T$(LD_SCRIPT)"

$(OS_IMAGE): $(BOOTLOADER_BIN) $(KERNEL_BIN) | $(BUILD_DIR)
    cat $^ > $@

$(BUILD_DIR):
    mkdir -p $@

run: $(OS_IMAGE)
    qemu-system-x86_64 -drive format=raw,file=$(OS_IMAGE)

clean:
    rm -rf $(BUILD_DIR)

.PHONY: all run clean
