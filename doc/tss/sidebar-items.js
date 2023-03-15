window.SIDEBAR_ITEMS = {"constant":[["DOUBLE_FAULT_IST_INDEX","The index of the double fault stack in a TaskStateSegment (TSS)"]],"fn":[["create_tss","Sets up TSS entry for the given CPU core. "],["tss_set_rsp0","Sets the current CPU’s TSS privilege stack 0 (RSP0) entry, which points to the stack that  the x86_64 hardware automatically switches to when transitioning from Ring 3 -> Ring 0. Should be set to an address within the current userspace task’s kernel stack. WARNING: If set incorrectly, the OS will crash upon an interrupt from userspace into kernel space!!"]]};