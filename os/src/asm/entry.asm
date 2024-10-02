/*
 * do_exception_vector must align 4 bytes
 */
.align 2
.global do_exception_vector
do_exception_vector:
    call save_s_context

    /* set return from exception symbol address */
    la ra, ret_from_exception

    /* PtRegs */
    mv a0, sp
    mv a1, s4
    tail do_exception

ret_from_exception:
recover_all:
    call recover_s_context
    sret

.global trigger_fault
trigger_fault:
    li a0, 0x70000
    ld a0, (a0)
    ret
