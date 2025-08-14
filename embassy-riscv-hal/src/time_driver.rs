use core::task::Waker;

struct RiscvTimeDriver{}


impl embassy_time_driver::Driver for RiscvTimeDriver {
    fn now(&self) -> u64 {
        let mtime: u64;
        // 0000000002004000-000000000200bfff (prio 0, i/o): riscv.aclint.mtimer
        #[cfg(target_arch = "riscv64")]
        unsafe {
            core::arch::asm!(
                  "ld {1}, ({0})",

                  in(reg) 0x2004000,
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
                  "bne {3}, {1}, 1b",
                
                  in(reg) 0x2004000,
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

    fn schedule_wake(&self, _at: u64, _waker: &Waker) {
        
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: RiscvTimeDriver = RiscvTimeDriver{});
