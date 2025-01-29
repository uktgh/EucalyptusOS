ASM = nasm
ASMFLAGS = -f elf32
LDFLAGS = -m elf_i386 -T linker/kernel.ld

BUILD_DIR = build
OBJ_DIR = $(BUILD_DIR)/objs
IMG_DIR = $(BUILD_DIR)/imgs

KERNEL_DIR = kernel
DRIVERS_DIR = $(KERNEL_DIR)/drivers
FS_DIR = $(KERNEL_DIR)/fs
INCLUDE_DIR = $(KERNEL_DIR)/inc

SOURCES = $(wildcard $(KERNEL_DIR)/*.asm) \
          $(wildcard $(DRIVERS_DIR)/*.asm) \
          $(wildcard $(FS_DIR)/*.asm)

OBJECTS = $(patsubst %.asm,$(OBJ_DIR)/%.o,$(notdir $(SOURCES)))

.PHONY: all clean directories

all: directories $(IMG_DIR)/os.img

run: build/imgs/os.img
	qemu-system-i386 \
	-nographic \
	-drive file=$(IMG_DIR)/os.img,format=raw

directories:
	mkdir -p $(BUILD_DIR) $(OBJ_DIR) $(IMG_DIR)

$(OBJ_DIR)/%.o: $(KERNEL_DIR)/%.asm
	$(ASM) $(ASMFLAGS) -i $(INCLUDE_DIR)/ $< -o $@

$(OBJ_DIR)/%.o: $(DRIVERS_DIR)/%.asm
	$(ASM) $(ASMFLAGS) -i $(INCLUDE_DIR)/ $< -o $@

$(OBJ_DIR)/%.o: $(FS_DIR)/%.asm
	$(ASM) $(ASMFLAGS) -i $(INCLUDE_DIR)/ $< -o $@

$(IMG_DIR)/os.img: $(OBJECTS) boot/boot.asm
	$(ASM) -f bin boot/boot.asm -o $@
	cat $(OBJECTS) >> $@

clean:
	rm -rf $(BUILD_DIR)