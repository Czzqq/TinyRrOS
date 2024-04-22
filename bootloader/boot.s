.section ".text.boot"

# 定义全局符号
.global _start
_start:
    # 关闭 M 模式中断
    csrw mie, zero

    # 设置栈， 大小为 4096
    la sp stack_top
    li t0, 4096
    add sp, sp, t0

    tail sbi_start
    
# data 段
.section .data
.align 12
.global stack_top
stack_top:
    .skip 4096