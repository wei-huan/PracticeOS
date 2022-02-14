
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 17
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_9_start
    .quad app_10_start
    .quad app_11_start
    .quad app_12_start
    .quad app_13_start
    .quad app_14_start
    .quad app_15_start
    .quad app_16_start
    .quad app_16_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "../user/build/bin/power_3.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "../user/build/bin/power_5.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "../user/build/bin/power_7.bin"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "../user/build/bin/sleep.bin"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
app_4_start:
    .incbin "../user/build/bin/sleep1.bin"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
app_5_start:
    .incbin "../user/build/bin/test1_write0.bin"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
app_6_start:
    .incbin "../user/build/bin/test1_write1.bin"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
app_7_start:
    .incbin "../user/build/bin/test2_setprio.bin"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
app_8_start:
    .incbin "../user/build/bin/test3_stride0.bin"
app_8_end:

    .section .data
    .global app_9_start
    .global app_9_end
app_9_start:
    .incbin "../user/build/bin/test3_stride1.bin"
app_9_end:

    .section .data
    .global app_10_start
    .global app_10_end
app_10_start:
    .incbin "../user/build/bin/test3_stride2.bin"
app_10_end:

    .section .data
    .global app_11_start
    .global app_11_end
app_11_start:
    .incbin "../user/build/bin/test3_stride3.bin"
app_11_end:

    .section .data
    .global app_12_start
    .global app_12_end
app_12_start:
    .incbin "../user/build/bin/test3_stride4.bin"
app_12_end:

    .section .data
    .global app_13_start
    .global app_13_end
app_13_start:
    .incbin "../user/build/bin/test3_stride5.bin"
app_13_end:

    .section .data
    .global app_14_start
    .global app_14_end
app_14_start:
    .incbin "../user/build/bin/yield0.bin"
app_14_end:

    .section .data
    .global app_15_start
    .global app_15_end
app_15_start:
    .incbin "../user/build/bin/yield1.bin"
app_15_end:

    .section .data
    .global app_16_start
    .global app_16_end
app_16_start:
    .incbin "../user/build/bin/yield2.bin"
app_16_end:
