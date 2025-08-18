use critical_section::RawRestoreState;

struct Spinlock {
    lk: core::cell::UnsafeCell<u32>,
}

unsafe impl Sync for Spinlock {}

impl Spinlock {
    const fn new() -> Self {
        Self {
            lk: core::cell::UnsafeCell::new(0),
        }
    }

    fn get(&self) {
        unsafe {
            core::arch::asm!(
                "1:",
                "lr.w {1}, ({0})",
                "bnez {1}, 1b",

                "addi {1}, {1}, 1",
                "sc.w {1}, {1}, ({0})",
                "bnez {1}, 1b",

                "fence rw, rw",

                in(reg) self.lk.get(), // {0}
                out(reg) _, // {1}

                options(nostack),
            );
        }
    }

    fn put(&self) {
        unsafe {
            *self.lk.get() = 0;
        }
    }
}

static CS_SPINLOCK: Spinlock = Spinlock::new();

struct SpinlockCriticalSection;
critical_section::set_impl!(SpinlockCriticalSection);

unsafe impl critical_section::Impl for SpinlockCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        let mut mstatus: usize;

        core::arch::asm!(
            "csrrci {0}, mstatus, 0x8",
            out(reg) mstatus,

            options(nostack),
        );
        CS_SPINLOCK.get();
        core::mem::transmute::<_, riscv::register::mstatus::Mstatus>(mstatus).mie()
    }

    unsafe fn release(was_active: RawRestoreState) {
        CS_SPINLOCK.put();
        // Only re-enable interrupts if they were enabled before the critical section.
        if was_active {
            core::arch::asm!("csrsi mstatus, 0x8", options(nostack),);
        }
    }
}
