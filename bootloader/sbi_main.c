
#include "include/csr.h"

#define FW_JUMP_ADDR 0x80200000

/*
 *  运行在 M 模式，切换到 S 模式
 */

void sbi_start(void)
{
    unsigned long val;

    val = read_csr(mstatus);
    val = INSERT_FIELD(val, MSTATUS_MPP, PRV_S);
    val = INSERT_FIELD(val, MSTATUS_MPIE, 0);
    write_csr(mstatus, val);

    /* 设置 M 模式的异常程序计数器，用于 mret 跳转 */
    write_csr(mepc, FW_JUMP_ADDR);
    /* 设置 S 模式的异常向量表入口地址 */
    write_csr(stvec, FW_JUMP_ADDR);
    /* 关闭 S 模式中断 */
    write_csr(sie, 0);
    /* 关闭 S 模式的页表转换 */
    write_csr(satp, 0);

    /* 切换到 S 模式 */
    asm volatile("mret");

}