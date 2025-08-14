#![no_std]

use riscv::register::mstatus;

mod time_driver;

pub fn init() {
    time_driver::init();
    unsafe {
        mstatus::set_mie();
    }
}
