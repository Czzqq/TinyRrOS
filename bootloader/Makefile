GNU ?= riscv64-linux-gnu

COPS += -save-temps=obj -g -O0 -Wall -nostdlib -nostdinc -Iinclude -mcmodel=medany -mabi=lp64 -march=rv64imafd -fno-PIE -fomit-frame-pointer
COPS += -DCONFIG_BOARD_QEMU

all : clean mysbi.bin

# Check if verbosity is ON for build process
CMD_PREFIX_DEFAULT := @
ifeq ($(V), 1)
	CMD_PREFIX :=
else
	CMD_PREFIX := $(CMD_PREFIX_DEFAULT)
endif

clean :
	rm -rf $(BUILD_DIR) $(SBI_BUILD_DIR) *.bin  *.map *.elf

##############
#  build SBI
##############
SBI_BUILD_DIR = build_sbi
SBI_SRC_DIR = .
$(SBI_BUILD_DIR)/%_c.o: $(SBI_SRC_DIR)/%.c
	$(CMD_PREFIX)mkdir -p $(SBI_BUILD_DIR); echo " CC  $@" ; $(GNU)-gcc $(COPS) -c $< -o $@

$(SBI_BUILD_DIR)/%_s.o: $(SBI_SRC_DIR)/%.S
	$(CMD_PREFIX)mkdir -p $(SBI_BUILD_DIR); echo " AS  $@" ; $(GNU)-gcc $(COPS) -c $< -o $@

SBI_C_FILES = $(wildcard $(SBI_SRC_DIR)/*.c)
SBI_ASM_FILES = $(wildcard $(SBI_SRC_DIR)/*.S)
SBI_OBJ_FILES = $(patsubst $(SBI_SRC_DIR)/%.c,$(SBI_BUILD_DIR)/%_c.o,$(SBI_C_FILES))
SBI_OBJ_FILES += $(patsubst $(SBI_SRC_DIR)/%.S,$(SBI_BUILD_DIR)/%_s.o,$(SBI_ASM_FILES))

# DEP_FILES = $(SBI_OBJ_FILES):%.o=%.d)
# -include $(DEP_FILES)

mysbi.bin: sbi_linker.ld $(SBI_OBJ_FILES)
	$(CMD_PREFIX)$(GNU)-ld -T sbi_linker.ld -o $(SBI_BUILD_DIR)/mysbi.elf  $(SBI_OBJ_FILES) -Map mysbi.map; echo " LD $(SBI_BUILD_DIR)/mysbi.elf"
	$(CMD_PREFIX)$(GNU)-objcopy $(SBI_BUILD_DIR)/mysbi.elf -O binary mysbi.bin; echo " OBJCOPY mysbi.bin"
	$(CMD_PREFIX)cp $(SBI_BUILD_DIR)/mysbi.elf mysbi.elf
	$(CMD_PREFIX)cp mysbi.elf $(O)/bootloader
