use core::cell::RefCell;
use core::task::Waker;
use critical_section::{CriticalSection, Mutex};
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use riscv::interrupt::Interrupt;
use riscv::register::mie;

struct RiscvTimeDriver {
    queue: Mutex<RefCell<Queue>>,
}

// NB: RISCV_ACLINT_DEFAULT_TIMEBASE_FREQ = 10_000_000

#[riscv_rt::core_interrupt(Interrupt::MachineTimer)]
fn machine_timer_isr() {
    unsafe {
        mie::clear_mtimer();
    }

    DRIVER.on_interrupt();
}

impl RiscvTimeDriver {
    fn on_interrupt(&self) {
        critical_section::with(|cs| {
            let mut next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
            while !self.set_alarm(&cs, next) {
                next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
            }
        });
    }

    fn set_mtimecmp(val: u64) {
        #[cfg(target_arch = "riscv64")]
        unsafe {
            core::arch::asm!(
                "sd {1}, ({0})",

                in(reg) 0x2004000,
                in(reg) val,
                options(nostack),
            );
        }
        #[cfg(target_arch = "riscv32")]
        unsafe {
            let lo: u32 = val as u32;
            let hi: u32 = (val >> 32) as u32;
            core::arch::asm!(
            "li {0}, -1",
            "sw {0}, 0({1})", // No smaller than old value.
            "sw {2}, 4({1})", // No smaller than new value.
            "sw {3}, 0({1})",  //New value.

                out(reg) _,
                in(reg) 0x2004000,
                in(reg) hi,
                in(reg) lo,
                options(nostack),
            );
        }
    }

    fn set_alarm(&self, _cs: &CriticalSection, at: u64) -> bool {
        if at <= self.now() {
            false
        } else {
            Self::set_mtimecmp(at);
            unsafe {
                mie::set_mtimer();
            }

            true
        }
    }
}

impl embassy_time_driver::Driver for RiscvTimeDriver {
    fn now(&self) -> u64 {
        let mtime: u64;
        // 0000000002004000-000000000200bfff (prio 0, i/o): riscv.aclint.mtimer
        #[cfg(target_arch = "riscv64")]
        unsafe {
            core::arch::asm!(
                "ld {1}, ({0})",

                in(reg) 0x2004000 + 0x7ff8, // RISCV_ACLINT_DEFAULT_MTIME
                out(reg) mtime,
                options(nostack),
            );
        }
        #[cfg(target_arch = "riscv32")]
        unsafe {
            let mut lo: u32;
            let mut hi: u32;

            core::arch::asm!(
                "1:",
                "lw {1}, 0({0})",
                "lw {2}, 4({0})",
                "lw {3}, 0({0})",
                "bltu {3}, {1}, 1b", // low 32 bits rolled over

                in(reg) 0x2004000  + 0x7ff8, // RISCV_ACLINT_DEFAULT_MTIME,
                out(reg) lo,
                out(reg) hi,
                out(reg) _,
                options(nostack),
            );
            mtime = (hi as u64) << 32 | lo as u64;
        }
        mtime
        //core::ptr::read_volatile(0x0000000002004000 as *mut u64)
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(&cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        })
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: RiscvTimeDriver = RiscvTimeDriver{queue: Mutex::new(RefCell::new(Queue::new()))});

pub(crate) fn init() {}
