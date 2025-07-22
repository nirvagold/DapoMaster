use crate::emit_log;
use crate::tricky_method;
use tauri::{App, Manager, WebviewUrl, Emitter};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    emit_log(&app_handle, "[INIT] Tauri setup initiated.");

    // Buat window splashscreen
    if app.get_webview_window("splashscreen").is_none() {
        emit_log(&app_handle, "[INIT] Creating splashscreen window...");
        tauri::WebviewWindowBuilder::new(
            app,
            "splashscreen",
            WebviewUrl::App("splash.html".into()),
        )
        .decorations(false)
        .inner_size(500.0, 300.0)
        .resizable(false)
        .visible(true)
        .build()?;
    }
    // Buat window utama (main), tapi sembunyikan dulu
    if app.get_webview_window("main").is_none() {
        emit_log(&app_handle, "[INIT] Creating main window...");
        tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
            .title("DapoMaster")
            .inner_size(800.0, 600.0)
            .min_inner_size(600.0, 600.0)
            .visible(false)
            .build()?;
    }

    let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
    let main_window = app.get_webview_window("main").unwrap();
    emit_log(&app_handle, "[INIT] Window handles acquired.");

    // Jalankan tricky method di background task
    tauri::async_runtime::spawn(async move {
        emit_log(&app_handle, "[BG_TASK] Background task started.");
        if let Err(e) = tricky_method::jalankan_tricky_method_rust(&app_handle).await {
            emit_log(&app_handle, &format!("[BG_TASK] Error in tricky_method: {}", e));
            // Kirim event ke frontend, frontend handle alert dan exit
            let _ = app_handle.emit("tricky_method_failed", e);
            // Jangan exit di backend, biar frontend yang exit setelah alert
        } else {
            emit_log(&app_handle, "[BG_TASK] Tricky_method executed successfully.");
            emit_log(&app_handle, "[BG_TASK] Background task finished. Transitioning windows...");
            let _ = splashscreen_window.close();
            let _ = main_window.show();
            emit_log(&app_handle, "[INIT] Main window is now visible.");
        }
    });

    Ok(())
} 