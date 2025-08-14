#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    semihosting::println!("hello world");
    embassy_riscv_hal::init();
    //    semihosting::sys::arm_compat::sys_exit(semihosting::sys::arm_compat::ExitReason::ADP_Stopped_ApplicationExit);
    loop {
        semihosting::println!("high");
        Timer::after_millis(1000).await;
        semihosting::println!("low");
        Timer::after_millis(1000).await;
    }
}
