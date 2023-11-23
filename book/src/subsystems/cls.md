# CPU local-storage

Operating systems often need to track resources on a per-CPU basis. {some more motivation}

Our goal is to have a simple API, akin to thread locals:

```rust
#[cpu_local]
static PREEMPTION_COUNTER: u8 = 0;

fn increment() {
    PREEMPTION_COUNTER.increment();
}

fn bar() {
    PREEMPTION_COUNTER.decrement();
}
```

{Talk about naive solution using arrays.}


```rust
static CPU_LOCAL: [u8; MAX_CPUS] = [0; MAX_CPUS];

fn increment() {
    let cpu_id = current_cpu();
    CPU_LOCAL[cpu_id] += 1;
}
```

{Have to disable preemption}.

```rust
static CPU_LOCAL: [u8; MAX_CPUS] = [0; MAX_CPUS];

fn increment() {
    let guard = hold_preemption();
    let cpu_id = current_cpu();
    CPU_LOCAL[cpu_id] += 1;
    drop(guard);
}
```

{This is better, but still not perfect}. Take the following code:

```rust
#[derive(Copy, Clone)]
struct NotSync {
    inner: u8,
}

impl !Sync for NotSync {}

static CPU_LOCAL: [NotSync; MAX_CPUS] = [NotSync { inner: 0 }; MAX_CPUS];
```

This should be allowed, and yet its not.

CPU locals implemented using arrays are also not cache friendly. To avoid [false
sharing] the elements of the array must be padded and aligned to cache lines.

However, this means every CPU-local, regardless of size will take up at least a
cache line of size (i.e. 64 bytes)

{CPU-local arrays also limit the max number of CPUs}

{Accessing CPU is slow?}
{https://0xax.gitbooks.io/linux-insides/content/Concepts/linux-cpu-1.html}

TODO: Why else are arrays bad.

Explain fs/gs, tpidr_el0/tpidr_el1

```rust
#[thread_local]
#[link_section = ".cls"]
#[used]
static PREEMPTION_COUNT: CpuLocalPreemptionCount = CpuLocalPreemptionCount {
    __inner: 0,
};

#[repr(transparent)]
#[doc(hidden)]
struct CpuLocalPreemptionCount {
    __inner: u8,
}

impl CpuLocalPreemptionCount {
    #[inline]
    pub fn fetch_add(&self, mut operand: u8) -> u8 {
        // ...
    }

    // More methods.
}
```

So the Rust compiler treats CPU-local storage as thread-local storage, but
placed in a different section to actual TLS variables.

TODO: Write about elf-cls fixups.

# x86-64

The generated code for the x86-64 architecture looks something like

```rust
pub fn replace(&self, value: T) -> T {
    let offset = {
        extern "C" {
            static __THESEUS_CLS_SIZE: u8;
            static __THESEUS_TLS_SIZE: u8;
        }

        let cls_size = unsafe { ptr::addr_of!(__THESEUS_CLS_SIZE) } as u64;
        let tls_size = unsafe { ptr::addr_of!(__THESEUS_TLS_SIZE) } as u64;

        if cls_size == u64::MAX && tls_size == u64::MAX {
            let offset: u64;
            unsafe {
                asm!(
                    "lea {offset}, [{cls}@TPOFF]",
                    offset = out(reg) offset,
                    cls = sym #name,
                    options(nomem, preserves_flags, nostack),
                )
            };
            offset
        }
        else {
            if tls_size == 0 {
                let offset: u64;
                unsafe {
                    asm!(
                        "lea {offset}, [{cls}@TPOFF]",
                        offset = out(reg) offset,
                        cls = sym PREEMPTION_COUNT,
                        options(nomem, preserves_flags, nostack),
                    )
                };
                offset
            } else {
                // The linker script aligns sections to page boundaries.
                const ALIGNMENT: u64 = 1 << 12;
                let cls_start_to_tls_start = (cls_size + ALIGNMENT - 1) & !(ALIGNMENT - 1);

                let from_cls_start: u64;
                unsafe {
                    asm!(
                        "lea {from_cls_start}, [{cls}@TPOFF + {tls_size} + {cls_start_to_tls_start}]",
                        from_cls_start = lateout(reg) from_cls_start,
                        cls = sym STATIC_NAME,
                        tls_size = in(reg) tls_size,
                        cls_start_to_tls_start = in(reg) cls_start_to_tls_start,
                        options(nomem, preserves_flags, nostack),
                    )
                };
                let offset = (cls_size - from_cls_start).wrapping_neg();
                offset
            }
        }
        offset
    };

    let mut ptr = {
        let gs = GS::read_base().as_u64();

        // If this CLS section was statically linked, its `offset` will be negative.
        let (value, _) = gs.overflowing_add(offset);

        value
    };

    let rref = unsafe { &mut *ptr };

    mem::swap(rref, &mut value);
    value
}
```

The function is composed of two main parts:
- Calculating the offset
- Performing the swap

To calculate the offset, we first get the address of two statics called
`__THESEUS_CLS_SIZE`, and `__THESEUS_TLS_SIZE`. In reality, these aren't
actually statics, but linker variables, set to the size of the `.cls` and `.tls`
sections respectively. When statically linking `nano_core`, they are set in the
[linker script][linker-script-variables], and when dynamically linking (i.e.
loadable mode) they are set by `mod_mgmt`. We can abuse this by setting both
variables to `u64::MAX` in `mod_mgmt`, and so `cls_size == u64::MAX && tls_size
== u64::MAX` is just checking whether or not the crate is dynamically linked.

After we calculate the offset, the rest of the function is simple: we add the
offset to the value in `gs`, giving us a pointer to the data, and then we can
{do whatever we want}.

## Static linking

Static linking is a lot more complicated, because we are limited by the linker,
which is unaware of CPU-local semantics. {Include explanation and graphic from
cls docs}.

## Dynamic linking

If both values are set to `u64::MAX`, the crate is being loaded by `mod_mgmt`,
and so we can assume that the offset provided by `mod_mgmt` (i.e. `{cls}@TPOFF`)
will be correct. The `lea` instruction is just a fancy way of moving the offset
into the `offset` variable. {We basically do the same calculations but in `mod_mgmt`
rather than `cls_macros`}.


# AArch64

```rust
pub fn replace(&self, value: T) -> T {
    let offset = {
        extern "C" {
            static __THESEUS_TLS_SIZE: u8;
        }

        let tls_size = unsafe { ptr::addr_of!(__THESEUS_TLS_SIZE) } as u64;

        const ALIGNMENT: u64 = 1 << 12;
        let tls_start_to_cls_start = (tls_size + ALIGNMENT - 1) & !(ALIGNMENT - 1);

        let preemption_guard = hold_preemption();

        let mut offset = 0;
        unsafe {
            asm!(
                "add {offset}, {offset}, #:tprel_hi12:{cls}, lsl #12",
                "add {offset}, {offset}, #:tprel_lo12_nc:{cls}",
                "sub {offset}, {offset}, {tls_start_to_cls_start}",
                offset = inout(reg) offset,
                cls = sym STATIC_NAME,
                tls_start_to_cls_start = in(reg) tls_start_to_cls_start,
                options(nomem, preserves_flags, nostack),
            )
        };
        offset
    };

    let tpidr_el1 = cortex_a::registers::TPIDR_EL1.get();
    let ptr = tpidr_el1 + offset as *mut T;

    let rref = unsafe { &mut *ptr };

    mem::swap(rref, &mut value);
    value
}
```

`__THESEUS_CLS_SIZE` and `__THESEUS_TLS_SIZE` are variables that are defined in
the linker script for static linking, and by `mod_mgmt` for dynamic linking.

{explain why we don't have separate code paths for static and dynamic linking
i.e. AArch64 doesn't have negative offsets}.

# Preemption counter

But the preemption counter itself is a CPU-local variable, so we need a way
to modify CPU-local variables, or at least a `u8` CPU-local, without disabling
preemption.

As a reminder, we disable preemption when accessing CPU-local variables so that
the task isn't migrated to a different CPU while we are performing operations.

Both x86-64 and AArch64 guarantee that interrupts {only happen on
instruction boundaries} [[1], [2]](TODO: Provide links). (TODO: Due to x86-64
being designed for CISC, there exist instructions such as `xadd` that allow us
to perform operations in one instruction, but not possible on AArch64)

Instead we reuse {some instructions designed for atomics and thread locals
i.e. ldxr/stxr}.

```rust
fn fetch_add(&self, mut operand: u8)
    let offset = #offset_expr;

    let ret;
    unsafe {
        asm!(
            "2:",
            // Load value.
            "mrs {tp_1}, tpidr_el1",
            "add {ptr}, {tp_1}, {offset}",
            "ldxrb {value:w}, [{ptr}]"),

            // Make sure task wasn't migrated between mrs and ldxr.
            "mrs {tp_2}, tpidr_el1",
            "cmp {tp_1}, {tp_2}",
            "b.ne 2b",

            // Compute and store value (reuse tp_1 register).
            "add {tp_1}, {value}, {operand}",
            "stxrb {cond:w}, {tp_1:w}, [{ptr}]",

            // Make sure task wasn't migrated between ldxr and stxr.
            "cbnz {cond}, 2b",

            tp_1 = out(reg) ret,
            ptr = out(reg) _,
            offset = in(reg) offset,
            value = out(reg) ret,
            tp_2 = out(reg) _,
            operand = in(reg) operand,
            cond = out(reg) _,

            options(nostack),
        )
    };
    ret
}
```

{similar to thread local with the primary difference being the `cmp` in between the `ldxr` and `stxr`} (TODO: Verify).
The
value of the thread-local pointer is guaranteed to not change for the duration
of a thread, however, the same is not true for the CPU-local pointer as the
thread can be migrated to a different CPU at any time. Hence, as the comment
suggests, we must ensure that the task isn't migrated between loading the
pointer (`mrs`) and loading the value (`ldxr`).

In fact, this approach is used for all integer CPU-local variables as it is
more efficient.

TODO: `elf_cls`

TODO: false sharing link wikipedia
TODO: https://github.com/theseus-os/Theseus/blob/746608a88ea34e1cfd5702e86f200c6190de06b5/kernel/nano_core/linker_higher_half-x86_64.ld#L118-L119
