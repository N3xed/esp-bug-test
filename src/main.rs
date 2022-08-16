use std::time::Duration;

use embedded_svc::executor::asynch::{Executor, WaitableExecutor};
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::asynch::{OnceTimer, TimerService};
use embedded_svc::utils::asyncify::timer::AsyncTimerService;
use embedded_svc::utils::asyncify::Asyncify;
use esp_idf_svc::executor::asynch::isr::tasks_spawner;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::EspISRTimerService;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let mut timers: AsyncTimerService<EspISRTimerService, _> =
        unsafe { EspISRTimerService::new().unwrap() }.into_async();

    let mut timer = timers.timer().unwrap();
    let systime = EspSystemTime;

    let (mut executor, tasks) = tasks_spawner::<2, _>()
        .spawn(async move {
            loop {
                let time = systime.now().as_secs_f32();
                println!("{time}");
                timer.after(Duration::from_millis(16)).unwrap().await;
            }
        })
        .unwrap()
        .release();

    executor.with_context(|exec, ctx| {
        exec.run(ctx, || false, Some(tasks));
    });
}
