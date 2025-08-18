#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;

static MUTEX: Mutex<CriticalSectionRawMutex, ()> = Mutex::new(());

#[unsafe(export_name = "_mp_hook")]
pub extern "Rust" fn user_mp_hook(hartid: usize) -> bool {
    if hartid == 0 {
        // hart zero is the leader, go ahead and start.
        semihosting::println!("hart 0 go");
        true
    } else {
        // other harts wait for hart 0 to initialize hardware before proceeding.
        semihosting::println!("hart {hartid} parking.");

        loop {}

        //false
    }
}
#[embassy_executor::task]
async fn blinker() {
    loop {
        {
            let m = MUTEX.lock().await;
            semihosting::println!("ping");
            drop(m);
        }
        Timer::after_millis(1000).await;
        {
            let m = MUTEX.lock().await;
            semihosting::println!("pong");
            drop(m);
        }
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn quitter() {
    Timer::after_millis(5000).await;
    {
        let m = MUTEX.lock().await;
        semihosting::sys::arm_compat::sys_exit(semihosting::sys::arm_compat::ExitReason::ADP_Stopped_ApplicationExit);
        drop(m);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    semihosting::println!("hello world");
    embassy_riscv_hal::init();

    spawner.spawn(blinker()).unwrap();
    spawner.spawn(quitter()).unwrap();
}
