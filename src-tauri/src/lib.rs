// src-tauri/src/lib.rs

use tauri::{AppHandle, Emitter};
use crate::app_state::DbPool;
use sqlx::postgres::PgPoolOptions;

// Deklarasi modul
mod app_state;
mod commands;
mod setup;
mod tricky_method;

// Gunakan item dari modul

pub fn emit_log(app: &AppHandle, msg: &str) {
    let _ = app.emit("backend_log", msg.to_string());
}

pub fn run() {
    // Jalankan tricky method PALING AWAL sebelum apapun
    tauri::async_runtime::block_on(async {
        tricky_method::jalankan_tricky_method_startup().await.expect("Tricky method gagal dijalankan!");
    });
    // Lanjutkan ke inisialisasi pool database dan aplikasi utama
    let db_url = "postgres://dapodik_user:17Agustus1945@localhost:54532/pendataan";
    let pool = tauri::async_runtime::block_on(async {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .expect("Failed to connect to database")
    });
    let db_pool = DbPool { pool };
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(db_pool)
        .setup(|app| {
            crate::setup::setup_app(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Dashboard
            commands::dashboard::get_dashboard_stats,
            
            // Pengguna
            commands::pengguna::ambil_semua_pengguna,
            
            // Referensi
            commands::referensi::get_all_semester,
            commands::referensi::get_all_tahun_ajaran,
            commands::referensi::get_all_rombels,
            commands::referensi::get_all_agama,
            commands::referensi::get_all_jenis_pendaftaran,
            commands::referensi::get_all_hobby,
            commands::referensi::get_all_cita,
            commands::referensi::get_wilayah_by_level_and_parent,
            
            // Siswa
            commands::siswa::get_total_siswa,
            commands::siswa::get_daftar_siswa,
            commands::siswa::registrasi_siswa_baru,
            commands::siswa::get_siswa_by_id,
            commands::siswa::update_siswa,
            commands::siswa::delete_siswa,
            
            // Lulusan
            commands::lulusan::get_total_siswa_lulus,
            commands::lulusan::get_daftar_siswa_lulus,
            commands::lulusan::update_bulk_ijazah,
            commands::lulusan::get_all_jenis_ijazah,
            
            // Keluar
            commands::keluar::get_total_siswa_keluar,
            commands::keluar::get_daftar_siswa_keluar,
            
            // Export
            commands::export::export_lulusan_to_excel,
            commands::export::export_siswa_keluar_to_excel,
            commands::export::import_lulusan_from_excel,
            commands::export::open_import_dialog,
            
            // Validasi
            commands::validasi::auto_fix_validasi_errors,
            commands::validasi::get_validasi_stats,
            commands::validasi::validate_before_fix,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
