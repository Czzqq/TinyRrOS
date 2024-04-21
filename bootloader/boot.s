# 定义常量
.equ STACK_SIZE, 4096

# 定义全局符号
.global _start

# text 段
.section .text
_start:
    # 重置栈指针
    la sp, stack_top

    /*
     *  ...
     */
    la a0, os_entru
    jr a0

loop:
    j loop

# data 段
.section .data

.section .bss
.align 4
stack_bottom:
    .skip STACK_SIZE
stack_top: