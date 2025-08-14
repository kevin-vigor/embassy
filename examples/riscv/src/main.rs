#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
//use embassy_riscv_hal::* as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    embassy_riscv_hal::init();
    
    semihosting::println!("hello world");
//    semihosting::sys::arm_compat::sys_exit(semihosting::sys::arm_compat::ExitReason::ADP_Stopped_ApplicationExit);
    loop {
        info!("high");
        Timer::after_millis(1000).await;
        info!("low");
        Timer::after_millis(1000).await;
    }
}
