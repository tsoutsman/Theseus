window.SIDEBAR_ITEMS = {"constant":[["ADDRESSABILITY_PER_P4_ENTRY","Value: 512 GiB."],["BYTES_PER_ADDR","64-bit architecture results in 8 bytes per address."],["ENTRIES_PER_PAGE_TABLE","Value: 512."],["KERNEL_HEAP_INITIAL_SIZE",""],["KERNEL_HEAP_MAX_SIZE","The kernel heap is allowed to grow to fill the entirety of its P4 entry."],["KERNEL_HEAP_P4_INDEX","Value: 509. The 509th entry is used for the kernel heap."],["KERNEL_HEAP_START","The higher-half heap gets the 512GB address range starting at the 509th P4 entry, which is the slot right below the recursive P4 entry (510). Actual value: 0o177777_775_000_000_000_0000, or 0xFFFF_FE80_0000_0000"],["KERNEL_OFFSET","The virtual address where the initial kernel (the nano_core) is mapped to. Actual value: 0xFFFFFFFF80000000 on x86_64. i.e., the linear offset between physical memory and kernel memory. So, for example, the VGA buffer will be mapped from 0xb8000 to 0xFFFFFFFF800b8000 (on x86_64). This is -2GiB from the end of the 64-bit address space."],["KERNEL_STACK_SIZE_IN_PAGES",""],["KERNEL_TEXT_P4_INDEX","Value: 511. The 511th entry is used (in part) for kernel text sections."],["KERNEL_TEXT_START","The kernel text region is where we load kernel modules.  It starts at the 511th P4 entry and goes up until the KERNEL_OFFSET, which is where the nano_core itself starts.  Actual value on x86_64: 0o177777_777_000_000_000_0000, or 0xFFFF_FF80_0000_0000"],["MAX_PAGE_NUMBER",""],["MAX_VIRTUAL_ADDRESS",""],["P1_INDEX_SHIFT","Value: 0. Shift the Page number (not the address!) by this to get the P1 index."],["P2_INDEX_SHIFT","Value: 9. Shift the Page number (not the address!) by this to get the P2 index."],["P3_INDEX_SHIFT","Value: 18. Shift the Page number (not the address!) by this to get the P3 index."],["P4_INDEX_SHIFT","Value: 27. Shift the Page number (not the address!) by this to get the P4 index."],["PAGE_SHIFT","The lower 12 bits of a virtual address correspond to the P1 page frame offset. "],["PAGE_SIZE","Page size is 4096 bytes, 4KiB pages."],["RECURSIVE_P4_INDEX","Value: 510. The 510th entry is used to recursively map the current P4 root page table frame such that it can be accessed and modified just like any other level of page table."],["RECURSIVE_P4_START","The start of the virtual address range covered by the 510th P4 entry, i.e., [`RECURSIVE_P4_INDEX`];"],["TEMPORARY_PAGE_VIRT_ADDR",""],["UPCOMING_PAGE_TABLE_RECURSIVE_P4_INDEX","Value: 508. The 508th entry is used to temporarily recursively map the P4 root page table frame of an upcoming (new) page table such that it can be accessed and modified."],["UPCOMING_PAGE_TABLE_RECURSIVE_P4_START","The start of the virtual address range covered by the 508th P4 entry, i.e., [`UPCOMING_PAGE_TABLE_RECURSIVE_P4_INDEX`];"]]};