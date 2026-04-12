slint::include_modules!();
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Логи
    #[cfg(not(target_os = "espidf"))]
    env_logger::init();

    // 2. Интерфейс
    let ui = MainWindow::new().map_err(|e| e.to_string())?;
    let ui_handle = ui.as_weak();

    // 3. Таймер для часов (обновление каждую секунду)
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_secs(1), move || {
        if let Some(ui) = ui_handle.upgrade() {
            let now = chrono::Local::now();
            ui.set_current_time(now.format("%H:%M:%S").to_string().into());
        }
    });

    println!("MewOS готова к работе!");
    ui.run().map_err(|e| e.to_string())?;

    Ok(())
}
