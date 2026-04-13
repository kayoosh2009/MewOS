slint::include_modules!();
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── 1. Logs ────────────────────────────────────────────────────────────────
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

    // ── 2. Create UI ───────────────────────────────────────────────────────────
    let ui = MainWindow::new().map_err(|e| e.to_string())?;

    // ── 3. Phone: call ─────────────────────────────────────────────────────────
    {
        let ui_handle = ui.as_weak();
        ui.on_call_pressed(move |number| {
            // Backspace: remove last character from dial number
            if number == "backspace" {
                if let Some(ui) = ui_handle.upgrade() {
                    let current = ui.get_dial_number().to_string();
                    if !current.is_empty() {
                        let new = current[..current.len() - 1].to_string();
                        ui.set_dial_number(new.into());
                    }
                }
                return;
            }

            if number.is_empty() {
                eprintln!("MewOS: No number entered");
                return;
            }

            println!("MewOS: Calling -> {}", number);

            #[cfg(not(target_os = "espidf"))]
            println!("MewOS [PC]: Call unavailable without GSM module");

            // ESP32: send AT+ATD{number};
        });
    }

    // ── 4. Phone: hang up ──────────────────────────────────────────────────────
    ui.on_hangup_pressed(|| {
        println!("MewOS: Call ended");
        // ESP32: send AT+ATH
    });

    // ── 5. Camera: capture ─────────────────────────────────────────────────────
    ui.on_camera_capture(|| {
        println!("MewOS: Photo captured");
        // ESP32: trigger camera module via SPI/I2C
    });

    // ── 6. Gallery: open photo ─────────────────────────────────────────────────
    ui.on_gallery_open(|index| {
        println!("MewOS: Opening photo #{}", index);
        // Load image from SD card or flash storage
    });

    // ── 7. Music: play track ───────────────────────────────────────────────────
    ui.on_track_play(|index| {
        println!("MewOS: Playing track #{}", index);
        // ESP32: output audio via I2S DAC/amplifier
        // PC: use rodio or symphonia crate
    });

    // ── 8. Music: delete track ─────────────────────────────────────────────────
    ui.on_track_delete(|index| {
        println!("MewOS: Track #{} deleted", index);
        // Remove file from flash or SD card
    });

    // ── 9. Dock: swap slots ────────────────────────────────────────────────────
    ui.on_dock_swap(|slot_a, slot_b| {
        println!("MewOS: Dock swap slot {} <-> slot {}", slot_a, slot_b);
        // UI-side swap is handled directly in Slint
        // This callback is for persisting the order if needed
    });

    // ── 10. Clock timer ────────────────────────────────────────────────────────
    let timer = slint::Timer::default();
    {
        let ui_handle = ui.as_weak();
        timer.start(slint::TimerMode::Repeated, Duration::from_secs(1), move || {
            if let Some(ui) = ui_handle.upgrade() {
                let now = chrono::Local::now();
                ui.set_current_time(now.format("%H:%M").to_string().into());
            }
        });
    }

    println!("MewOS by Kayoosh Studio — ready!");
    ui.run().map_err(|e| e.to_string())?;

    Ok(())
}