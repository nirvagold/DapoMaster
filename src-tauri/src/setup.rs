use crate::emit_log;
use crate::tricky_method;
use tauri::{App, Manager, WebviewUrl, Emitter};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    emit_log(&app_handle, "[SETUP] Mulai setup_app...");

    // Buat window splashscreen
    if app.get_webview_window("splashscreen").is_none() {
        emit_log(&app_handle, "[SETUP] Membuat window splashscreen...");
        tauri::WebviewWindowBuilder::new(
            app,
            "splashscreen",
            WebviewUrl::App("splash.html".into()),
        )
        .decorations(false)
        .inner_size(500.0, 300.0)
        .resizable(false)
        .visible(true)
        .center()
        .build()?;
        emit_log(&app_handle, "[SETUP] Window splashscreen berhasil dibuat dan ditampilkan.");
    } else {
        emit_log(&app_handle, "[SETUP] Window splashscreen sudah ada.");
    }
    // Buat window utama (main), tapi sembunyikan dulu
    if app.get_webview_window("main").is_none() {
        emit_log(&app_handle, "[SETUP] Membuat window utama (main)...");
        tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
            .title("DapoMaster")
            .inner_size(800.0, 600.0)
            .min_inner_size(750.0, 600.0)
            .visible(false)
            .build()?;
        emit_log(&app_handle, "[SETUP] Window utama (main) berhasil dibuat (masih tersembunyi).");
    } else {
        emit_log(&app_handle, "[SETUP] Window utama (main) sudah ada.");
    }

    let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
    let main_window = app.get_webview_window("main").unwrap();
    emit_log(&app_handle, "[SETUP] Handle window splashscreen dan main sudah diperoleh.");

    // Setelah setup selesai, tampilkan window utama dan tutup splashscreen
    emit_log(&app_handle, "[SETUP] Menampilkan window utama (main) dan menutup splashscreen...");
    let _ = main_window.show();
    let _ = splashscreen_window.close();
    emit_log(&app_handle, "[SETUP] Window utama (main) sekarang sudah tampil, splashscreen ditutup.");

    Ok(())
} 