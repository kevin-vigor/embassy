#![no_std]
#![no_main]

use embassy_executor::Spawner;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    semihosting::println!("hello world");
    semihosting::sys::arm_compat::sys_exit(semihosting::sys::arm_compat::ExitReason::ADP_Stopped_ApplicationExit);
    loop {}
}
