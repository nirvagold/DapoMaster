use crate::app_state::DbPool;
use tauri::{AppHandle, State};

#[derive(serde::Serialize)]
pub struct DashboardStats {
    pub total_siswa: i64,
    pub total_ptk: i64,
    pub total_rombel: i64,
    pub total_jurusan: i64,
}

#[tauri::command]
pub async fn get_dashboard_stats(app: AppHandle, state: State<'_, DbPool>) -> Result<DashboardStats, String> {
    crate::emit_log(&app, "CMD: get_dashboard_stats - Fetching stats.");

    let total_siswa: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM peserta_didik")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;
    let total_ptk: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ptk")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;
    let total_rombel: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rombongan_belajar")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;
    let total_jurusan: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM jurusan_sp")
        .fetch_one(&state.pool).await.map_err(|e| e.to_string())?;

    let stats = DashboardStats {
        total_siswa: total_siswa.0,
        total_ptk: total_ptk.0,
        total_rombel: total_rombel.0,
        total_jurusan: total_jurusan.0,
    };
    
    crate::emit_log(&app, "CMD: get_dashboard_stats - Successfully fetched all stats.");
    Ok(stats)
} 