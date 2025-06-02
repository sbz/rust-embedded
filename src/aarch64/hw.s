/*
 * hw.s - aarch64 assembly hello world with as
 */

.data

buf:
  .ascii  "Hello AARCH64!\n"
size = . - buf
exit_status = 42

.text

.globl _start
_start:
        /* write(1, buf, sizeof(buf)) */
        mov x0, #1
        ldr x1, =buf
        ldr x2, =size
        mov w8, #0x40
        svc #0

        /* exit(exit_status) */
        ldr x0, =exit_status
        mov w8, #0x5d
        svc #0
