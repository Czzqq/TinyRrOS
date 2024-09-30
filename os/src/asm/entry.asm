/*
 * do_exception_vector must align 4 bytes
 */
.align 2
.global do_exception_vector
do_exception_vector:
    call save_s_context

    la ra, do_exception_vector

    /* PtRegs */
    mv a0, sp
    mv a1, s4
    tail do_exception

ret_from_exception:
recover_all:
    call recover_s_context
    sret
