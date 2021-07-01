# := 定义变量防止递归展开 只能引用前面定义过的变量
# ex:
# 	A = $(B)
# 	B = $(A)
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/blog_os
KERNEL_BIN := $(KERNEL_ELF).bin

# board
# ?= 仅在变量没有定义或没有值时才赋值
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := bootloader/$(SBI)-$(BOARD).bin

# kernel entry
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA := 0x80200000
else ifeq ($(BOARD), k210)
	KERNEL_ENTRY_PA := 0x80020000
endif

# binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# disassembly
DISASM ?= -d

build: $(KERNEL_BIN)

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(target)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Platform: $(BOARD)
	@cargo build --release

clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

run: run-inner

run-inner: build
ifeq ($(BOARD), qemu)
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)
else
	@echo "unsuport platform now"
endif

debug: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_ELF),addr=$(KERNEL_ENTRY_PA) -S -s

.PHONY: build env kernel clean disasm run-inner
