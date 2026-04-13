slint::include_modules!();
use std::time::Duration;

struct CalcState {
    current_value: String,
    previous_value: f64,
    operation: String,
    waiting_for_second_number: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Инициализация логов
    #[cfg(not(target_os = "espidf"))]
    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }
        env_logger::init();
    }

    #[cfg(target_os = "espidf")]
    {
        esp_idf_sys::link_patches();
        esp_idf_svc::log::EspLogger::initialize_default();
    }

    // 2. Создание интерфейса
    let ui = MainWindow::new().map_err(|e| e.to_string())?;
    let ui_handle = ui.as_weak();

    // 3. Логика поиска
    ui.on_search_pressed(move |query| {
        if query.is_empty() {
            return;
        }
        
        println!("MewOS: Отправка запроса в Kayoosh Search -> {}", query);
        
        // Открытие браузера работает только на ПК
        #[cfg(not(target_os = "espidf"))]
        {
            let url = format!("https://www.google.com/search?q={}", query);
            if let Err(e) = webbrowser::open(&url) {
                eprintln!("Ошибка открытия браузера: {}", e);
            }
        }
        
        // На ESP32 здесь будет логика сетевого запроса через Wi-Fi
    });

    // 4. Таймер для часов
    let timer = slint::Timer::default();
    let ui_timer_handle = ui.as_weak();
    timer.start(slint::TimerMode::Repeated, Duration::from_secs(1), move || {
        if let Some(ui) = ui_timer_handle.upgrade() {
            let now = chrono::Local::now();
            ui.set_current_time(now.format("%H:%M").to_string().into());
        }
    });

    println!("MewOS от Kayoosh Studio готова к работе!");
    ui.run().map_err(|e| e.to_string())?;

    Ok(())
}