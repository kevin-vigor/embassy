#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    semihosting::println!("hello world");
    embassy_riscv_hal::init();
    for _ in 0..5 {
        semihosting::println!("ping");
        Timer::after_millis(1000).await;
        semihosting::println!("pong");
        Timer::after_millis(1000).await;
    }
    semihosting::println!("bye");
    semihosting::sys::arm_compat::sys_exit(semihosting::sys::arm_compat::ExitReason::ADP_Stopped_ApplicationExit);
}
